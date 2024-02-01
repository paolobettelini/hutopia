use actix::{Actor, ActorFutureExt, Addr, Handler, Running, StreamHandler};
use actix_web::{web, web::Data, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::*;
use actix::AsyncContext;
use actix::{fut, ActorContext, ContextFutureSpawner, WrapFuture};
use chat_plugin_protocol::uuid::Uuid;
use chat_plugin_protocol::protocol::{Parcel, Settings};
use chat_plugin_protocol::message::*;

/// Define HTTP actor
struct WsConn {
    id: Option<Uuid>,
    chat: Addr<Chat>,
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        if let Some(id) = self.id {
            self.chat.do_send(Disconnect { id });
        }
        Running::Stop
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {},
            Ok(ws::Message::Binary(bin)) => self.handle_binary_msg(ctx, &bin),
            _ => (),
        }
    }
}

impl WsConn {
    fn handle_binary_msg(&mut self, ctx: &mut <WsConn as Actor>::Context, bin: &[u8]) {
        let message = {
            let settings = &Settings::default();
            let res = ProtocolMessage::from_raw_bytes(bin, settings);
            res.unwrap()
        };

        let message = if let ProtocolMessage::ServerBound(pckt) = message {
            pckt
        } else {
            return;
        };
        match message {
            ServerBoundPacket::SendMsg(msg) => {
                // Broadcast
                if let Some(id) = self.id {
                    println!("Received msg from: {}", id);
                    self.chat.do_send(ClientActorMessage {
                        id,
                        msg,
                    })
                }
            },
            ServerBoundPacket::Connect(id) => {
                let uuid = !id;
                println!("Received connect from: {}", uuid);
                let addr = ctx.address();

                self.id = Some(uuid);

                // Initialize session
                let addr = ctx.address();
                self.chat
                    // Send connect msg to actor
                    .send(Connect {
                        addr: addr.recipient(),
                        id: uuid,
                    })
                    .into_actor(self)
                    .then(|res, _, ctx| {
                        match res {
                            Ok(_res) => (),
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            },
            ServerBoundPacket::Disconnect => {},
            ServerBoundPacket::QueryMsg => {
                if let Some(id) = self.id {
                    self.chat.do_send(ServeMessages { id })
                }
            },
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        let packet = msg.0;
        let bytes = packet.raw_bytes(&Settings::default()).unwrap();
        ctx.binary(bytes);
    }
}

pub async fn init_connection(
    req: HttpRequest,
    stream: web::Payload,
    chat: Data<Addr<Chat>>,
) -> Result<HttpResponse, Error> {
    use std::str::FromStr;
    let handler = WsConn {
        id: None,
        chat: chat.get_ref().clone(),
    };

    let resp = ws::start(handler, &req, stream);
    resp
}

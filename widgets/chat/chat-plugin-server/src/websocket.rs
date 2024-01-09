use actix::{Actor, ActorFutureExt, Addr, Handler, Running, StreamHandler};
use actix_web::{web, web::Data, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::*;
use actix::AsyncContext;
use actix::{fut, ActorContext, ContextFutureSpawner, WrapFuture};
use uuid::Uuid;

/// Define HTTP actor
struct WsConn {
    id: Uuid,
    chat: Addr<Chat>,
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.chat
            .send(Connect {
                addr: addr.recipient(),
                id: self.id,
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
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.chat.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => self.chat.do_send(ClientActorMessage {
                id: self.id,
                msg: text.to_string(),
            }),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub async fn init_connection(
    req: HttpRequest,
    stream: web::Payload,
    chat: Data<Addr<Chat>>,
) -> Result<HttpResponse, Error> {
    let handler = WsConn {
        id: Uuid::new_v4(),
        chat: chat.get_ref().clone(),
    };

    let resp = ws::start(handler, &req, stream);
    resp
}

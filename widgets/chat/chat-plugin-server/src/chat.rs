use actix::{Actor, StreamHandler, Addr, Recipient, Handler};
use actix_web_actors::ws;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use actix::Context;
use actix::AsyncContext;
use uuid::Uuid;
use crate::*;

type Socket = Recipient<WsMessage>;

#[derive(Debug, PartialEq)]
pub struct Chat {
    sessions: HashMap<Uuid, Socket>,
    // channels: HashMap<Uuid, HashSet<Uuid>>,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl Chat {
    // TODO modify to "broadcast".
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));

            println!("Sending message {message} to {id_to}");
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Chat {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Chat {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {

        }
    }
}

impl Handler<Connect> for Chat {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(
            msg.id,
            msg.addr,
        );

        self.send_message(&format!("your id is {}", msg.id), &msg.id);
    }
}

impl Handler<ClientActorMessage> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self
            .sessions
            .iter()
            .for_each(|client| self.send_message(&msg.msg, client.0));
    }
}
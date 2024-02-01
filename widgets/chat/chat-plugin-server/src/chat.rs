use actix::{Actor, Handler, Recipient};

use actix::Context;
use std::collections::HashMap;

use crate::*;
use chat_plugin_protocol::uuid::Uuid;
use chat_plugin_protocol::message::*;
use chat_plugin_protocol::message::ClientBoundPacket::ServeMsg;
use chat_plugin_protocol::SerializableUuid;

type Socket = Recipient<WsMessage>;

pub struct Chat {
    database: Database,
    sessions: HashMap<Uuid, Socket>,
    // channels: HashMap<Uuid, HashSet<Uuid>>,
}

impl Chat {

    pub fn new(database: Database, sessions: HashMap<Uuid, Socket>) -> Self {
        Self { database, sessions }
    }

    fn send_message(&self, message: &str, id_from: &Uuid, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let msg = message.to_owned();
            let packet = ProtocolMessage::ClientBound(ServeMsg(SerializableUuid(*id_from), msg));
            let _ = socket_recipient.do_send(WsMessage(packet));

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
        if self.sessions.remove(&msg.id).is_some() {}
    }
}

impl Handler<Connect> for Chat {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Inserting session");
        self.sessions.insert(msg.id, msg.addr);

        //self.send_message(&format!("your id is {}", msg.id), &msg.id);
    }
}

impl Handler<ServeMessages> for Chat {
    type Result = ();

    fn handle(&mut self, pckt: ServeMessages, _: &mut Context<Self>) -> Self::Result {
        let id = pckt.id;

        let messages = self.database.get_messages();
        for msg in messages {
            self.send_message(&msg.message_text, &msg.user_id, &id);
        }
    }
}

impl Handler<ClientActorMessage> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {        
        self.sessions
            .iter()
            .for_each(|client| self.send_message(&msg.msg, &msg.id, client.0));

        self.database.insert_message(&msg.id, msg.msg);
    }
}

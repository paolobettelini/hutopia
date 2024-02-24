use actix::{Actor, Handler, Recipient};

use actix::Context;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::*;
use chat_plugin_protocol::uuid::Uuid;
use chat_plugin_protocol::message::*;
use chat_plugin_protocol::message::ClientBoundPacket::ServeMsg;
use chat_plugin_protocol::SerializableUuid;

type Socket = Recipient<WsMessage>;

lazy_static::lazy_static! {
    // This data is shared between the plugin instances
    // throughout all the threads.
    static ref SHARED_DATA: Arc<Mutex<SharedData>> = {
        let config = config::get_config();

        // init db
        let url = match std::env::var(config.plugin.db_connection_env) {
            Ok(v) => v,
            Err(e) => panic!("DB env variable not found")
        };
        let database = Database::new(url);

        let sessions = Default::default();

        Arc::new(Mutex::new(SharedData { database, sessions }))
    };
}

pub struct SharedData {
    sessions: HashMap<Uuid, Socket>,
    database: Database,
}

pub struct Chat {
    shared: Arc<Mutex<SharedData>>,
}

impl Chat {

    pub fn new() -> Self {
        let shared = SHARED_DATA.clone();
        Self { shared }
    }

    fn send_message(
        &self,
        message: &str,
        id_from: &Uuid,
        id_to: &Uuid,
        sessions: &HashMap<Uuid, Socket>,
    ) {
        if let Some(socket_recipient) = sessions.get(id_to) {
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
        if self.shared.lock().unwrap().sessions.remove(&msg.id).is_some() {}
    }
}

impl Handler<Connect> for Chat {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Inserting session");
        self.shared.lock().unwrap().sessions.insert(msg.id, msg.addr);

        //self.send_message(&format!("your id is {}", msg.id), &msg.id);
    }
}

impl Handler<ServeMessages> for Chat {
    type Result = ();

    fn handle(&mut self, pckt: ServeMessages, _: &mut Context<Self>) -> Self::Result {
        let id = pckt.id;

        let shared = self.shared.lock().unwrap();
        let messages = shared.database.get_messages();
        for msg in messages {
            self.send_message(&msg.message_text, &msg.user_id, &id, &shared.sessions);
        }
    }
}

impl Handler<ClientActorMessage> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {        
        let shared = self.shared.lock().unwrap();
        
        shared.sessions
            .iter()
            .for_each(|client| self.send_message(&msg.msg, &msg.id, client.0, &shared.sessions));

        shared.database.insert_message(&msg.id, msg.msg);
    }
}

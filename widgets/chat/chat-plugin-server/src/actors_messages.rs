use actix::{Message, Recipient};
use chat_plugin_protocol::uuid::Uuid;
use chat_plugin_protocol::message::ProtocolMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub ProtocolMessage); // Message to send to client

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect { // Client connect request
    pub addr: Recipient<WsMessage>,
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")] // Client message sent message
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
}

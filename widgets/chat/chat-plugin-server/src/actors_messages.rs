use actix::{Message, Recipient};
use chat_plugin_protocol::message::ProtocolMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub ProtocolMessage); // Message to send to client

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect { // Client connect request
    pub addr: Recipient<WsMessage>,
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "()")] // Client message sent message
pub struct ClientActorMessage {
    pub username: String,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")] // Serve messages message
pub struct ServeMessages {
    pub username: String,
}
extern crate protocol;
use crate::{uuid::Uuid, SerializableUuid};

#[derive(Protocol, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
pub enum ProtocolMessage {
    ServerBound(ServerBoundPacket),
    ClientBound(ClientBoundPacket),
}

#[derive(Protocol, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
pub enum ServerBoundPacket {
    SendMsg(String),
    Connect(SerializableUuid),
    Disconnect, // TODO ...
    QueryMsg, // TODO date range and channel

}

#[derive(Protocol, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
pub enum ClientBoundPacket {
    ServeMsg(SerializableUuid, String),
}
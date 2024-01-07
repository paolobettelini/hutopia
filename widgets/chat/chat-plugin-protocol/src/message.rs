extern crate protocol;

use std::net::IpAddr;
use uuid::Uuid;

#[derive(Protocol, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
pub enum ProtocolMessage {
    Disconnect,
    Connect,
    Msg(String),
}
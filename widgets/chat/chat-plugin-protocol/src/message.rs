extern crate protocol;

#[derive(Protocol, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
pub enum ProtocolMessage {
    Disconnect,
    Connect,
    Msg(String),
}
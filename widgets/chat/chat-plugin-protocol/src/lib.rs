#[macro_use]
extern crate protocol_derive;

pub use uuid;
pub use hutopia_utils::uuid_protocol::SerializableUuid;
pub use protocol;
pub mod message;

// TODO - usare protocol features = "derive" ?
// al posto di importare protocol-derive
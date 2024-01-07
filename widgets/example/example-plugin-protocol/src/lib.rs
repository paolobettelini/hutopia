#[macro_use]
extern crate protocol_derive;

mod message;

pub use protocol::*;
pub use message::*;

// TODO - usare protocol features = "derive" ?
// al posto di importare protocol-derive
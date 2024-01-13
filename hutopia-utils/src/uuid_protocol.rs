use protocol::{hint, Parcel, Error, Settings};
use std::io::prelude::*;

use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct SerializableUuid(pub Uuid);

impl Parcel for SerializableUuid
{
    const TYPE_NAME: &'static str = "Uuid";

    fn read_field(read: &mut dyn Read,
                  settings: &Settings,
                  _: &mut hint::Hints)
        -> Result<Self, Error> {
        let bytes: [u8; 16] = Parcel::read(read, settings)?;

        Ok(SerializableUuid(Uuid::from_bytes(bytes)))
    }

    fn write_field(&self,
                   write: &mut dyn Write,
                   _: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        write.write(self.0.as_bytes())?;
        Ok(())
    }
}

impl std::ops::Not for SerializableUuid {
    type Output = Uuid;

    // Required method
    fn not(self) -> Self::Output {
        self.0
    }
}
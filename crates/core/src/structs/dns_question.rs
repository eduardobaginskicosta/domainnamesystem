use super::BytePacketBuffer as Buffer;
use crate::enums::{BytePacketError as Error, QueryType};

// * >>> *

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DnsQuestion {
  pub qtype: QueryType,
  pub name: String,
}

impl DnsQuestion {
  pub fn new(qtype: QueryType, name: String) -> Self {
    Self { qtype, name }
  }

  // *

  pub fn read(&mut self, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.read_qname(&mut self.name)?;
    self.qtype = QueryType::from(buffer.read_u16()?);
    buffer.read_u16()?;
    Ok(())
  }

  pub fn write(&mut self, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.write_qname(&self.name)?;
    buffer.write_u16(self.qtype.to_u16())?;
    buffer.write_u16(0x01)
  }
}

use crate::{
  enums::{BytePacketError as Error, QueryType},
  structs::BytePacketBuffer as Buffer,
};
use std::net::Ipv4Addr;

// * >>> *

pub fn write_a_record(
  buffer: &mut Buffer,
  domain: &String,
  addr: &Ipv4Addr,
  ttl: &u32,
) -> Result<(), Error> {
  buffer.write_qname(domain)?;

  buffer.write_u16(QueryType::A.to_u16())?;
  buffer.write_u16(0x01)?;
  buffer.write_u32(*ttl)?;
  buffer.write_u16(0x04)?;

  for octet in &addr.octets() {
    buffer.write(*octet)?;
  }
  Ok(())
}

use crate::{
  enums::{BytePacketError as Error, QueryType},
  structs::BytePacketBuffer as Buffer,
};
use std::net::Ipv6Addr;

// * >>> *

pub fn write_aaaa_record(
  buffer: &mut Buffer,
  domain: &String,
  addr: &Ipv6Addr,
  ttl: &u32,
) -> Result<(), Error> {
  buffer.write_qname(domain)?;

  buffer.write_u16(QueryType::AAAA.to_u16())?;
  buffer.write_u16(0x01)?;
  buffer.write_u32(*ttl)?;
  buffer.write_u16(0x10)?;

  for segment in addr.segments() {
    buffer.write_u16(segment)?;
  }
  Ok(())
}

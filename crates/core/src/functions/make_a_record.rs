use crate::{
  enums::{BytePacketError as Error, DnsRecord},
  legacy_code,
  structs::BytePacketBuffer as Buffer,
};
use std::net::Ipv4Addr;

// * >>> *

pub fn make_a_record(
  buffer: &mut Buffer,
  domain: String,
  ttl: u32,
) -> Result<DnsRecord, Error> {
  let raw_ip: u32 = buffer.read_u32()?;
  let addr: Ipv4Addr = legacy_code!(
    {
      Ipv4Addr::new(
        (raw_ip >> 0x18) as u8,
        (raw_ip >> 0x10) as u8,
        (raw_ip >> 0x08) as u8,
        (raw_ip & 0xFF) as u8,
      )
    },
    { Ipv4Addr::from(raw_ip) }
  );

  Ok(DnsRecord::A {
    address: addr,
    domain,
    ttl,
  })
}

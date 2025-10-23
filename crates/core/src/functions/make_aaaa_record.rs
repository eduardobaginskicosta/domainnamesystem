use crate::{
  enums::{BytePacketError as Error, DnsRecord},
  structs::BytePacketBuffer as Buffer,
};
use std::net::Ipv6Addr;

// * >>> *

pub fn make_aaaa_record(
  buffer: &mut Buffer,
  domain: String,
  ttl: u32,
) -> Result<DnsRecord, Error> {
  const BYTE_MASK: u32 = 0xFFFF;

  let raw_addr1: u32 = buffer.read_u32()?;
  let raw_addr2: u32 = buffer.read_u32()?;
  let raw_addr3: u32 = buffer.read_u32()?;
  let raw_addr4: u32 = buffer.read_u32()?;

  let addr: Ipv6Addr = Ipv6Addr::new(
    (raw_addr1 >> 0x10) as u16,
    (raw_addr1 & BYTE_MASK) as u16,
    (raw_addr2 >> 0x10) as u16,
    (raw_addr2 & BYTE_MASK) as u16,
    (raw_addr3 >> 0x10) as u16,
    (raw_addr3 & BYTE_MASK) as u16,
    (raw_addr4 >> 0x10) as u16,
    (raw_addr4 & BYTE_MASK) as u16,
  );

  Ok(DnsRecord::AAAA {
    address: addr,
    domain,
    ttl,
  })
}

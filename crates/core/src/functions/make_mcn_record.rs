use crate::{
  enums::{BytePacketError as Error, DnsRecord, QueryType},
  legacy_code,
  structs::BytePacketBuffer as Buffer,
};

// * >>> *

pub fn make_mcn_record(
  qtype: QueryType,
  buffer: &mut Buffer,
  domain: String,
  ttl: u32,
) -> Result<DnsRecord, Error> {
  let mut host: String = String::new();
  buffer.read_qname(&mut host)?;

  match qtype {
    QueryType::CNAME => Ok(DnsRecord::CNAME { domain, host, ttl }),
    QueryType::NS => Ok(DnsRecord::NS { domain, host, ttl }),
    QueryType::MX => Ok(DnsRecord::MX {
      priority: buffer.read_u16()?,
      domain,
      host,
      ttl,
    }),
    _ => legacy_code!({ unreachable!() }, {
      Err(Error::InvalidQueryType(qtype.to_u16()))
    }),
  }
}

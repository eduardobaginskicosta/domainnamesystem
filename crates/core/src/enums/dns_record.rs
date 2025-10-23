use super::{BytePacketError as Error, QueryType};
use crate::{functions::*, legacy_code, structs::BytePacketBuffer as Buffer};
use std::net::{Ipv4Addr, Ipv6Addr};

// * >>> *

#[derive(PartialEq, Eq, Debug, Clone, Hash, PartialOrd, Ord)]
pub enum DnsRecord {
  UNKNOWN {
    data_len: u16,
    domain: String,
    qtype: u16,
    ttl: u32,
  },
  AAAA {
    address: Ipv6Addr,
    domain: String,
    ttl: u32,
  },
  MX {
    priority: u16,
    domain: String,
    host: String,
    ttl: u32,
  },
  CNAME {
    domain: String,
    host: String,
    ttl: u32,
  },
  NS {
    domain: String,
    host: String,
    ttl: u32,
  },
  A {
    address: Ipv4Addr,
    domain: String,
    ttl: u32,
  },
}

impl DnsRecord {
  pub fn read(buffer: &mut Buffer) -> Result<DnsRecord, Error> {
    let mut domain: String = String::new();
    buffer.read_qname(&mut domain)?;

    let qtype_num: u16 = buffer.read_u16()?;
    let qtype: QueryType = QueryType::from(qtype_num);
    buffer.read_u16()?;

    let ttl: u32 = buffer.read_u32()?;
    let data_len: u16 = buffer.read_u16()?;

    match qtype {
      QueryType::UNKNOWN(_) => {
        buffer.step(data_len as usize)?;
        Ok(DnsRecord::UNKNOWN {
          data_len,
          domain,
          qtype: qtype_num,
          ttl,
        })
      },
      QueryType::AAAA => make_aaaa_record(buffer, domain, ttl),
      QueryType::A => make_a_record(buffer, domain, ttl),
      QueryType::MX | QueryType::NS | QueryType::CNAME => {
        make_mcn_record(qtype, buffer, domain, ttl)
      },
    }
  }

  // *

  pub fn write(&self, buffer: &mut Buffer) -> Result<usize, Error> {
    let start_pos: usize = buffer.pos();
    match self {
      Self::AAAA {
        address,
        domain,
        ttl,
      } => write_aaaa_record(buffer, domain, address, ttl)?,
      Self::MX {
        priority,
        domain,
        host,
        ttl,
      } => write_mx_record(buffer, priority, domain, host, ttl)?,
      Self::A {
        address,
        domain,
        ttl,
      } => write_a_record(buffer, domain, address, ttl)?,
      Self::CNAME { domain, host, ttl } | Self::NS { domain, host, ttl } => {
        let qtype: QueryType = legacy_code!(
          {
            match self {
              DnsRecord::CNAME { .. } => QueryType::CNAME,
              DnsRecord::NS { .. } => QueryType::CNAME,
              _ => unreachable!(),
            }
          },
          {
            if let DnsRecord::CNAME { .. } = self {
              QueryType::CNAME
            } else {
              QueryType::NS
            }
          }
        );

        write_cnns_record(qtype, buffer, domain, host, ttl)?;
      },
      Self::UNKNOWN { .. } => legacy_code!({}, { return Err(Error::UnknownRecordError) }),
    }

    Ok(buffer.pos() - start_pos)
  }
}

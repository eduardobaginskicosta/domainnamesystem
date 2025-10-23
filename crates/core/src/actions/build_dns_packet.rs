use crate::{
  enums::{BytePacketError as Error, DnsRecord, QueryType, ResultCode},
  structs::{DnsHeader, DnsPacket, DnsQuestion, LookAtRecord},
};

// * >>> *

pub fn build_dns_packet(record: &LookAtRecord) -> Result<DnsPacket, Error> {
  const DEFAULT_TTL: u32 = 0xE10;
  let mut packet: DnsPacket = DnsPacket::new();

  let num_domains: usize = record.domains.len();
  let has_ipv4: bool = !record.ipv4_addrs.is_empty();
  let has_ipv6: bool = !record.ipv6_addrs.is_empty();

  let mut num_questions: usize = 0x00;
  if has_ipv4 {
    num_questions += num_domains;
  }
  if has_ipv6 {
    num_questions += num_domains;
  }

  let num_answers: usize =
    num_domains * (record.ipv4_addrs.len() + record.ipv6_addrs.len());
  packet.header = DnsHeader {
    id: 0x29A, // random ID
    recursion_desired: true,
    truncated_message: false,
    authoritative_answer: true,
    opcode: 0,
    response: true,
    rescode: ResultCode::NoError,
    checking_disabled: false,
    authed_data: true,
    z: false,
    recursion_available: true,
    questions: num_questions as u16,
    answers: num_answers as u16,
    authoritative_entries: num_domains as u16,
    resource_entries: num_domains as u16,
  };

  for domain in &record.domains {
    if has_ipv4 {
      packet.questions.push(DnsQuestion {
        name: domain.clone(),
        qtype: QueryType::A,
      });
    }

    if has_ipv6 {
      packet.questions.push(DnsQuestion {
        name: domain.clone(),
        qtype: QueryType::AAAA,
      });
    }

    for ipv4 in &record.ipv4_addrs {
      packet.answers.push(DnsRecord::A {
        domain: domain.clone(),
        address: *ipv4,
        ttl: DEFAULT_TTL,
      });
    }

    for ipv6 in &record.ipv6_addrs {
      packet.answers.push(DnsRecord::AAAA {
        domain: domain.clone(),
        address: *ipv6,
        ttl: DEFAULT_TTL,
      });
    }

    packet.authorities.push(DnsRecord::NS {
      domain: domain.clone(),
      host: format!("ns1.{}", domain),
      ttl: DEFAULT_TTL,
    });

    packet.resources.push(DnsRecord::MX {
      domain: domain.clone(),
      priority: 10,
      host: format!("mail.{}", domain),
      ttl: DEFAULT_TTL,
    });
  }

  Ok(packet)
}

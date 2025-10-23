use crate::{
  actions::build_dns_packet,
  enums::ResultCode,
  structs::{DnsPacket, DnsQuestion, DnsServerConfig},
};

// * >>> *

pub fn handle_look_at(
  config: &DnsServerConfig,
  question: &DnsQuestion,
  response: &mut DnsPacket,
  debug: bool,
) -> Option<DnsPacket> {
  let req_domain: String = question.name.to_lowercase();

  for record in &config.look_at {
    let matches_domain: bool = record
      .domains
      .iter()
      .map(|d| d.trim_end_matches('.').to_lowercase())
      .any(|d| req_domain == d || req_domain.ends_with(&format!(".{}", d)));

    if !matches_domain {
      continue;
    }

    // system to block domains
    if !record.ipv4_addrs.iter().any(|ip| ip.octets() != [0; 4])
      && !record.ipv6_addrs.iter().any(|ip| ip.segments() != [0; 8])
    {
      if debug {
        println!(
          "[DEBUG]: Blocked request: {} (Domain Blocking System)",
          question.name
        )
      }

      response.header.rescode = ResultCode::Refused;
      return Some(response.clone());
    }

    if let Ok(mut result) = build_dns_packet(record) {
      result.questions.push(question.clone());
      result.header.id = response.header.id;
      result.header.recursion_desired = response.header.recursion_desired;
      result.header.recursion_available = response.header.recursion_available;
      result.header.response = response.header.response;
      return Some(result);
    }
  }

  None
}

use super::lookup;
use crate::{
  enums::{BytePacketError as Error, QueryType, ResultCode},
  legacy_code,
  structs::DnsPacket,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

// * >>> *

pub fn recursive_lookup(
  socket: &UdpSocket,
  mut dns_servers: Vec<Ipv4Addr>,
  qname: &str,
  qtype: QueryType,
) -> Result<DnsPacket, Error> {
  while let Some(current_ns) = dns_servers.pop() {
    let mut ns_ip: Ipv4Addr = current_ns;

    loop {
      let server: SocketAddr = SocketAddr::new(IpAddr::V4(ns_ip), 53);
      let response: DnsPacket = lookup(socket, qname, qtype, server)?;

      if (!response.answers.is_empty() && response.header.rescode == ResultCode::NoError)
        || response.header.rescode == ResultCode::NxDomain
      {
        return Ok(response);
      }

      if let Some(new_ns_ip) = response.get_resolved_ns(qname) {
        ns_ip = new_ns_ip;
        continue;
      }

      let new_ns_name: &str = match response.get_unresolved_ns(qname) {
        Some(name) => name,
        None => return Ok(response),
      };

      let recursive_response: DnsPacket = recursive_lookup(
        socket,
        vec![
          Ipv4Addr::from([198, 41, 0, 4]), // a.root-servers.net
          Ipv4Addr::from([1, 1, 1, 1]),    // one.one.one.one (cloudflare)
          Ipv4Addr::from([1, 0, 0, 1]),    // one.one.one.one (cloudflare)
        ],
        new_ns_name,
        QueryType::A,
      )?;

      if let Some(new_ip) = recursive_response.get_random_a() {
        ns_ip = new_ip;
      } else {
        return Ok(response);
      }
    }
  }

  Err(legacy_code!(
    { Error::Custom(format!("All DNS servers failed")) },
    { Error::LookupFailed }
  ))
}

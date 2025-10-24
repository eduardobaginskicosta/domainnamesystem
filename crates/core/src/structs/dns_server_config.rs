use super::LookAtRecord;
use std::net::{Ipv4Addr, Ipv6Addr};

// * >>> *

#[derive(Debug, Clone)]
pub struct DnsServerConfig {
  pub nameservers: Vec<Ipv4Addr>,
  pub look_at: Vec<LookAtRecord>,

  #[cfg(feature = "scalability")]
  pub max_messages_count: usize,
  #[cfg(feature = "scalability")]
  pub max_workers_count: usize,
}

impl DnsServerConfig {
  #[cfg(feature = "scalability")]
  pub fn new(
    nameservers: Vec<Ipv4Addr>,
    max_workers: usize,
    max_messages: usize,
  ) -> Self {
    Self {
      nameservers,
      look_at: Vec::new(),
      max_messages_count: if max_messages > 0 { max_messages } else { 1 },
      max_workers_count: if max_workers > 0 { max_workers } else { 1 },
    }
  }

  #[cfg(not(feature = "scalability"))]
  pub fn new(nameservers: Vec<Ipv4Addr>) -> Self {
    Self {
      nameservers,
      look_at: Vec::new(),
    }
  }

  // *

  fn __add_look_at_record(
    &mut self,
    domains: Vec<String>,
    ipv4: Vec<Ipv4Addr>,
    ipv6: Vec<Ipv6Addr>,
  ) {
    if !domains.is_empty() {
      self.look_at.push(LookAtRecord::new(domains, ipv4, ipv6));
    }
  }

  pub fn look_at(&mut self, domain: String, ipv4: Vec<Ipv4Addr>, ipv6: Vec<Ipv6Addr>) {
    self.__add_look_at_record(vec![domain], ipv4, ipv6);
  }

  pub fn look_many(
    &mut self,
    domains: Vec<String>,
    ipv4: Vec<Ipv4Addr>,
    ipv6: Vec<Ipv6Addr>,
  ) {
    self.__add_look_at_record(domains, ipv4, ipv6);
  }
}

use super::LookAtRecord;
use std::net::{Ipv4Addr, Ipv6Addr};

// * >>> *

#[derive(Debug, Clone)]
pub struct DnsServerConfig {
  pub nameservers: Vec<Ipv4Addr>,
  pub look_at: Vec<LookAtRecord>,
}

impl DnsServerConfig {
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

use std::net::{Ipv4Addr, Ipv6Addr};

// * >>> *

#[derive(Debug, Clone)]
pub struct LookAtRecord {
  pub ipv6_addrs: Vec<Ipv6Addr>,
  pub ipv4_addrs: Vec<Ipv4Addr>,
  pub domains: Vec<String>,
}

impl LookAtRecord {
  pub fn new(domains: Vec<String>, ipv4: Vec<Ipv4Addr>, ipv6: Vec<Ipv6Addr>) -> Self {
    Self {
      ipv6_addrs: ipv6,
      ipv4_addrs: ipv4,
      domains,
    }
  }
}

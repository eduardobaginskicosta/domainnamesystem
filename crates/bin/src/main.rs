use dns_core::DnsServer;
use serde::Deserialize;
use std::{
  fs::{metadata, read_to_string},
  io::{Error, ErrorKind},
  net::Ipv4Addr,
};
use toml::de::from_str as toml_parse_str;

// * >>> *

#[derive(Deserialize)]
struct ServerConfig {
  nameservers: Vec<String>,
  max_messages: usize,
  max_workers: usize,
  debug: bool,
}

#[derive(Deserialize)]
struct MultipleDomain {
  ipv6: Vec<String>,
  ipv4: Vec<String>,
  name: Vec<String>,
}

#[derive(Deserialize)]
struct SingleDomain {
  ipv6: Vec<String>,
  ipv4: Vec<String>,
  name: String,
}

#[derive(Deserialize)]
struct DomainConfig {
  multiple: Option<Vec<MultipleDomain>>,
  single: Option<Vec<SingleDomain>>,
}

#[derive(Deserialize)]
struct Config {
  domains: DomainConfig,
  server: ServerConfig,
}

// * >>> *

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Error> {
  if !metadata("server.toml").is_ok() {
    return Err(Error::new(
      ErrorKind::NotFound,
      "Failed to open configuration file 'server.toml'!",
    ));
  }
  let config_string: String = read_to_string("server.toml").unwrap();

  match toml_parse_str::<Config>(&config_string) {
    Ok(config) => {
      let nameservers: Vec<Ipv4Addr> = config
        .server
        .nameservers
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

      // *

      let mut server: DnsServer = DnsServer::new(
        nameservers,
        config.server.max_workers,
        config.server.max_messages,
        config.server.debug,
      )?;

      if let Some(singles) = &config.domains.single {
        for domain in singles {
          server.config.look_at(
            domain.name.clone(),
            domain.ipv4.iter().map(|ip| ip.parse().unwrap()).collect(),
            domain.ipv6.iter().map(|ip| ip.parse().unwrap()).collect(),
          );
        }
      }
      if let Some(multiples) = &config.domains.multiple {
        for domain in multiples {
          server.config.look_many(
            domain.name.clone(),
            domain.ipv4.iter().map(|ip| ip.parse().unwrap()).collect(),
            domain.ipv6.iter().map(|ip| ip.parse().unwrap()).collect(),
          );
        }
      }

      server.start().await
    },
    Err(e) => {
      return Err(Error::new(
        ErrorKind::InvalidData,
        format!("Failed to parse configuration file: {}", e.message()),
      ));
    },
  }
}

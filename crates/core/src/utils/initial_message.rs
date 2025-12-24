use crate::legacy_code;
use std::net::SocketAddr;

// * >>> *

pub(crate) fn initial_message(
  bind_addr: SocketAddr,
  max_workers: usize,
  max_messages: usize,
  debug: bool,
) {
  println!(
    r#"
    Project: Domain Name System (DNS)
    Description: Partial implementation of RFC 1034 and RFC 1035 specifications.
    Author: Eduardo Baginski Costa <eduardobcosta1234@gmail.com>
    License: MIT License
    Repository: https://github.com/eduardobaginskicosta/domainnamesystem
    Buy Me a Coffee (Donate): https://buymeacoffee.com/eduardobaginskicosta

    ( Configure the server by editing 'config.toml' )

    Initializing the '{}' DNS server on the IP: {}
    DEBUG Mode: {}
    SCALE Mode: {} (Max Workers: {} | Max Messages Queue: {})
    "#,
    legacy_code!({ "LEGACY" }, { "EXPERIMENTAL" }),
    bind_addr,
    if debug { "Enabled" } else { "Disbaled" },
    if cfg!(feature = "scalability") {
      "Enabled"
    } else {
      "Disbaled"
    },
    max_workers,
    max_messages
  );
}

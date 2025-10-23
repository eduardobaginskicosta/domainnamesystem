pub mod actions;
pub mod enums;
pub mod functions;
pub mod structs;
pub mod utils;

mod dns_server;

// * >>> *

pub use dns_server::DnsServer;
pub use structs::DnsServerConfig;

// * >>> *

#[macro_export]
macro_rules! legacy_block {
  ($legacy:block, $novo:block) => {{
    // * === LEGACY BLOCK === *
    #[cfg(feature = "legacy")]
    {
      $legacy
    }

    // * === EXPERIMENTAL BLOCK === *
    #[cfg(not(feature = "legacy"))]
    {
      $novo
    }
  }};
}

#[macro_export]
macro_rules! legacy_code {
  ($legacy:block, $experimental:block) => {{
    // * === LEGACY === *
    #[cfg(feature = "legacy")]
    {
      $legacy
    }

    // * === EXPERIMENTAL === *
    #[cfg(not(feature = "legacy"))]
    {
      $experimental
    }
  }};
}

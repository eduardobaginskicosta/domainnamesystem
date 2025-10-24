pub mod actions;
pub mod enums;
pub mod functions;
pub mod macros;
pub mod structs;
pub mod utils;
#[cfg(feature = "scalability")]
pub mod workers;

mod dns_server;

// * >>> *

pub use dns_server::DnsServer;
pub use structs::DnsServerConfig;

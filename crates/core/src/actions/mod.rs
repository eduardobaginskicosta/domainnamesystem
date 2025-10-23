mod build_dns_packet;
mod handle_query;
mod lookup;
mod recursive_lookup;

// * >>> *

pub use build_dns_packet::build_dns_packet;
pub use handle_query::handle_query;
pub use lookup::lookup;
pub use recursive_lookup::recursive_lookup;

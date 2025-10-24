use super::DnsServerConfig;
use std::{
  net::{SocketAddr, UdpSocket},
  sync::Arc,
};

// * >>> *

#[derive(Debug)]
pub struct WorkerTask {
  pub socket: Arc<UdpSocket>,
  pub lookup: Arc<UdpSocket>,
  pub config: Arc<DnsServerConfig>,
  pub data: Vec<u8>,
  pub src: SocketAddr,
  pub debug: bool,
}

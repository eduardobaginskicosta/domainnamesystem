use crate::{scalable_block, structs::DnsServerConfig, utils::initial_message};
use local_ip_address::local_ip;
use std::{
  io::{Error, ErrorKind},
  net::{IpAddr, Ipv4Addr, UdpSocket},
  sync::Arc,
  time::Duration,
};
use tokio::sync::mpsc;

// *

#[cfg(feature = "scalability")]
use crate::{structs::WorkerTask, workers::worker_pool};

#[cfg(not(feature = "scalability"))]
pub use crate::actions::handle_query;
#[cfg(not(feature = "scalability"))]
pub use tokio::spawn;

// * >>> *

#[derive(Debug)]
pub struct DnsServer {
  pub config: DnsServerConfig,
  lookup_client: UdpSocket,
  socket: UdpSocket,
  debug: bool,

  #[cfg(feature = "scalability")]
  worker_tx: mpsc::Sender<WorkerTask>,
}

impl DnsServer {
  #[cfg(feature = "scalability")]
  pub fn new(
    nameservers: Vec<Ipv4Addr>,
    max_workers: usize,
    max_messages: usize,
    debug: bool,
  ) -> Result<Self, Error> {
    let host_ip: IpAddr = local_ip().expect("Failed to detect local IP");
    let default_timeout: Duration = Duration::from_secs(20);

    let lookup_client: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    lookup_client.set_read_timeout(Some(default_timeout))?;
    lookup_client.set_write_timeout(Some(default_timeout))?;

    let server_socket: UdpSocket = UdpSocket::bind((host_ip, 53))?;
    server_socket.set_read_timeout(Some(default_timeout))?;
    server_socket.set_write_timeout(Some(default_timeout))?;
    server_socket.set_nonblocking(true)?;

    let (worker_tx, worker_rx) = mpsc::channel::<WorkerTask>(max_messages);
    tokio::spawn(async move {
      worker_pool(worker_rx, max_workers, max_messages, debug).await;
    });

    Ok(Self {
      lookup_client,
      socket: server_socket,
      config: DnsServerConfig::new(nameservers, max_workers, max_messages),
      debug,
      worker_tx,
    })
  }

  #[cfg(not(feature = "scalability"))]
  pub fn new(nameservers: Vec<Ipv4Addr>, debug: bool) -> Result<Self, Error> {
    let host_ip: IpAddr = local_ip().expect("Failed to detect local IP");
    let default_timeout: Duration = Duration::from_secs(20);

    let lookup_client: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    lookup_client.set_read_timeout(Some(default_timeout))?;
    lookup_client.set_write_timeout(Some(default_timeout))?;

    let server_socket: UdpSocket = UdpSocket::bind((host_ip, 53))?;
    server_socket.set_read_timeout(Some(default_timeout))?;
    server_socket.set_write_timeout(Some(default_timeout))?;
    server_socket.set_nonblocking(true)?;

    Ok(Self {
      lookup_client,
      socket: server_socket,
      config: DnsServerConfig::new(nameservers),
      debug,
    })
  }

  // *

  pub async fn start(self) -> Result<(), Error> {
    let socket: Arc<UdpSocket> = Arc::new(self.socket);
    let lookup_client: Arc<UdpSocket> = Arc::new(self.lookup_client);
    let config: Arc<DnsServerConfig> = Arc::new(self.config);
    initial_message(
      socket.local_addr()?,
      config.max_workers_count,
      config.max_messages_count,
      self.debug,
    );

    let mut buffer: [u8; 1280] = [0u8; 1280];
    loop {
      match socket.recv_from(&mut buffer) {
        Ok((len, src)) => {
          let data: Vec<u8> = buffer[..len].to_vec();
          let socket: Arc<UdpSocket> = Arc::clone(&socket);
          let lookup: Arc<UdpSocket> = Arc::clone(&lookup_client);
          let config: Arc<DnsServerConfig> = Arc::clone(&config);

          scalable_block!(
            {
              if let Err(e) = self
                .worker_tx
                .send(WorkerTask {
                  data,
                  src,
                  socket: socket.clone(),
                  lookup: lookup.clone(),
                  config: config.clone(),
                  debug: self.debug,
                })
                .await
              {
                if self.debug {
                  println!("[DEBUG]: Error sending task to worker pool: {}", e);
                }
              }
            },
            {
              spawn(async move {
                if let Err(e) =
                  handle_query(&config, &lookup, &socket, data, src, self.debug)
                {
                  if self.debug {
                    println!("[DEBUG]: Error handling query from {}: {}", src, e);
                  }
                }
              });
            }
          );
        },
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
          continue; // non-blocking socket: continue pooling
        },
        Err(e) => {
          if self.debug {
            println!("[DEBUG]: Socket error: {}", e);
          }
        },
      }
    }
  }
}

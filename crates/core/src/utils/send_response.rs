use crate::{
  enums::BytePacketError as Error,
  structs::{BytePacketBuffer as Buffer, DnsPacket},
};
use std::net::{SocketAddr, UdpSocket};

// * >>> *

pub fn send_response(
  socket: &UdpSocket,
  response: &mut DnsPacket,
  src: SocketAddr,
) -> Result<(), Error> {
  let mut res_buffer: Buffer = Buffer::new();
  response.write(&mut res_buffer)?;

  let data: &[u8] = res_buffer.get_range(0, res_buffer.pos())?;
  socket.send_to(data, src)?;
  Ok(())
}

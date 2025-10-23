use crate::{
  enums::{BytePacketError as Error, QueryType},
  structs::{BytePacketBuffer as Buffer, DnsPacket, DnsQuestion},
};
use std::net::{SocketAddr, UdpSocket};

// * >>> *

pub fn lookup(
  socket: &UdpSocket,
  qname: &str,
  qtype: QueryType,
  server: SocketAddr,
) -> Result<DnsPacket, Error> {
  let mut packet: DnsPacket = DnsPacket::new();
  packet.header.id = 0x29A; // random ID
  packet.header.questions = 0x01;
  packet.header.recursion_desired = true;
  packet
    .questions
    .push(DnsQuestion::new(qtype, qname.to_string()));

  let mut req_buffer: Buffer = Buffer::new();
  packet.write(&mut req_buffer)?;
  socket.send_to(&req_buffer.buffer[0..req_buffer.position], server)?;

  let mut res_buffer: Buffer = Buffer::new();
  socket.recv_from(&mut res_buffer.buffer)?;

  DnsPacket::from_buffer(&mut res_buffer)
}

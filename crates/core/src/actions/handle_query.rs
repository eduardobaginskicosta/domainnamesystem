use crate::{
  actions::recursive_lookup,
  enums::{BytePacketError as Error, ResultCode},
  structs::{BytePacketBuffer as Buffer, DnsPacket, DnsServerConfig},
  utils::*,
};
use std::net::{SocketAddr, UdpSocket};

// * >>> *

pub fn handle_query(
  config: &DnsServerConfig,
  client_socket: &UdpSocket,
  socket: &UdpSocket,
  buffer: Vec<u8>,
  src: SocketAddr,
  debug: bool,
) -> Result<(), Error> {
  let mut req_buffer: Buffer = Buffer::new();
  req_buffer.buffer[..buffer.len()].copy_from_slice(&buffer);

  let request: DnsPacket = DnsPacket::from_buffer(&mut req_buffer)?;
  let mut response: DnsPacket = DnsPacket::new();
  response.header.id = request.header.id;
  response.header.recursion_desired = true;
  response.header.recursion_available = true;
  response.header.response = true;

  let Some(question) = request.questions.first() else {
    response.header.rescode = ResultCode::FormError;
    return send_response(socket, &mut response, src);
  };

  if let Some(mut result_packet) = handle_look_at(&config, question, &mut response, debug)
  {
    return send_response(socket, &mut result_packet, src);
  }

  match recursive_lookup(
    &client_socket,
    config.nameservers.clone(),
    &question.name,
    question.qtype,
  ) {
    Ok(mut result) => {
      response.questions.push(question.clone());
      response.header.rescode = result.header.rescode;
      response.answers.append(&mut result.answers);
      response.authorities.append(&mut result.authorities);
      response.resources.append(&mut result.resources);
    },
    Err(_) => {
      response.header.rescode = ResultCode::ServerFail;
    },
  }

  send_response(socket, &mut response, src)
}

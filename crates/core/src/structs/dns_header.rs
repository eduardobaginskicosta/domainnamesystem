use super::BytePacketBuffer as Buffer;
use crate::enums::{BytePacketError as Error, ResultCode};
use std::result::Result;

// * >>> *

#[derive(Debug, Clone)]
pub struct DnsHeader {
  pub id: u16,

  pub authoritative_answer: bool,
  pub recursion_desired: bool,
  pub truncated_message: bool,
  pub response: bool,
  pub opcode: u8,

  pub recursion_available: bool,
  pub checking_disabled: bool,
  pub authed_data: bool,
  pub rescode: ResultCode,
  pub z: bool,

  pub authoritative_entries: u16,
  pub resource_entries: u16,
  pub questions: u16,
  pub answers: u16,
}

impl DnsHeader {
  pub fn new() -> Self {
    Self {
      id: 0,
      authoritative_answer: false,
      recursion_desired: false,
      truncated_message: false,
      response: false,
      opcode: 0,
      recursion_available: false,
      checking_disabled: false,
      authed_data: false,
      rescode: ResultCode::NoError,
      z: false,
      authoritative_entries: 0,
      resource_entries: 0,
      questions: 0,
      answers: 0,
    }
  }

  // *

  pub fn read(&mut self, buffer: &mut Buffer) -> Result<(), Error> {
    self.id = buffer.read_u16()?;

    let flags: u16 = buffer.read_u16()?;
    let a: u8 = (flags >> 0x08) as u8;
    let b: u8 = (flags & 0xFF as u16) as u8;

    self.recursion_desired = a & 0x01 != 0;
    self.truncated_message = a & 0x02 != 0;
    self.authoritative_answer = a & 0x04 != 0;
    self.opcode = (a >> 0x03) & 0x0F;
    self.response = a & 0x80 != 0;

    self.rescode = ResultCode::from(b & 0xFF);
    self.checking_disabled = b & 0x10 != 0;
    self.authed_data = b & 0x20 != 0;
    self.z = b & 0x40 != 0;
    self.recursion_available = b & 0x80 != 0;

    self.questions = buffer.read_u16()?;
    self.answers = buffer.read_u16()?;
    self.authoritative_entries = buffer.read_u16()?;
    self.resource_entries = buffer.read_u16()?;

    Ok(())
  }

  // *

  pub fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.write_u16(self.id)?;

    let a: u8 = (self.recursion_desired as u8)
      | ((self.truncated_message as u8) << 0x01)
      | ((self.authoritative_answer as u8) << 0x02)
      | (self.opcode << 0x03)
      | ((self.response as u8) << 0x07);

    let b: u8 = (self.rescode as u8)
      | ((self.checking_disabled as u8) << 0x04)
      | ((self.authed_data as u8) << 0x05)
      | ((self.z as u8) << 0x06)
      | ((self.recursion_available as u8) << 0x07);

    buffer.write(a)?;
    buffer.write(b)?;

    buffer.write_u16(self.questions)?;
    buffer.write_u16(self.answers)?;
    buffer.write_u16(self.authoritative_entries)?;
    buffer.write_u16(self.resource_entries)
  }
}

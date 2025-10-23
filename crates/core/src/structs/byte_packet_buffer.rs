use crate::{enums::BytePacketError as Error, legacy_block, legacy_code};
use std::result::Result;

// * >>> *

pub const PACKET_BUFFER_SIZE: usize = 0x500;
pub struct BytePacketBuffer {
  pub position: usize,
  pub buffer: [u8; PACKET_BUFFER_SIZE],
}

impl BytePacketBuffer {
  pub fn new() -> Self {
    Self {
      position: 0x00,
      buffer: [0x00; PACKET_BUFFER_SIZE],
    }
  }

  // *

  pub fn pos(&self) -> usize {
    self.position
  }

  pub fn step(&mut self, steps: usize) -> Result<(), Error> {
    self.position += steps;
    Ok(())
  }

  pub fn seek(&mut self, pos: usize) -> Result<(), Error> {
    self.position = pos;
    Ok(())
  }

  // *

  pub fn read(&mut self) -> Result<u8, Error> {
    legacy_block!(
      // * === LEGACY === *
      {
        if self.position >= PACKET_BUFFER_SIZE {
          return Err(Error::EndOfBuffer);
        }

        let result: u8 = unsafe { *self.buffer.get_unchecked(self.position) };
        self.position += 0x01;
        Ok(result)
      },
      // * === EXPERIMENTAL === *
      {
        match self.buffer.get(self.position) {
          Some(&byte) => {
            self.position += 0x01;
            Ok(byte)
          },
          None => Err(Error::EndOfBuffer),
        }
      }
    )
  }

  pub fn read_u16(&mut self) -> Result<u16, Error> {
    let high: u16 = legacy_code!({ self.read()? as u16 }, {
      match self.read() {
        Ok(byte) => byte as u16,
        Err(e) => return Err(e),
      }
    });

    let low: u16 = legacy_code!({ self.read()? as u16 }, {
      match self.read() {
        Ok(byte) => byte as u16,
        Err(e) => return Err(e),
      }
    });

    Ok((high << 0x08) | low)
  }

  pub fn read_u32(&mut self) -> Result<u32, Error> {
    let byte1: u32 = legacy_code!({ self.read()? as u32 }, {
      match self.read() {
        Ok(byte) => byte as u32,
        Err(e) => return Err(e),
      }
    });
    let byte2: u32 = legacy_code!({ self.read()? as u32 }, {
      match self.read() {
        Ok(byte) => byte as u32,
        Err(e) => return Err(e),
      }
    });
    let byte3: u32 = legacy_code!({ self.read()? as u32 }, {
      match self.read() {
        Ok(byte) => byte as u32,
        Err(e) => return Err(e),
      }
    });
    let byte4: u32 = legacy_code!({ self.read()? as u32 }, {
      match self.read() {
        Ok(byte) => byte as u32,
        Err(e) => return Err(e),
      }
    });

    Ok((byte1 << 0x18) | (byte2 << 0x10) | (byte3 << 0x08) | byte4)
  }

  // *

  pub fn get(&mut self, pos: usize) -> Result<u8, Error> {
    legacy_code!(
      {
        if pos >= PACKET_BUFFER_SIZE {
          return Err(Error::EndOfBuffer);
        }
        Ok(unsafe { *self.buffer.get_unchecked(pos) })
      },
      {
        self
          .buffer
          .get(pos)
          .map(|&byte| byte)
          .ok_or_else(|| Error::EndOfBuffer)
      }
    )
  }

  pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], Error> {
    legacy_block!(
      {
        if start + len >= PACKET_BUFFER_SIZE {
          return Err(Error::EndOfBuffer);
        }

        Ok(unsafe { &self.buffer.get_unchecked(start..start + len) })
      },
      {
        self
          .buffer
          .get(start..start + len)
          .ok_or(Error::EndOfBuffer)
      }
    )
  }

  // *

  pub fn write(&mut self, val: u8) -> Result<(), Error> {
    legacy_block!(
      {
        if self.position >= PACKET_BUFFER_SIZE {
          return Err(Error::EndOfBuffer);
        }

        unsafe {
          *self.buffer.get_unchecked_mut(self.position) = val;
        }
        self.position += 0x01;
        Ok(())
      },
      {
        self
          .buffer
          .get_mut(self.position)
          .ok_or(Error::EndOfBuffer)
          .map(|byte| {
            *byte = val;
            self.position += 0x01;
          })
      }
    )
  }

  pub fn write_u16(&mut self, val: u16) -> Result<(), Error> {
    self.write((val >> 0x08) as u8)?;
    self.write((val & 0xFF) as u8)
  }

  pub fn write_u32(&mut self, val: u32) -> Result<(), Error> {
    const BYTE_MASK: u32 = 0xFF;

    self.write(((val >> 0x18) & BYTE_MASK) as u8)?;
    self.write(((val >> 0x10) & BYTE_MASK) as u8)?;
    self.write(((val >> 0x08) & BYTE_MASK) as u8)?;
    self.write((val & BYTE_MASK) as u8)
  }

  // *

  pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
    for &byte in bytes {
      legacy_code!(
        {
          self.write(byte)?;
        },
        {
          match self.write(byte) {
            Ok(()) => {},
            Err(e) => return Err(e),
          }
        }
      )
    }
    Ok(())
  }

  // *

  pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), Error> {
    const COMPRESSION_POINTER: u8 = 0xC0;
    let mut position: usize = self.pos();
    let mut jumped: bool = false;

    let mut jumps_performed: i32 = 0x00;
    let mut delim: &'static str = "";
    let max_jumps: i32 = 0x05;

    loop {
      if jumps_performed > max_jumps {
        return Err(Error::MaxJumpsExceeded);
      }

      let length: u8 = legacy_code!({ self.get(position)? }, {
        match self.get(position) {
          Ok(len) => len,
          Err(e) => return Err(e),
        }
      });

      if matches!(length & COMPRESSION_POINTER, COMPRESSION_POINTER) {
        if !jumped {
          self.seek(position + 0x02)?;
        }

        let byte2: u16 = legacy_code!({ self.get(position + 0x01)? as u16 }, {
          match self.get(position + 0x01) {
            Ok(byte) => byte as u16,
            Err(e) => return Err(e),
          }
        });

        let offset: u16 =
          (((length as u16) ^ COMPRESSION_POINTER as u16) << 0x08) | byte2;
        position = offset as usize;

        jumps_performed += 0x01;
        jumped = true;
        continue;
      }

      position += 0x01;
      if length == 0x00 {
        break;
      }

      let label_bytes: &[u8] =
        legacy_code!({ self.get_range(position, length as usize)? }, {
          match self.get_range(position, length as usize) {
            Ok(bytes) => bytes,
            Err(e) => return Err(e),
          }
        });

      if !label_bytes.is_empty() {
        outstr.push_str(delim);
        outstr.push_str(&String::from_utf8_lossy(label_bytes).to_lowercase());
      }

      position += length as usize;
      delim = ".";
    }

    if !jumped {
      self.seek(position)?;
    }
    Ok(())
  }

  pub fn write_qname(&mut self, qname: &str) -> Result<(), Error> {
    const MAX_LABEL_LENGTH: usize = 0x3F;

    for label in qname.split('.') {
      let length: usize = label.len();

      if length > MAX_LABEL_LENGTH {
        return Err(Error::LabelToLoong);
      }

      legacy_block!(
        {
          self.write(length as u8)?;
          self.write_bytes(label.as_bytes())?;
        },
        {
          match self.write(length as u8) {
            Ok(()) => {},
            Err(e) => return Err(e),
          }

          match self.write_bytes(label.as_bytes()) {
            Ok(()) => {},
            Err(e) => return Err(e),
          }
        }
      );
    }

    self.write(0x00)?;
    Ok(())
  }

  // *

  pub fn set(&mut self, pos: usize, val: u8) -> Result<(), Error> {
    legacy_block!(
      {
        unsafe {
          *self.buffer.get_unchecked_mut(pos) = val;
        }
        Ok(())
      },
      {
        if let Some(value) = self.buffer.get_mut(pos) {
          *value = val;
          Ok(())
        } else {
          Err(Error::OutOfBounds)
        }
      }
    )
  }

  pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(), Error> {
    self.set(pos, (val >> 0x08) as u8)?;
    self.set(pos + 0x01, (val & 0xFF) as u8)
  }
}

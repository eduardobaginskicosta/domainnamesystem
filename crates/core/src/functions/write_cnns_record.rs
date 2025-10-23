use crate::{
  enums::{BytePacketError as Error, QueryType},
  structs::BytePacketBuffer as Buffer,
};

// * >>> *

pub fn write_cnns_record(
  qtype: QueryType,
  buffer: &mut Buffer,
  domain: &String,
  host: &String,
  ttl: &u32,
) -> Result<(), Error> {
  buffer.write_qname(domain)?;

  buffer.write_u16(qtype.to_u16())?;
  buffer.write_u16(0x01)?;
  buffer.write_u32(*ttl)?;

  let pos: usize = buffer.pos();
  buffer.write_u16(0x00)?;
  buffer.write_qname(host)?;

  let size: usize = buffer.pos() - (pos + 0x02);
  buffer.set_u16(pos, size as u16)?;
  Ok(())
}

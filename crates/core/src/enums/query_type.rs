#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum QueryType {
  UNKNOWN(u16),
  CNAME,
  AAAA,
  MX,
  NS,
  A,
}

impl QueryType {
  pub fn from(num: u16) -> Self {
    match num {
      0x1C => Self::AAAA,
      0x0F => Self::MX,
      0x05 => Self::CNAME,
      0x02 => Self::NS,
      0x01 => Self::A,
      _ => Self::UNKNOWN(num),
    }
  }

  pub fn to_u16(self) -> u16 {
    match self {
      Self::AAAA => 0x1C,
      Self::MX => 0x0F,
      Self::CNAME => 0x05,
      Self::NS => 0x02,
      Self::A => 0x01,
      Self::UNKNOWN(num) => num,
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ResultCode {
  NoError = 0x00,
  FormError = 0x01,
  ServerFail = 0x02,
  NxDomain = 0x03,
  NoTimp = 0x04,
  Refused = 0x05,
}

impl ResultCode {
  pub fn from(num: u8) -> Self {
    match num {
      0x01 => ResultCode::FormError,
      0x02 => Self::ServerFail,
      0x03 => Self::NxDomain,
      0x04 => Self::NoTimp,
      0x05 => Self::Refused,
      0x00 | _ => ResultCode::NoError,
    }
  }
}

use core::fmt::{Display, Formatter, Result};
use std::{error::Error, io::Error as IoError};

// * >>> *

#[derive(Debug)]
pub enum BytePacketError {
  MaxJumpsExceeded,
  InvalidPosition,
  LabelToLoong,
  EndOfBuffer,

  IoError(IoError),
  Error(Box<dyn Error>),
  Custom(String),

  // * === EXPERIMENTAL === *
  #[cfg(not(feature = "legacy"))]
  InvalidQueryType(u16),
  #[cfg(not(feature = "legacy"))]
  EmptyResponseRecived,
  #[cfg(not(feature = "legacy"))]
  UnknownRecordError,
  #[cfg(not(feature = "legacy"))]
  OutOfBounds,
  #[cfg(not(feature = "legacy"))]
  LookupFailed,
}

// * >>> *

impl Display for BytePacketError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

impl Error for BytePacketError {}

// * >>> *

impl From<IoError> for BytePacketError {
  fn from(value: IoError) -> Self {
    BytePacketError::IoError(value)
  }
}

impl From<Box<dyn Error>> for BytePacketError {
  fn from(value: Box<dyn Error>) -> Self {
    BytePacketError::Error(value)
  }
}

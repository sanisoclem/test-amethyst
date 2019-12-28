use std::io;

#[derive(Debug)]
pub enum AssetEnumerationError {
  Io(io::Error),
}

impl From<io::Error> for AssetEnumerationError {
  fn from(err: io::Error) -> AssetEnumerationError {
    AssetEnumerationError::Io(err)
  }
}

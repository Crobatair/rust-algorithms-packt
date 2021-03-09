
use failure_derive::*;


// Wrapper to wrap IO error and BinCode on single Error
#[derive(Fail, Debug)]
pub enum BlobError {
  #[fail(display = "No Room")]
  NoRoom,
  #[fail(display = "Too Big {}", 0)]
  TooBig(u64),
  #[fail(display = "Not Found")]
  NotFound,
  #[fail(display = "BinCode {}", 0)]
  Bincode(bincode::Error),
  #[fail(display = "IO {}", 0)]
  IO(std::io::Error),
}

// Impl for Bincode
impl From<bincode::Error> for BlobError {
  fn from(e: bincode::Error) -> Self {
    BlobError::Bincode(e)
  }
}

// impl for std::io::Error on BlobError
// basically, convert from e to BlocError::IO
impl From<std::io::Error> for BlobError {
  fn from(e: std::io::Error) -> Self {
    BlobError::IO(e)
  }
}


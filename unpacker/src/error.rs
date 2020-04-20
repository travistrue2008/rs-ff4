use std::io;
use std::result;
use tim2;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoBasePath,
    InvalidNodeKind,
    InvalidFileNum(usize, usize),
    InvalidDecodeLength(usize, usize),
    Io(io::Error),
    Tim2(tim2::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<tim2::Error> for Error {
    fn from(err: tim2::Error) -> Error {
        Error::Tim2(err)
    }
}

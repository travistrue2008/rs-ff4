use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoBasePath,
    InvalidNodeKind,
    InvalidFileNum(usize, usize),
    InvalidDecodeLength(usize, usize),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

use gl_toolkit;
use std::io;
use std::result;
use tim2;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GlToolkit(gl_toolkit::Error),
    InvalidTilesetIndex(usize),
    InvalidTilesetTrigger(u8),
    Io(io::Error),
    Tim2(tim2::Error),
}

impl From<gl_toolkit::Error> for Error {
    fn from(err: gl_toolkit::Error) -> Error {
        Error::GlToolkit(err)
    }
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

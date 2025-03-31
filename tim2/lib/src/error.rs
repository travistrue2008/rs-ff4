use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	InvalidIdentifier(u32),
	InvalidAlignment(u8),
	InvalidBpp(u8),
	InvalidBppFormat(u8),
	InvalidPixelSize(usize),
	Io(io::Error),
	TrueColorAndPaletteFound,
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

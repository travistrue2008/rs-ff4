use std::{fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	InvalidIdentifier(u32),
	InvalidBpp(u8),
	InvalidBppFormat(u8),
	InvalidPixelSize(usize),
	InvalidSwizzleSrcIndex(usize, Box<dyn fmt::Debug>),
	InvalidSwizzleDstIndex(usize, Box<dyn fmt::Debug>),
	Io(io::Error),
	TrueColorAndPaletteFound,
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

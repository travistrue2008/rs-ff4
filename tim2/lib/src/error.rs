use std::{fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	InvalidIdentifier(u32),
	InvalidAlignment(u8),
	InvalidBpp(u8),
	InvalidBppFormat(u8),
	InvalidPixelSize(usize),
	InvalidSwizzleSrcIndex(usize, usize, usize, Box<dyn fmt::Debug>),
	InvalidSwizzleDstIndex(usize, usize, usize, Box<dyn fmt::Debug>),
	MipmapsUnimplemented,
	Io(io::Error),
	TrueColorAndPaletteFound,
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

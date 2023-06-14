use std::io;

#[derive(Debug)]
pub enum Error {
	InvalidIdentifier(u32),
	InvalidBpp(u8),
	InvalidBppFormat(u8),
	InvalidRange(usize),
	Io(io::Error),
	TrueColorAndPaletteFound,
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

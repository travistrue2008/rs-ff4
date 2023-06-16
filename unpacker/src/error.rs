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
	Image(image::ImageError),
	Iso9660(iso9660::ISOError),
	Tim2(tim2::Error),
	Walkdir(walkdir::Error),
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

impl From<image::ImageError> for Error {
	fn from(err: image::ImageError) -> Error {
		Error::Image(err)
	}
}

impl From<iso9660::ISOError> for Error {
	fn from(err: iso9660::ISOError) -> Error {
		Error::Iso9660(err)
	}
}

impl From<tim2::Error> for Error {
	fn from(err: tim2::Error) -> Error {
		Error::Tim2(err)
	}
}

impl From<walkdir::Error> for Error {
	fn from(err: walkdir::Error) -> Error {
		Error::Walkdir(err)
	}
}

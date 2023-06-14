use crate::common::*;
use crate::error::Error;
use crate::frame::Frame;

use byteorder::{ByteOrder, BigEndian, LittleEndian};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const IDENT: u32 = 0x54494d32;

#[derive(Debug)]
struct Header {
	version: u16,
	count: usize,
}

impl Header {
	fn read(buffer: &[u8], offset: &mut usize) -> Result<Header, Error> {
		let mut load_part = |size| { get_slice(&buffer, offset, size) };
		let identifier = BigEndian::read_u32(load_part(4));
		let version = LittleEndian::read_u16(load_part(2));
		let count = LittleEndian::read_u16(load_part(2)) as usize;

		load_part(8);

		if identifier != IDENT {
			return Err(Error::InvalidIdentifier(identifier))
		}

		Ok(Header { version, count })
	}
}

#[derive(Debug)]
pub struct Image {
	header: Header,
	frames: Vec<Frame>,
}

impl Image {
	fn read(buffer: &[u8], offset: &mut usize) -> Result<Image, Error> {
		let header = Header::read(buffer, offset)?;
		let mut frames = Vec::with_capacity(header.count);

		for _ in 0..header.count {
			frames.push(Frame::read(buffer, offset)?);
		}

		Ok(Image { header, frames })
	}

	pub fn version(&self) -> u16 {
		self.header.version
	}

	pub fn frames(&self) -> &Vec<Frame> {
		&self.frames
	}

	pub fn get_frame(&self, index: usize) -> &Frame {
		&self.frames[index]
	}
}

/// Loads a TIM2 image file into memory from buffer.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::prelude::*;
/// 
/// fn main() {
///     let mut buffer = Vec::new();
///     let mut file = File::open("../assets/test.tm2").unwrap();
///     file.read_to_end(&mut buffer).unwrap();
/// 
///     let image = tim2::from_buffer(&buffer).unwrap();
/// 
///     /* print the header info for each frame found */
///     for (i, frame) in image.frames().iter().enumerate() {
///         println!("frame[{}]: <{}  {}>", i, frame.header().width(), frame.header().height());
///     }
/// }
/// ```
pub fn from_buffer(buffer: &[u8]) -> Result<Image, Error> {
	let mut offset = 0usize;

	Image::read(&buffer, &mut offset)
}

/// Loads a TIM2 image file into memory.
///
/// # Examples
///
/// ```
/// fn main() {
///     let image = tim2::load("../assets/test.tm2").unwrap();
/// 
///     /* print the header info for each frame found */
///     for (i, frame) in image.frames().iter().enumerate() {
///         println!("frame[{}]: <{}  {}>", i, frame.header().width(), frame.header().height());
///     }
/// }
/// ```
pub fn load<P: AsRef<Path>>(path: P) -> Result<Image, Error> {
	let mut offset = 0usize;
	let mut buffer = Vec::new();
	let mut file = File::open(path)?;

	file.read_to_end(&mut buffer)?;
	Image::read(&buffer, &mut offset)
}

use crate::error::Error;

use std::fmt;

#[derive(Copy, Clone)]
pub enum Format {
	Indexed,
	Abgr1555,
	Rgb888,
	Rgba8888,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pixel {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

impl Pixel {
	pub fn new() -> Pixel {
		Pixel { r: 255, g: 255, b: 255, a: 255 }
	}

	pub fn from(r: u8, g: u8, b: u8, a: u8) -> Pixel {
		Pixel { r, g, b, a }
	}

	pub fn from_buf(buf: &[u8]) -> Result<Pixel, Error> {
		match buf.len() {
			2 => {
				let raw = ((buf[0] as u16) << 8) | buf[1] as u16;

				Ok(Pixel {
					r: ((raw & 0x001F) as f32 / 31.0 * 255.0) as u8,
					g: (((raw >> 5) & 0x001F) as f32 / 31.0 * 255.0) as u8,
					b: (((raw >> 10) & 0x001F) as f32 / 31.0 * 255.0) as u8,
					a: if raw >> 15 == 1 { 255 } else { 0 },
				})
			},
			3 => Ok(Pixel {
				r: buf[0],
				g: buf[1],
				b: buf[2],
				a: 255,
			}),
			4 => Ok(Pixel {
				r: buf[0],
				g: buf[1],
				b: buf[2],
				a: buf[3],
			}),
			n => Err(Error::InvalidRange(n)),
		}
	}

	pub fn r(&self) -> u8 { self.r }

	pub fn g(&self) -> u8 { self.g }

	pub fn b(&self) -> u8 { self.b }

	pub fn a(&self) -> u8 { self.a }
}

impl Default for Pixel {
	fn default() -> Self { Pixel::new() }
}

impl fmt::Display for Pixel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "px<{}  {}  {}  {}>", self.r(), self.g(), self.b(), self.a())
	}
}

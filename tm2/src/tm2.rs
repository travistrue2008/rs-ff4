use byteorder::{ByteOrder, BigEndian, LittleEndian};
use std::convert::AsMut;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const IDENT: u32 = 0x54494d32;
const SWIZZLE_WIDTH: usize = 16;
const SWIZZLE_HEIGHT: usize = 8;

type ColorKey = Option<[u8;3]>;

pub enum PixelFormat {
	Indexed,
	Abgr5551,
	Rgb888,
	Rgba8888,
}

#[derive(Debug)]
pub enum Error {
	InvalidIdentifier(u32),
	InvalidBpp(u8),
	InvalidBppFormat(u8),
	Io(io::Error),
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

#[derive(Debug)]
struct Header {
	identifier: u32,
	version: u16,
	count: u16,
}

impl Header {
	fn read(buffer: &[u8], offset: &mut usize) -> Result<Header, Error> {
		let mut load_part = |size| { get_slice(&buffer, offset, size) };
		let identifier = BigEndian::read_u32(load_part(4));
		let version = LittleEndian::read_u16(load_part(2));
		let count = LittleEndian::read_u16(load_part(2));

		load_part(8);
		if identifier != IDENT {
			return Err(Error::InvalidIdentifier(identifier))
		}

		Ok(Header { identifier, version, count })
	}
}

#[derive(Debug)]
struct ImageHeader {
	total_size: u32,
    palette_size: u32,
    image_size: u32,
    header_size: u16,
    color_entry_count: u16,
    paletted: u8,
	mipmap_count: u8,
	clut_format: u8,
	bpp: u8,
	width: u16,
	height: u16,
	gs_regs: u32,
	gs_tex_clut: u32,
	gs_tex_0: [u8;8],
	gs_tex_1: [u8;8],
}

impl ImageHeader {
	fn read(buffer: &[u8], offset: &mut usize) -> Result<ImageHeader, Error> {
		let mut load_part = |size| { get_slice(&buffer, offset, size) };
	
		let result = ImageHeader {
			total_size: LittleEndian::read_u32(load_part(4)),
			palette_size: LittleEndian::read_u32(load_part(4)),
			image_size: LittleEndian::read_u32(load_part(4)),
			header_size: LittleEndian::read_u16(load_part(2)),
			color_entry_count: LittleEndian::read_u16(load_part(2)),
			paletted: load_part(1)[0],
			mipmap_count: load_part(1)[0],
			clut_format: load_part(1)[0],
			bpp: ImageHeader::find_bpp(load_part(1)[0])?,
			width: LittleEndian::read_u16(load_part(2)),
			height: LittleEndian::read_u16(load_part(2)),
			gs_tex_0: clone_into_array(load_part(8)),
			gs_tex_1: clone_into_array(load_part(8)),
			gs_regs: LittleEndian::read_u32(load_part(4)),
			gs_tex_clut: LittleEndian::read_u32(load_part(4)),
		};

		let user_data_size = result.header_size as usize - 48;
		if user_data_size > 0 {
			load_part(user_data_size);
		}

		Ok(result)
	}

	fn find_bpp(v: u8) -> Result<u8, Error> {
		match v {
			1 => Ok(16),
			2 => Ok(24),
			3 => Ok(32),
			4 => Ok(4),
			5 => Ok(8),
			n => Err(Error::InvalidBppFormat(n)),
		}
	}

	pub fn is_linear_palette(&self) -> bool {
		self.clut_format & 0x80 != 0
	}

	pub fn pixel_size(&self) -> u8 {
		if self.bpp == 8 {
			self.clut_format & 0x07 + 1
		} else {
			(self.bpp / 8) as u8
		}
	}

	pub fn pixel_format(&self) -> Result<PixelFormat, Error> {
		match self.bpp {
			8 => Ok(PixelFormat::Indexed),
			16 => Ok(PixelFormat::Abgr5551),
			24 => Ok(PixelFormat::Rgb888),
			32 => Ok(PixelFormat::Rgba8888),
			n => Err(Error::InvalidBpp(n)),
		}
	}
}

#[derive(Debug)]
pub struct Image {
	header: ImageHeader,
	pixels: Vec::<u8>,
	palette: Vec::<u8>,
}

impl Image {
	fn read(buffer: &[u8], offset: &mut usize) -> Result<Image, Error> {
		let header = ImageHeader::read(buffer, offset)?;
		let image_size = header.image_size as usize;
		let palette_size = header.palette_size as usize;
		let pixels_slice = get_slice(buffer, offset, image_size);
		let palette_slice = get_slice(buffer, offset, palette_size);
		let pixels = Image::unswizzle(pixels_slice, &header);

		Ok(Image {
			header,
			pixels,
			palette: palette_slice.to_vec(),
		})
	}

	fn unswizzle(buffer: &[u8], header: &ImageHeader) -> Vec::<u8> {
		let mut result = Vec::with_capacity(buffer.len());
		for _ in 0..buffer.len() {
			result.push(0);
		}

		let mut i = 0usize;
		let width = header.width as usize;
		let height = header.height as usize;

		for y in (0..height).step_by(SWIZZLE_HEIGHT) {
			for x in (0..width).step_by(SWIZZLE_WIDTH) {
				for tile_y in y..(y + SWIZZLE_HEIGHT) {
					for tile_x in x..(x + SWIZZLE_WIDTH) {
						if tile_x < width && tile_y < height {
							let index = (tile_y * width + tile_x) as usize;
							if let Some(v) = buffer.get(i) {
								result[index] = *v;
							}
						}

						i += 1;
					}
				}
			}
		}

		result
	}

	pub fn width(&self) -> i32 {
		self.header.width as i32
	}

	pub fn height(&self) -> i32 {
		self.header.height as i32
	}
}

#[derive(Debug)]
pub struct RawImage {
	width: i32,
	height: i32,
	pixels: Vec<u8>,
}

impl RawImage {
	pub fn make(width: i32, height: i32, pixels: Vec<u8>) -> RawImage {
		RawImage { width, height, pixels }
	}

	pub fn width(&self) -> i32 { self.width }

	pub fn height(&self) -> i32 { self.height }

	pub fn pixels(&self) -> &[u8] { &self.pixels }
}

#[derive(Debug)]
pub struct Data {
	header: Header,
	images: Vec<Image>,
}

impl Data {
	fn read(buffer: &[u8], offset: &mut usize) -> Result<Data, Error> {
		let header = Header::read(buffer, offset)?;
		let mut images = Vec::new();

		for _ in 0..header.count {
			images.push(Image::read(buffer, offset)?);
		}

		Ok(Data { header, images })
	}

	pub fn to_raw(&self, index: usize, key: ColorKey) -> RawImage {
		let image = &self.images[index];
		let size = (image.width() * image.height() * 4) as usize;
		let mut pixels = Vec::with_capacity(size);

		for pixel in image.pixels.iter() {
			let start_index = *pixel as usize * 4;
			let end_index = start_index + 4;
			let slice = &image.palette[start_index..end_index];

			if let Some(key) = key {
				let matched =
					slice[0] == key[0] &&
					slice[1] == key[1] &&
					slice[2] == key[2];

				for comp in [
					slice[0],
					slice[1],
					slice[2],
					if matched { 0 } else { slice[3] },
				].iter() {
					pixels.push(*comp);
				}				
			} else {
				for comp in slice.iter() {
					pixels.push(*comp);
				}
			}
		}

		RawImage::make(image.width(), image.height(), pixels)
	}
}

fn get_slice<'a>(buffer: &'a [u8], offset: &mut usize, size: usize) -> &'a [u8] {
	let start_index = *offset;
	let end_index = start_index + size;

	*offset += size;
	&buffer[start_index..end_index]
}

fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
	<A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);

    a
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Data, Error> {
	let mut offset = 0usize;
	let mut buffer = Vec::new();
	let mut file = File::open(path)?;

	file.read_to_end(&mut buffer)?;
	Data::read(&buffer, &mut offset)
}

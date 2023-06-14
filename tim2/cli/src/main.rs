use image::ColorType;
use std::fs;
use std::io;
use std::path::Path;
use std::result;
use tim2::{Image, Pixel};

const COLOR_KEY: Pixel = Pixel::from(0, 255, 0, 255);

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Tim2(tim2::Error),
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

pub fn write_png(img: &Image) -> Result<()> {
	let frame = img.get_frame(0);

	if !frame.has_mipmaps() {
		let width = frame.header().width() as u32;
		let height = frame.header().height() as u32;
		let raw_pixels = frame.to_raw(Some(COLOR_KEY));
		let path = replace_ext(&path, "png")?;

		image::save_buffer(path, &raw_pixels, width, height, ColorType::Rgba8).unwrap();
	}
}

fn process_entry(path: &Path) -> Result<()> {
	let img = tim2::load(path)?;
	let frame_headers = 

	println!("{:?} v{} frames:", path, img.frames().len());

	write_png(&img)?;

	Ok(())
}

fn main() {
	fs::read_dir("../assets")
		.unwrap()
		.filter_map(|entry| entry.ok())
		.for_each(|entry| {
			process_entry(entry.path()).unwrap();
		});
}

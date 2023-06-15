use image::ColorType;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::result;
use tim2::{Header, Image, Pixel};

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

pub fn write_png(path: &Path, img: &Image) -> Result<()> {
	let color_key = Some(Pixel::from(0, 255, 0, 255));
	let frame = img.get_frame(0);

	if !frame.header().has_mipmaps() {
		let width = frame.header().width() as u32;
		let height = frame.header().height() as u32;
		let raw_pixels = frame.to_raw(color_key);
		let mut output_path = PathBuf::from(&path);

		output_path.set_extension("png");
		image::save_buffer(output_path, &raw_pixels, width, height, ColorType::Rgba8).unwrap();
	}

	Ok(())
}

fn process_entry(path: &Path) -> Result<()> {
	let img = tim2::load(path)?;

	let frame_headers: Vec<Header> = img
		.frames()
		.iter()
		.map(|frame| frame.header().clone())
		.collect();

	println!("{:?} v{} frames: {:#?}", path, img.frames().len(), frame_headers);

	write_png(&path, &img)?;

	Ok(())
}

fn main() {
	fs::read_dir("../assets")
		.unwrap()
		.filter_map(|entry| entry.ok())
		.filter(|entry| entry.path().extension().unwrap() == "tm2")
		.for_each(|entry| {
			process_entry(&entry.path()).unwrap();
		});
}

use crate::common::*;
use crate::error::Result;

use image::ColorType;
use std::fs;
use std::path::Path;
use std::str;
use tim2::Pixel;
use walkdir::WalkDir;

static PATH_EXTRACTED: &str = "../iso/extracted";
static PATH_OUTPUT: &str = "../iso/images";

pub fn write_png<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
	let color_key = Pixel::from(0, 255, 0, 255);
	let image_result = std::panic::catch_unwind(|| tim2::from_buffer(&buffer).unwrap());

	match image_result {
		Ok(image) => {
			let frame = image.get_frame(0);
			if !frame.has_mipmaps() {
				let width = frame.header().width() as u32;
				let height = frame.header().height() as u32;
				let raw_pixels = frame.to_raw(Some(color_key));
				let path = replace_ext(&path, "png")?;

				image::save_buffer(path, &raw_pixels, width, height, ColorType::Rgba8).unwrap();
			}
		},
		Err(_) => println!("WARNING: unable to transmute to PNG: {:?}", path.as_ref()),
	}

	Ok(())
}

pub fn process() -> Result<()> {
	println!("Writing PNG files...");

	if Path::new(PATH_OUTPUT).is_dir() {
		fs::remove_dir_all(PATH_OUTPUT).unwrap();
	}

	WalkDir::new(PATH_EXTRACTED)
		.into_iter()
		.filter(|entry| entry.is_ok())
		.map(|entry| entry.unwrap())
		.filter(|entry| entry.metadata().unwrap().is_file())
		.filter(|entry| {
			let path = entry.path();

			let ext = match path.extension() {
				Some(v) => v.to_str().unwrap(),
				None => return false,
			};

			ext == "tm2"
		})
		.for_each(|entry| {
			let input_path = entry.path();
			let output_path = input_path.to_str().unwrap().replace(PATH_EXTRACTED, PATH_OUTPUT);
			let output_path = Path::new(&output_path);

			// println!("INPUT PATH: {:?}", input_path);
			// println!("OUTPUT PATH: {:?}", output_path);

			let buffer = fs::read(&input_path).unwrap();

			if !output_path.parent().unwrap().exists() {
				fs::create_dir_all(output_path.parent().unwrap()).unwrap();
			}

			write_png(&output_path, &buffer).unwrap();
		});

	Ok(())
}

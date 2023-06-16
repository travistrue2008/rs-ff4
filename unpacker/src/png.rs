use crate::error::Result;

use image::ColorType;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use tim2::Pixel;
use walkdir::WalkDir;

static PATH_EXTRACTED: &str = "../iso/extracted";
static PATH_OUTPUT: &str = "../iso/images";
static PATH_ERRORS: &str = "../iso/images/errors";

fn recreate_dir(path: &str) -> Result<()> {
	if Path::new(path).is_dir() {
		fs::remove_dir_all(path)?;
	}

	fs::create_dir_all(path)?;

	Ok(())
}

fn write_png(path: &Path, buffer: &[u8]) -> Result<()> {
	let color_key = Pixel::from(0, 255, 0, 255);
	let img = tim2::from_buffer(&buffer)?;
	let frame = img.get_frame(0);

	if !frame.header().is_paletted() {
		println!("Truecolor found: {:?} bpp: {}", path, frame.header().bpp());
	}

	if !frame.header().has_mipmaps() {
		let width = frame.header().width() as u32;
		let height = frame.header().height() as u32;
		let raw_pixels = frame.to_raw(Some(color_key));
		let mut output_path = PathBuf::from(path);

		output_path.set_extension("png");

		image::save_buffer(&output_path, &raw_pixels, width, height, ColorType::Rgba8)?;
	}

	Ok(())
}

fn has_tm2_extension(entry: &walkdir::DirEntry) -> bool {
	let path = entry.path();

	let ext = match path.extension() {
		Some(v) => v.to_str().unwrap(),
		None => return false,
	};

	ext == "tm2"
}

fn process_entry(entry: &walkdir::DirEntry) {
	let input_path = entry.path();
	let output_path = input_path.to_str().unwrap().replace(PATH_EXTRACTED, PATH_OUTPUT);
	let output_path = Path::new(&output_path);

	let buffer = fs::read(&input_path).unwrap();

	if !output_path.parent().unwrap().exists() {
		fs::create_dir_all(output_path.parent().unwrap()).unwrap();
	}

	match write_png(&output_path, &buffer) {
		Ok(_) => {},
		Err(err) => {
			println!("WARNING: unable write PNG: {:?}\n{:#?}", input_path, err);

			let filename = output_path.file_name().unwrap().to_str().unwrap();
			let path_str = format!("{}/errors/{}", PATH_OUTPUT, filename);
			let path = PathBuf::from(path_str);

			fs::write(path, &buffer).unwrap();
		},
	};
}

pub fn process() -> Result<()> {
	println!("Writing PNG files...");

	recreate_dir(PATH_OUTPUT)?;
	recreate_dir(PATH_ERRORS)?;

	WalkDir::new(PATH_EXTRACTED)
		.into_iter()
		.filter(|entry| entry.is_ok())
		.map(|entry| entry.unwrap())
		.filter(|entry| entry.metadata().unwrap().is_file())
		.filter(|entry| has_tm2_extension(entry))
		.for_each(|entry| process_entry(&entry));

	Ok(())
}

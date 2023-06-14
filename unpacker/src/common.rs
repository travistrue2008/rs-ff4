use crate::error::{Result, Error};

use byteorder::{ByteOrder, LittleEndian};
use image::ColorType;
use std::format;
use std::path::{Path, PathBuf};
use std::str;
use std::fs::File;
use std::io::prelude::*;
use tim2::Pixel;

pub fn read_slice<'a>(buffer: &'a [u8], offset: &mut usize, length: usize) -> &'a [u8] {
    let start_index = *offset as usize;
    let end_index = start_index + length;

    *offset += length;
    &buffer[start_index..end_index]
}

pub fn read_u16(buffer: &[u8], offset: &mut usize) -> u16 {
    let slice = read_slice(buffer, offset, 2);

    LittleEndian::read_u16(slice)
}

pub fn read_u32(buffer: &[u8], offset: &mut usize) -> u32 {
    let slice = read_slice(buffer, offset, 4);

    LittleEndian::read_u32(slice)
}

pub fn read_str(buffer: &[u8], offset: &mut usize, size: usize) -> String {
    let start_index = *offset as usize;
    let end_index = start_index + size;
    let slice = &buffer[start_index..end_index];
    let mut copy_buffer = vec![0u8; size];

    for (i, value) in slice.iter().enumerate() {
        copy_buffer[i] = if *value >= 32 && *value < 127 {
            *value
        } else {
            32
        };
    }

    let result = str::from_utf8(&copy_buffer).unwrap().trim();
    *offset += size;

    String::from(String::from(result).trim())
}

pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
	<A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);

    a
}

pub fn get_base_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let buf = path.as_ref();

    if let Some(filename) = buf.file_name().and_then(|s| s.to_str()) {
        let raw = buf.to_str().unwrap();
        let base = raw.replace(filename, "");

        Ok(PathBuf::from(base))
    } else {
        Err(Error::NoBasePath)
    }
}

pub fn remove_ext<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let base = get_base_path(path.as_ref())?;
    let raw_stem = path.as_ref().file_stem().unwrap().to_str().unwrap();
    let stem = Path::new(raw_stem);
    let result = base.join(stem);

    Ok(result)
}

pub fn replace_ext<P: AsRef<Path>>(path: P, ext: &str) -> Result<PathBuf> {
    let ext_buf = remove_ext(path)?;
    let base = ext_buf.to_str().unwrap();
    let raw = format!("{}.{}", base, ext);
    let result = PathBuf::from(&raw);

    Ok(result)
}

pub fn write_png<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
	let color_key = Pixel::from(0, 255, 0, 255);
	let image_result = std::panic::catch_unwind(|| tim2::from_buffer(&buffer).unwrap());

	match image_result {
		Ok(image) => {
			let frame = image.get_frame(0);
			if !frame.has_mipmaps() {
				let width = frame.width() as u32;
				let height = frame.height() as u32;
				let raw_pixels = frame.to_raw(Some(color_key));
				let path = replace_ext(&path, "png")?;

				image::save_buffer(path, &raw_pixels, width, height, ColorType::Rgba8).unwrap();
			}
		},
		Err(_) => println!("WARNING: unable to transmute to PNG: {:?}", path.as_ref()),
	}

	Ok(())
}

pub fn write_file<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
	let mut pos = 0usize;
	let mut file = File::create(&path)?;

	while pos < buffer.len() {
		pos += file.write(&buffer[pos..])?;
	}

	Ok(())
}

use crate::common::*;
use crate::error::{Error, Result};
use crate::lzss;

use std::fs;
use std::path::Path;
use std::str;
use walkdir::WalkDir;

static PATH_OUTPUT: &str = "../iso/extracted";

fn has_tm2_header(slice: &[u8]) -> bool {
	if let Ok(header) = str::from_utf8(&slice) {
		header == "TIM2"
	} else {
		false
	}
}

fn get_tm2_header_offset(buffer: &[u8]) -> Option<usize> {
	if buffer.len() >= 16 {
		if has_tm2_header(&buffer[5..9]) {
			return Option::Some(4);
		}
	
		if has_tm2_header(&buffer[9..13]) {
			return Option::Some(8);
		}
	}

	None
}

fn decode_buffer(buffer: &[u8]) -> Result<Vec<u8>> {
	match get_tm2_header_offset(buffer) {
		Some(i) => lzss::decode(&buffer[i..]),
		None => Ok(buffer.to_vec()),
	}
}

#[derive(Debug)]
struct FileEntry {
	offset: usize,
	size: usize,
	name: String,
}

fn read_header(buffer: &[u8], offset: &mut usize) -> Result<Vec<FileEntry>> {
	let mut entries = Vec::new();
	let count = read_u16(buffer, offset) as usize;

	for i in 0..count {
		*offset += if i > 0 { 4 } else { 2 };

		let file_offset = read_u32(buffer, offset) as usize;
		let size = read_u32(buffer, offset) as usize;
		let file_num = read_u32(buffer, offset) as usize;
		let name = read_str(buffer, offset, 48);

		if file_num != i {
			return Err(Error::InvalidFileNum(file_num, i));
		}

		entries.push(FileEntry {
			offset: file_offset,
			size,
			name,
		});
	}

	Ok(entries)
}

fn decode_entry(buffer: &[u8], entry: &FileEntry) -> Result<Vec<u8>> {
	let start_offset = entry.offset;
	let end_offset = start_offset + entry.size as usize;
	let slice = &buffer[start_offset..end_offset];

	decode_buffer(slice)
}

fn write_entry<P: AsRef<Path>>(path: P, buffer: &[u8], entry: &FileEntry) -> Result<()> {
	match decode_entry(&buffer, entry) {
		Ok(decoded) => {
			let mut path = path.as_ref().join(&entry.name);

			if decoded.len() > 4 && has_tm2_header(&decoded[0..4]) {
				path = replace_ext(path, "tm2")?;
			}

			write_file(path, &decoded)
		},
		Err(Error::InvalidDecodeLength(offset, len)) => {
			let path_ref = path.as_ref();
			let p = path_ref.to_str().unwrap();

			println!("WARNING: mismatch decoding: {} / {}\n\t{}", offset, len, p);

			Ok(())
		},
		Err(e) => Err(e),
	}
}

pub fn extract_files<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
	let mut offset: usize = 0;

	match read_header(&buffer, &mut offset) {
		Ok(entries) => {
			if entries.len() > 1 {
				let path = remove_ext(path)?;
				fs::create_dir_all(&path)?;
		
				for entry in entries {
					write_entry(&path, &buffer, &entry)?;
				}
			} else {
				let entry = &entries[0];
				let path = get_base_path(path)?;
		
				write_entry(path, &buffer, &entry)?;
			}

			Ok(())
		},
		Err(Error::InvalidFileNum(num, index)) => {
			let path_ref = path.as_ref();
			let p = path_ref.to_str().unwrap();

			println!("WARNING: invalid file num: {} should be: {}\n\t{}", num, index, p);

			Ok(())
		},
		Err(err) => Err(err),
	}
}

fn process_file(path: &Path) -> Result<()> {
	let ext = path.extension().unwrap().to_str().unwrap();
	let buffer = fs::read(path)?;

	match ext {
		"lzs" => {
			let decoded = lzss::decode(&buffer[4..])?;

			extract_files(path, &decoded)?;
		},
		"tm2" => {
			let decoded = decode_buffer(&buffer)?;

			write_file(path, &decoded)?;
		},
		_ => {},
	};

	Ok(())
}

pub fn process() -> Result<()> {
	println!("Decoding files...");

	for entry in WalkDir::new(PATH_OUTPUT) {
		let entry = entry?;

		if entry.metadata()?.is_file() {
			process_file(entry.path())?;
		}
	}

	Ok(())
}

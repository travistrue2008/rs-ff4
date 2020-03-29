extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::path::Path;

struct FileEntry {
	offset: usize,
	size: usize,
	name: String,
}

struct Header {
	count: u16,
	entries: Vec<FileEntry>,
}

fn has_tm2_header(slice: &[u8]) -> bool {
	if let Ok(header) = str::from_utf8(&slice) {
		header == "TIM2"
	} else {
		false
	}
}

fn get_slice<'a>(buffer: &'a [u8], offset: &mut usize, length: usize) -> &'a [u8] {
	let start_index = *offset as usize;
	let end_index = start_index + length;

	*offset += length;
	&buffer[start_index..end_index]
}

fn read_u16(buffer: &[u8], offset: &mut usize) -> u16 {
	let slice = get_slice(buffer, offset, 2);

	LittleEndian::read_u16(slice)
}

fn read_usize(buffer: &[u8], offset: &mut usize) -> usize {
	let slice = get_slice(buffer, offset, 4);

	LittleEndian::read_u32(slice) as usize
}

fn read_str(buffer: &[u8], offset: &mut usize) -> String {
	let start_index = *offset as usize;
	let end_index = start_index + 48;
	let slice = &buffer[start_index..end_index];
	let mut copy_buffer = [0u8; 48];

	for (i, value) in slice.iter().enumerate() {
		copy_buffer[i] = if *value >= 32 && *value < 127 { *value } else { 32 };
	}

	let result = str::from_utf8(&copy_buffer).unwrap().trim();

	*offset += 48;
	String::from(result)
}

fn read_header(buffer: &[u8], offset: &mut usize) -> Header {
	let mut entries = Vec::new();
	let count = read_u16(buffer, offset);

	for i in 0..(count as u32) {
		*offset += if i > 0 { 4 } else { 2 };

		let file_offset = read_usize(buffer, offset);
		let size = read_usize(buffer, offset);
		let file_num = read_usize(buffer, offset);
		let name = read_str(buffer, offset);

		entries.push(FileEntry {
			offset: file_offset,
			size,
			name,
		});

		if file_num != i as usize {
			println!("WARN: invalid file num: {} ({})", file_num, i);
		}
	}

	Header { count, entries }
}

fn decode_lztx(buffer: &[u8]) -> Vec<u8> {
	let mut pos = 0;
	let mut dec_pos = 0;
	let mut control = 0;
	let mut result = Vec::new();

	while pos < buffer.len() - 1 {
		control = buffer[pos];
		pos += 1;

		for i in 0..8 {
			if ((control >> i) & 0x1) == 0 {
				let byte1 = buffer[pos] as usize;
				let byte2 = buffer[pos+1] as usize;
				let length = (byte2 & 0x0F) as i32 + 3;
				let offset = (((byte2 & 0xF0) << 4) | byte1) as i32;
				let mut r = dec_pos - ((dec_pos + 0xFEE - offset) & 0xFFF);
				pos += 2;

				for _ in 0..length {
					if r >= 0 {
						result.push(result[r as usize]);
					} else {
						result.push(0);
					}

					dec_pos += 1;
					r += 1;
				}
			} else {
				result.push(buffer[pos]);
				dec_pos += 1;
				pos += 1;
			}

			if pos >= buffer.len() {
				break;
			}
		}
	}

	result
}

fn decode_file(buffer: &[u8], entry: &FileEntry) -> Vec<u8> {
	let start_offset = entry.offset;
	let end_offset = start_offset + entry.size as usize;
	let slice = &buffer[start_offset..end_offset];

	if has_tm2_header(&slice[5..9]) {
		return decode_lztx(&slice[4..]);
	}

	if has_tm2_header(&slice[9..13]) {
		return decode_lztx(&slice[8..]);
	}
	
	slice.to_vec()
}

fn write_file<P: AsRef<Path>>(path: P, buffer: &[u8]) {
	let mut pos = 0;
	let mut file = File::create(path).unwrap();

	while pos < buffer.len() {
		pos += file.write(&buffer[pos..]).unwrap();
	}
}

fn extract_files(buffer: &[u8], archive_name: &str) {
	let mut offset: usize = 0;
	let header = read_header(&buffer, &mut offset);

	let path = format!("./assets/{}", archive_name);
	fs::create_dir_all(path).unwrap();

	for entry in header.entries {
		let decoded = decode_file(&buffer, &entry);
		let raw_path = format!("./assets/{}/{}", archive_name, entry.name);
		let path = if has_tm2_header(&decoded[0..4]) {
			let stem = Path::new(&entry.name).file_stem().unwrap().to_str().unwrap();

			format!("./assets/{}/{}.tm2", archive_name, &stem)
		} else {
			raw_path
		};

		write_file(path, &decoded);
	}
}

fn main() {
	let mut buffer = Vec::new();
	let mut file = File::open("./assets/00ta_mon.lzs").unwrap();
	
	file.read_to_end(&mut buffer).unwrap();

	if has_tm2_header(&buffer[5..9]) {
		let decoded = decode_lztx(&buffer[4..]);

		write_file("./assets/00ta_mon.tm2", &decoded);
	} else {
		extract_files(&buffer, "00ta_mon");
	}
}

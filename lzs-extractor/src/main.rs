extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;

struct FileEntry {
	offset: usize,
	size: usize,
	name: String,
}

struct Header {
	count: u16,
	entries: Vec<FileEntry>,
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
	let ident =
		(slice[0] as u32) << 24 |
		(slice[1] as u32) << 16 |
		(slice[2] as u32) <<  8 |
		(slice[3] as u32);

	if ident == 0x4c5a5458 {
		decode_lztx(&slice[8..])
	} else {
		slice.to_vec()
	}
}

fn main() {
	let mut offset: usize = 0;
	let mut buffer = Vec::new();
	let mut file = File::open("./assets/menu_gallery_pic.lzs").unwrap();
	
	file.read_to_end(&mut buffer).unwrap();
	fs::create_dir_all("./assets/menu_gallery_pic/").unwrap();

	let header = read_header(&buffer, &mut offset);
	for entry in header.entries {
		let path = format!("./assets/menu_gallery_pic/{}", entry.name);
		let mut file = File::create(path).unwrap();
		let mut pos = 0;
		let decoded = decode_file(&buffer, &entry);

		while pos < decoded.len() {
			pos += file.write(&decoded[pos..]).unwrap();
		}
	}
}

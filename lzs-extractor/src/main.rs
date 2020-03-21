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

fn main() {
	let mut offset: usize = 0;
	let mut buffer = Vec::new();
	let mut file = File::open("./assets/CN_ancient_waterway.lzs").unwrap();
	
	file.read_to_end(&mut buffer);

	let header = read_header(&buffer, &mut offset);
	for (i, entry) in header.entries.iter().enumerate() {
		fs::create_dir_all("./assets/CN_ancient_waterway/");

		let path = format!("./assets/CN_ancient_waterway/{}", entry.name);
		let mut file = File::create(path).unwrap();

		let start_offset = entry.offset;
		let end_offset = start_offset + entry.size as usize;
		let slice = &buffer[start_offset..end_offset];

		file.write(slice);
	}
}

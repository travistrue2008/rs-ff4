use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::str;

struct FileEntry {
	offset: u32,
	size: u32,
	name: String,
}

struct Header {
	count: u16,
	entries: Vec<FileEntry>,
}

fn read_u16(reader: &mut BufReader<&File>) -> u16 {
	let mut buffer = [0; 2];

	reader.read(&mut buffer).unwrap();
	LittleEndian::read_u16(&buffer)
}

fn read_u32(reader: &mut BufReader<&File>) -> u32 {
	let mut buffer = [0; 4];

	reader.read(&mut buffer).unwrap();
	LittleEndian::read_u32(&buffer)
}

fn read_str(reader: &mut BufReader<&File>) -> String {
	let mut buffer = [0; 48];

	reader.read(&mut buffer).unwrap();

	for i in 0..buffer.len() {
		if buffer[i] == 255 {
			buffer[i] = 0;
		}
	}

	let result = str::from_utf8(&buffer).unwrap();
	String::from(result)
}

fn read_header(reader: &mut BufReader<&File>) -> Header {
	let count = read_u16(reader);
	let mut entries = Vec::new();

	for i in 0..(count as u32) {
		if i > 0 {
			read_u16(reader);
		}

		read_u16(reader);

		let offset = read_u32(reader);
		let size = read_u32(reader);
		let file_num = read_u32(reader);
		let name = read_str(reader);

		entries.push(FileEntry { offset, size, name });

		if file_num != i {
			println!("WARN: invalid file num: {} ({})", file_num, i);
		}
	}

	Header { count, entries }
}

fn write_file (reader: &mut BufReader<&File>) {
}

fn main() {
	let mut file = File::open("./assets/CN_ancient_waterway.lzs").unwrap();
	let mut reader = BufReader::new(&file);
	let header = read_header(&mut reader);

	for (i, entry) in header.entries.iter().enumerate() {
		println!("{}: {}", i, entry.name);

		file.seek(SeekFrom::Start(entry.offset as u64)).unwrap();

		let mut buffer = Vec::with_capacity(entry.size as usize);
		let mut reader = BufReader::new(&file);
		let pos = reader.read(&mut buffer).unwrap();
		println!("pos: {}", pos);
	}
}

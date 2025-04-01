use crate::common::*;
use crate::error::{Error, Result};

use iso9660::{DirectoryEntry, ISO9660};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const DIR_ISO_MOVIES: &str = "./PSP_GAME/USRDIR/movie";

fn get_directory_entries(iso: &ISO9660<File>, path: &str) -> Result<Vec<DirectoryEntry<File>>> {
	let src_directory = iso.open(path)?
		.expect(format!("Cannot open directory: {:?}", path).as_str());

	let dir = match src_directory {
		DirectoryEntry::Directory(dir) => dir,
		_ => return Err(Error::NotADirectory(path.to_string())),
	};

	let result: Vec<DirectoryEntry<File>> = dir
		.contents()
		.map(std::result::Result::unwrap)
		.collect();

	Ok(result)
}

fn extract_file(iso: &ISO9660<File>, iso_path: &str, output_path: &str, filename: &str) -> Result<()> {
	let src_path = format!("{}/{}", &iso_path, &filename);

	let src_file = iso.open(&src_path)?
		.expect(format!("Cannot open file: {:?}", src_path).as_str());

	if let DirectoryEntry::File(file) = src_file {
		let mut buffer = Vec::new();
		let mut reader = file.read();

		reader.read_to_end(&mut buffer)?;

		let output_path = PathBuf::from("../iso")
			.join(output_path)
			.join(filename)
			.into_os_string()
			.into_string()
			.unwrap();

		let mut output_file = File::create(output_path)?;

		output_file.write_all(&buffer)?;
	};

	Ok(())
}

fn extract_top_files(iso: &ISO9660<File>) -> Result<()> {
	extract_file(&iso, "./PSP_GAME/SYSDIR", "", "EBOOT.BIN")?;
	extract_file(&iso, "./PSP_GAME/USRDIR", "", "PAC0.BIN")?;
	extract_file(&iso, "./PSP_GAME/USRDIR", "", "PAC1.BIN")?;

	Ok(())
}

fn extract_movies(iso: &ISO9660<File>) -> Result<()> {
	fs::create_dir("../iso/movies")?;

	get_directory_entries(iso, DIR_ISO_MOVIES)?
		.iter()
		.filter(|item| match item {
			DirectoryEntry::File(_file) => true,
			_ => false,
		})
		.for_each(|item| {
			println!("file: {:?}", item.identifier());

			extract_file(
				&iso,
				DIR_ISO_MOVIES,
				"movies",
				item.identifier()
			).unwrap();
		});

	Ok(())
}

pub fn process() -> Result<()> {
	println!("Extracting from ISO...");

	let iso_file = File::open("../ff4.iso").expect("'ff4.iso' not found at root");
	let iso = ISO9660::new(iso_file)?;

	recreate_dir(Path::new("../iso"))?;
	extract_top_files(&iso)?;
	extract_movies(&iso)?;

	Ok(())
}

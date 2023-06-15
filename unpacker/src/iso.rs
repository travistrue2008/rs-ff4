use crate::error::{Result};

use iso9660::{DirectoryEntry, ISO9660};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn recreate_dir() -> Result<()> {
	if Path::new("../iso").is_dir() {
		fs::remove_dir_all("../iso")?;
	}

	fs::create_dir("../iso")?;

	Ok(())
}

fn extract_file(iso: &ISO9660<File>, path: &str, filename: &str) -> Result<()> {
	let src_path = format!("{}/{}", &path, &filename);

	let src_file = iso.open(&src_path)?
		.expect(format!("Cannot open file: {:?}", src_path).as_str());

	if let DirectoryEntry::File(file) = src_file {
		let mut buffer = Vec::new();
		let mut reader = file.read();

		reader.read_to_end(&mut buffer)?;

		let output_path = PathBuf::from("../iso")
			.join(filename)
			.into_os_string()
			.into_string()
			.unwrap();

		let mut output_file = File::create(output_path)?;

		output_file.write_all(&buffer)?;
	};

	Ok(())
}

fn extract_files_from_iso() -> Result<()> {
	let iso_file = File::open("../ff4.iso").expect("'ff4.iso' not found at root");
	let iso = ISO9660::new(iso_file)?;

	extract_file(&iso, "./PSP_GAME/SYSDIR", "EBOOT.BIN")?;
	extract_file(&iso, "./PSP_GAME/USRDIR", "PAC0.BIN")?;
	extract_file(&iso, "./PSP_GAME/USRDIR", "PAC1.BIN")?;

	Ok(())
}

pub fn process() -> Result<()> {
	println!("Extracting from ISO...");

	recreate_dir()?;
	extract_files_from_iso()?;

	Ok(())
}

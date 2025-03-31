use crate::common::*;
use crate::error::Result;
use crate::metadata::{Children, Node, Metadata};

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

static PATH_OUTPUT: &str = "../iso/extracted";

fn process_directory(archive: &mut File, nodes: &Children, path: &Path) -> Result<()> {
	fs::create_dir_all(path)?;

	for node in nodes {
		match node {
			Node::Directory(name, children) => {
				let buf = path.join(name);
				let path = buf.as_path();

				process_directory(archive, children, path)?;
			},
			Node::File(filename, offset, size) => {
				let buffer = path.join(filename);
				let path = buffer.as_path();

				let mut buffer = vec![0u8; *size];
				archive.seek(SeekFrom::Start(*offset as u64))?;
				archive.read_exact(&mut buffer)?;

				write_file(path, &buffer)?
			},
		};
	}

	Ok(())
}

pub fn process() -> Result<()> {
	println!("Extracting from PAC1.BIN...");

	let metadata = Metadata::load()?;
	let nodes = metadata.root()?;

	let mut archive = File::open("../iso/PAC1.BIN")
		.expect("PAC1.BIN not found");

	process_directory(&mut archive, nodes, Path::new(PATH_OUTPUT))?;

	Ok(())
}

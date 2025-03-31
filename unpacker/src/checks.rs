use crate::error::Result;
use crate::metadata::{Children, Node, Metadata};

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fs;
use std::path::Path;

static PATH_OUTPUT: &str = "../iso/extracted";

fn process_directory(nodes: &Children, path: &Path) -> Result<()> {
	for node in nodes {
		match node {
			Node::Directory(name, children) => {
				let pathbuf = path.join(name);
				let path = pathbuf.as_path();

				process_directory(children, path)?;
			},
			Node::File(filename, _offset, _size) => {
				let pathbuf = path.join(filename);
				let full_path = pathbuf.as_path();
				let buffer = fs::read(full_path)?;
				let mut sha256 = Sha256::new();

				sha256.input(&buffer);

				let hash = sha256.result_str();
			},
		};
	}

	Ok(())
}

pub fn process() -> Result<()> {
	println!("Checking extracted files...");

	let metadata = Metadata::load()?;
	let nodes = metadata.root()?;

	process_directory(nodes, Path::new(PATH_OUTPUT))?;

	Ok(())
}

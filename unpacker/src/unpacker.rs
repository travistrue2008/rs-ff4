use crate::common::*;
use crate::error::{Result};
use crate::metadata::{Children, Node, Metadata};

use std::fs;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::Path;

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
                let buf = path.join(filename);
                let path = buf.as_path();

                let mut buf = vec![0u8; *size];
                archive.seek(SeekFrom::Start(*offset as u64))?;
                archive.read_exact(&mut buf)?;

                println!("{:?}: {}", path, buf.len());

                let mut file = File::create(path)?;
                file.write(&buf)?;
            },
        };
    }

    Ok(())
}

pub fn unpack<P: AsRef<Path>>(meta_path: P, archive_path: P, root_dir: P) -> Result<()> {
    let metadata = Metadata::load(meta_path)?;
    let mut archive = File::open(archive_path)?;
    let nodes = metadata.root()?;

    process_directory(&mut archive, nodes, root_dir.as_ref())?;

    Ok(())
}

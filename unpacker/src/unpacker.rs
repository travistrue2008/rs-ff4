use crate::common::*;
use crate::error::{Result, Error};
use crate::lzss;
use crate::metadata::{Children, Node, Metadata};

use std::fs;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::Path;
use std::str;

#[derive(Debug)]
struct FileEntry {
    offset: usize,
    size: usize,
    name: String,
}

fn has_tm2_header(slice: &[u8]) -> bool {
    if let Ok(header) = str::from_utf8(&slice) {
        header == "TIM2"
    } else {
        false
    }
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

        entries.push(FileEntry {
            offset: file_offset,
            size,
            name,
        });

        if file_num != i {
            return Err(Error::InvalidFileNum(file_num, i));
        }
    }

    Ok(entries)
}

fn decode_buffer(buffer: &[u8]) -> Result<Vec<u8>> {
    if buffer.len() >= 16 {
        if has_tm2_header(&buffer[5..9]) {
            return lzss::decode(&buffer[4..]);
        }
    
        if has_tm2_header(&buffer[9..13]) {
            return lzss::decode(&buffer[8..]);
        }
    }

    Ok(buffer.to_vec())
}

fn decode_entry(buffer: &[u8], entry: &FileEntry) -> Result<Vec<u8>> {
    let start_offset = entry.offset;
    let end_offset = start_offset + entry.size as usize;
    let slice = &buffer[start_offset..end_offset];

    decode_buffer(slice)
}

fn write_file<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
    let mut pos = 0;
    let mut file = File::create(path)?;

    while pos < buffer.len() {
        pos += file.write(&buffer[pos..])?;
    }

    Ok(())
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

            println!("WARNING: mismatch decoding: {} / {}\n    {}", offset, len, p);

            Ok(())
        },
        Err(e) => Err(e),
    }
}

fn extract_files<P: AsRef<Path>>(path: P, buffer: &[u8], ) -> Result<()> {
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

            println!("WARNING: invalid file num: {}  should be: {}\n    {}", num, index, p);

            Ok(())
        },
        Err(err) => Err(err),
    }
}

fn process_file(path: &Path, buffer: &[u8], ext: &str) -> Result<()> {
    match ext {
        "lzs" => {
            let decoded_buf = lzss::decode(&buffer)?;

            extract_files(path, &decoded_buf)?;
        },
        "tm2" => {
            let decoded_buf = decode_buffer(&buffer)?;

            write_file(path, &decoded_buf)?;
        },
        _ => {
            let mut file = File::create(path)?;

            file.write(&buffer)?;
        },
    };

    Ok(())
}

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
                let ext = path.extension().unwrap().to_str().unwrap();

                let mut buffer = vec![0u8; *size];
                archive.seek(SeekFrom::Start(*offset as u64))?;
                archive.read_exact(&mut buffer)?;

                process_file(path, &buffer, ext)?;
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

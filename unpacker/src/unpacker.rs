use crate::common::*;
use crate::error::Result;
use crate::lzss;

use image;
use image::ColorType;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use tim2;
use tim2::Pixel;

fn has_tm2_header(slice: &[u8]) -> bool {
    if let Ok(header) = str::from_utf8(&slice) {
        header == "TIM2"
    } else {
        false
    }
}

fn get_tm2_header_offset(buffer: &[u8]) -> Option<usize> {
    if buffer.len() >= 16 {
        if has_tm2_header(&buffer[5..9]) {
            return Option::Some(4);
        }
    
        if has_tm2_header(&buffer[9..13]) {
            return Option::Some(8);
        }
    }

    None
}

fn decode_buffer(buffer: &[u8]) -> Result<Vec<u8>> {
    match get_tm2_header_offset(buffer) {
        Some(i) => lzss::decode(&buffer[i..]),
        None => Ok(buffer.to_vec()),
    }
}

fn write_png<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
    let color_key = Pixel::from(0, 255, 0, 255);
    let image_result = std::panic::catch_unwind(|| tim2::from_buffer(&buffer).unwrap());

    match image_result {
        Ok(image) => {
            let frame = image.get_frame(0);
            if !frame.has_mipmaps() {
                let width = frame.header().width() as u32;
                let height = frame.header().height() as u32;
                let raw_pixels = frame.to_raw(Some(color_key));
                let path = replace_ext(&path, "png")?;

                image::save_buffer(path, &raw_pixels, width, height, ColorType::Rgba8).unwrap();
            }
        },
        Err(_) => println!("WARNING: unable to transmute to PNG: {:?}", path.as_ref()),
    }

    Ok(())
}

fn write_file<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
    let mut pos = 0usize;
    let mut file = File::create(&path)?;
    let path_ref = &path.as_ref();

    while pos < buffer.len() {
        pos += file.write(&buffer[pos..])?;
    }

    if let Some(ext) = path_ref.extension().and_then(OsStr::to_str) {
        if ext == "tm2" {
            write_png(path, buffer)?;
        }
    }

    Ok(())
}

pub mod lzs {
    use super::*;
    use crate::error::{Result, Error};

    use std::fs;
    use std::path::Path;

    #[derive(Debug)]
    struct FileEntry {
        offset: usize,
        size: usize,
        name: String,
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
    
    fn decode_entry(buffer: &[u8], entry: &FileEntry) -> Result<Vec<u8>> {
        let start_offset = entry.offset;
        let end_offset = start_offset + entry.size as usize;
        let slice = &buffer[start_offset..end_offset];
    
        decode_buffer(slice)
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
    
    pub fn process_buffer<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
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

    pub fn process_file<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<()> {
        let mut file = File::open(input_path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;
        process_buffer(&output_path, &buffer)?;

        Ok(())
    }
}

pub mod bin {
    use super::*;
    use crate::error::{Result};
    use crate::lzss;
    use crate::metadata::{Children, Node, Metadata};

    use std::fs;
    use std::fs::File;
    use std::io::SeekFrom;
    use std::path::Path;

    fn process_file(path: &Path, buffer: &[u8], ext: &str, recursive: bool) -> Result<()> {
        match ext {
            "lzs" => {
                let decoded = lzss::decode(&buffer[4..])?;

                if recursive {
                    super::lzs::process_buffer(path, &decoded)?;
                } else {
                    write_file(path, &decoded)?;
                }
            },
            "tm2" => {
                let decoded = decode_buffer(&buffer)?;

                write_file(path, &decoded)?;
            },
            _ => {
                let mut file = File::create(path)?;

                file.write(&buffer)?;
            },
        };

        Ok(())
    }

    fn process_directory(archive: &mut File, nodes: &Children, path: &Path, recursive: bool) -> Result<()> {
        fs::create_dir_all(path)?;

        for node in nodes {
            match node {
                Node::Directory(name, children) => {
                    let buf = path.join(name);
                    let path = buf.as_path();

                    process_directory(archive, children, path, recursive)?;
                },
                Node::File(filename, offset, size) => {
                    let buffer = path.join(filename);
                    let path = buffer.as_path();
                    let ext = path.extension().unwrap().to_str().unwrap();

                    let mut buffer = vec![0u8; *size];
                    archive.seek(SeekFrom::Start(*offset as u64))?;
                    archive.read_exact(&mut buffer)?;

                    process_file(path, &buffer, ext, recursive)?;
                },
            };
        }

        Ok(())
    }

    pub fn process<P: AsRef<Path>>(meta_path: P, archive_path: P, output_dir: P, recursive: bool) -> Result<()> {
        let metadata = Metadata::load(meta_path)?;
        let mut archive = File::open(archive_path).expect("Archive file not found");
        let nodes = metadata.root()?;

        process_directory(&mut archive, nodes, output_dir.as_ref(), recursive)?;

        Ok(())
    }
}

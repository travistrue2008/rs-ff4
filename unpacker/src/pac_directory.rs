use crate::common::*;
use crate::error::{Result};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const CHECKSUM_SIZE: usize = 32;

#[derive(Debug)]
struct Header {
    record_count: usize,
    file_count: usize,
    name_table_size: usize,
    archive_total_size: usize,
}

impl Header {
    fn read(buffer: &[u8], offset: &mut usize) -> Header {
        let header = Header {
            record_count: read_u32(buffer, offset) as usize,
            file_count: read_u32(buffer, offset) as usize,
            name_table_size: read_u32(buffer, offset) as usize,
            archive_total_size: read_u32(buffer, offset) as usize,
        };

        get_slice(buffer, offset, 0x10);
        header
    }
}

#[derive(Debug)]
struct Record {
    id: u32,
    parent_record_id: u32,
    fileinfos_offset: u32,
    number_of_fileinfos: u32,
    directory_info_offset: u32,
}

impl Record {
    fn read(buffer: &[u8], offset: &mut usize) -> Record {
        let id = read_u32(buffer, offset);
        let parent_record_id = read_u32(buffer, offset);
        let fileinfos_offset = read_u32(buffer, offset);
        let number_of_fileinfos = read_u32(buffer, offset);
        get_slice(buffer, offset, 0x4);

        let directory_info_offset = read_u32(buffer, offset);
        get_slice(buffer, offset, 0x8);

        Record {
            id,
            parent_record_id,
            fileinfos_offset,
            number_of_fileinfos,
            directory_info_offset,
        }
    }
}

#[derive(Debug)]
struct FileInfo {
    is_file: u16,
    filename_offset: u32,
    filename_length: u32,
    file_offset: u32,
    file_real_size: u32,
    record_id: u32,
    file_full_size: u32,
    sha_256: [u8; CHECKSUM_SIZE],
}

impl FileInfo {
    fn read(buffer: &[u8], offset: &mut usize) -> FileInfo {
        read_u16(buffer, offset);

        let is_file = read_u16(buffer, offset);
        let filename_offset = read_u32(buffer, offset);
        let filename_length = read_u32(buffer, offset);
        let file_offset = read_u32(buffer, offset);
        let file_real_size = read_u32(buffer, offset);
        read_u32(buffer, offset);

        let record_id = read_u32(buffer, offset);
        let file_full_size = read_u32(buffer, offset);
        let sha_256 = clone_into_array(get_slice(buffer, offset, CHECKSUM_SIZE));

        FileInfo {
            is_file,
            filename_offset,
            filename_length,
            file_offset,
            file_real_size,
            record_id,
            file_full_size,
            sha_256,
        }
    }
}

#[derive(Debug)]
pub struct Directory {
    header: Header,
}

impl Directory {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Directory> {
        let mut offset = 0usize;
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;

        file.read_to_end(&mut buffer)?;
        Directory::read(&buffer, &mut offset)
    }

    fn read(buffer: &[u8], offset: &mut usize) -> Result<Directory> {
        let header = Header::read(&buffer, offset);

        let mut records = Vec::with_capacity(header.record_count);
        for _ in 0..header.record_count {
            records.push(Record::read(buffer, offset));
        }

        let mut files = Vec::with_capacity(header.file_count);
        for _ in 0..header.file_count {
            files.push(FileInfo::read(buffer, offset));
        }

        Ok(Directory {
            header,
        })
    }
}

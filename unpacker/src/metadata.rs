use crate::common::*;
use crate::error::{Result, Error};

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::vec::Vec;

const CHECKSUM_SIZE: usize = 32;

pub type Children = Vec::<Node>;
type Records = Vec::<Record>;
type Infos = Vec::<Info>;
type Names = HashMap::<usize, String>;

#[derive(Debug)]
enum InfoKind {
    Directory,
    File,
}

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
    parent_id: u32,
    info_offset: usize,
    info_count: usize,
    directory_info_offset: usize,
}

impl Record {
    fn read(buffer: &[u8], offset: &mut usize) -> Record {
        let id = read_u32(buffer, offset);
        let parent_id = read_u32(buffer, offset);
        let info_offset = read_u32(buffer, offset) as usize;
        let info_count = read_u32(buffer, offset) as usize;
        get_slice(buffer, offset, 0x4);

        let directory_info_offset = read_u32(buffer, offset) as usize;
        get_slice(buffer, offset, 0x8);

        Record {
            id,
            parent_id,
            info_offset,
            info_count,
            directory_info_offset,
        }
    }
}

#[derive(Debug)]
struct Info {
    record_id: u32,
    kind: InfoKind,
    file_offset: usize,
    file_real_size: usize,
    file_full_size: usize,
    filename_offset: usize,
    filename_length: usize,
    sha_256: [u8; CHECKSUM_SIZE],
}

impl Info {
    fn read(buffer: &[u8], offset: &mut usize) -> Info {
        get_slice(buffer, offset, 0x2);

        let kind = if read_u16(buffer, offset) == 1 {
            InfoKind::File
        } else {
            InfoKind::Directory
        };

        let filename_offset = read_u32(buffer, offset) as usize;
        let filename_length = read_u32(buffer, offset) as usize;
        let file_offset = read_u32(buffer, offset) as usize;
        let file_real_size = read_u32(buffer, offset) as usize;
        get_slice(buffer, offset, 0x4);

        let record_id = read_u32(buffer, offset);
        let file_full_size = read_u32(buffer, offset) as usize;
        let sha_256 = clone_into_array(get_slice(buffer, offset, CHECKSUM_SIZE));

        Info {
            record_id,
            kind,
            file_offset,
            file_real_size,
            file_full_size,
            filename_offset,
            filename_length,
            sha_256,
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Directory(String, Children),
    File(String, usize, usize),
}

#[derive(Debug)]
pub struct Metadata {
    header: Header,
    root: Node,
}

impl Metadata {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Metadata> {
        let mut offset = 0usize;
        let mut buffer = Vec::new();
        let mut file = File::open(path).expect("Index file not found");

        file.read_to_end(&mut buffer)?;
        Metadata::read(&buffer, &mut offset)
    }

    fn read(buffer: &[u8], offset: &mut usize) -> Result<Metadata> {
        let header = Header::read(&buffer, offset);
        let mut record_index = 0usize;

        let mut records = Vec::with_capacity(header.record_count);
        for _ in 0..header.record_count {
            records.push(Record::read(buffer, offset));
        }

        let mut infos = Vec::with_capacity(header.file_count);
        for _ in 0..header.file_count {
            infos.push(Info::read(buffer, offset));
        }

        let names = Metadata::build_filenames(&buffer, offset, header.name_table_size, &infos);

        Ok(Metadata {
            header,
            root: Metadata::build_directory(String::from("data"), &mut record_index, &records, &infos, &names),
        })
    }

    fn build_filenames(buffer: &[u8], offset: &mut usize, size: usize, infos: &Infos) -> Names {
        let end_index = *offset + size;
        let name_buf = &buffer[*offset..end_index];
        let result = infos.iter().map(|info| {
            let start_index = info.filename_offset as usize;
            let end_index = start_index + info.filename_length;
            let slice = &name_buf[start_index..end_index];
            let filename = String::from(str::from_utf8(slice).unwrap().trim());

            (info.filename_offset, filename)
        }).collect();

        *offset += size;

        result
    }

    fn build_directory(name: String, index: &mut usize, records: &Records, infos: &Infos, names: &Names) -> Node {
        let record = &records[*index];
        let mut children = Vec::with_capacity(record.info_count);

        *index += 1;

        for i in 0..record.info_count {
            let info = &infos[record.info_offset + i];
            let name = &names[&info.filename_offset];
            let filename = String::from(name);

            children.push(match info.kind {
                InfoKind::File => Node::File(filename, info.file_offset, info.file_real_size),
                InfoKind::Directory => Metadata::build_directory(filename, index, records, infos, names),
            });
        }

        Node::Directory(name, children)
    }

    pub fn root(&self) -> Result<&Children> {
        match &self.root {
            Node::Directory(_, children) => Ok(children),
            Node::File(_, _, _) => Err(Error::InvalidNodeKind),
        }
    }
}

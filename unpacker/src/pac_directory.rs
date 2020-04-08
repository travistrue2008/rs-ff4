use byteorder::{ByteOrder, LittleEndian};
use std::io::{BufReader, File};

pub struct Header {}

impl Header {
    fn read(reader: &mut BufReader) -> Header {
        Header {}
    }
}

pub struct Directory {
    header: Header,
}

impl Directory {
    pub fn read(file: &File) -> Directory {
        let reader = BufReader::new(file);
        let header = Header::read(&mut reader);

        Directory {
            header,
        }
    }
}

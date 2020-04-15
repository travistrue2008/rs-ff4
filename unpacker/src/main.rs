mod common;
mod error;
mod lzss;
mod metadata;
mod unpacker;

use clap::{Arg, App};
use std::path::{Path, PathBuf};

fn main() {
    let matches = App::new("Final Fantasy IV - Archive Unpacker")
        .version("1.0")
        .author("Travis J True")
        .about("Unpacks files from the .BIN files found in the PSP ISO of FF4")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("Path to PAC0.BIN and PAC1.BIN files")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Output path")
            .takes_value(true))
        .get_matches();

    let input_path = Path::new(matches.value_of("path").unwrap_or("./"));
    let output_path = match matches.value_of("output") {
        Some(v) => PathBuf::from(v),
        None => PathBuf::from(input_path),
    };

    let meta_path = input_path.join("PAC0.BIN");
    let archive_path = input_path.join("PAC1.BIN");

    unpacker::unpack(meta_path, archive_path, output_path).unwrap();
}

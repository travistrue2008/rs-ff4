mod common;
mod error;
mod lzss;
mod metadata;
mod extractor;

use crate::extractor::{bin, lzs};

use clap::{Arg, ArgMatches, App, SubCommand};
use std::path::PathBuf;

fn get_input_path(matches: &ArgMatches) -> PathBuf {
    PathBuf::from(matches.value_of("path").unwrap_or("./"))
}

fn get_output_path(matches: &ArgMatches) -> PathBuf {
    let input_path = get_input_path(matches);

    match matches.value_of("output") {
        Some(v) => PathBuf::from(v),
        None => PathBuf::from(input_path),
    }
}

fn process_bin(matches: &ArgMatches) {
    let recursive = matches.is_present("recursive");
    let input_path = get_input_path(matches);
    let output_path = get_output_path(matches).join("output");
    let meta_path = input_path.join("PAC0.BIN");
    let archive_path = input_path.join("PAC1.BIN");

    bin::process(&meta_path, &archive_path, &output_path, recursive).unwrap();
}

fn process_lzs(matches: &ArgMatches) {
    let input_path = get_input_path(matches);
    let output_path = get_output_path(matches);

    lzs::process_file(&input_path, &output_path).unwrap();
}

fn main() {
    let app = App::new("Final Fantasy IV - Archive unpacker")
        .version("1.0")
        .author("Travis J True")
        .about("Unpacks files from the .BIN files found in the PSP ISO of FF4")
        .subcommand(SubCommand::with_name("bin")
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
            .arg(Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Recursively decompress subsequent lzs files")))
        .subcommand(SubCommand::with_name("lzs")
            .arg(Arg::with_name("path")
                .short("p")
                .long("path")
                .help("Path to .lzs file")
                .takes_value(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output path")
                .takes_value(true)));

    let matches = app.get_matches();

    match matches.subcommand() {
        ("bin", Some(m)) => process_bin(&m),
        ("lzs", Some(m)) => process_lzs(&m),
        _ => println!("{}", matches.usage()),
    };
}

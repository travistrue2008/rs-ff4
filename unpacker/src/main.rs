mod pac_directory;

use pac_directory::Directory;

use bitreader::BitReader;
use std::io::File;

fn main() {
    let file = File::open("assets/PAC0.BIN");
    let directory = Directory::read(file);

    println!("directory: {:?}", directory);
}

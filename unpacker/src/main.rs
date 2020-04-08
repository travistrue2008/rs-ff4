mod common;
mod error;
mod pac_directory;

use pac_directory::Directory;

fn main() {
    let directory = Directory::load("assets/PAC0.BIN").unwrap();

    println!("directory: {:?}", directory);
}

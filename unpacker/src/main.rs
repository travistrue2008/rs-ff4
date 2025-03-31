mod common;
mod decode;
mod error;
mod extract;
mod extract_check;
mod iso;
mod lzss;
mod metadata;
mod png;
mod tilesets;

use crate::error::Result;

fn main() -> Result<()> {
	iso::process()?;
	extract::process()?;
	extract_check::process()?;
	decode::process()?;
	png::process()?;

	Ok(())
}

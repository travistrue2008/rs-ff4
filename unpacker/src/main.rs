mod common;
mod decode;
mod error;
mod extract;
mod iso;
mod lzss;
mod metadata;
mod tilesets;

use crate::error::Result;

fn main() -> Result<()> {
	iso::process()?;
	extract::process()?;
	decode::process()?;

	Ok(())
}

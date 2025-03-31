mod checks;
mod common;
mod decode;
mod error;
mod extract;
mod iso;
mod lzss;
mod metadata;
mod png;

use crate::error::Result;

fn main() -> Result<()> {
	iso::process()?;
	extract::process()?;
	checks::process()?;
	decode::process()?;
	png::process()?;

	Ok(())
}

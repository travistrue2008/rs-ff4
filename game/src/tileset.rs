use crate::common::*;
use crate::error::Result;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
}

#[derive(Debug)]
pub struct Tileset {
    pub width: usize,
    pub height: usize,
    pub cells: Vec::<Cell>,
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Tileset> {
    let mut offset = 0usize;
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let width = read_u16(&buffer, &mut offset) as usize;
    let height = read_u16(&buffer, &mut offset) as usize;
    let mut cells = Vec::with_capacity(width * height);

    for i in 0..(width * height) {
        let mut temp_offset = 0usize;
        let start_index = i * 6 + offset;
        let end_index = start_index + 6;
        let slice = &buffer[start_index..end_index];

        cells.push(Cell {
            v1: read_u16(&slice, &mut temp_offset) as usize,
            v2: read_u16(&slice, &mut temp_offset) as usize,
            v3: read_u16(&slice, &mut temp_offset) as usize,
        })
    }

    Ok(Tileset {
        width,
        height,
        cells,
    })
}

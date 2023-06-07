use crate::error::Result;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use image;
use image::ColorType;
use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use tim2;
use tim2::Pixel;

const TILE_SIZE: usize = 32;
const SECTION_SIZE: usize = TILE_SIZE * TILE_SIZE * 4;

type SubSection = [u8; SECTION_SIZE];

struct Image {
    width: usize,
    height: usize,
    raw: Vec::<u8>,
}

fn get_sub_section(x_tile: usize, y_tile: usize, width: usize, buffer: &[u8]) -> SubSection {
    let x_start = x_tile * TILE_SIZE;
    let y_start = y_tile * TILE_SIZE;
    let mut result = [0; SECTION_SIZE];

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let local_index = ((y * TILE_SIZE) + x) * 4;
            let image_index = (((y + y_start) * width) + (x + x_start)) * 4;

            result[local_index + 0] = buffer[image_index + 0];
            result[local_index + 1] = buffer[image_index + 1];
            result[local_index + 2] = buffer[image_index + 2];
            result[local_index + 3] = buffer[image_index + 3];
        }
    }

    result
}

fn calc_hash(buffer: &SubSection) -> String {
    let mut hasher = Sha256::new();

    hasher.input(buffer);
    hasher.result_str()
}

fn filter_directories<P: AsRef<Path>>(path: P, items: ReadDir) -> Vec::<PathBuf> {
    let result: Vec::<PathBuf> = items
        .filter_map(|item| item.ok())
        .filter(|item| item.file_type().unwrap().is_dir())
        .map(|item| item.file_name().into_string().unwrap())
        .filter(|item| item.starts_with("CN_"))
        .filter(|item| !item.ends_with("_char"))
        .filter(|item| !item.contains("_d_"))
        .map(|item| path.as_ref().join(item))
        .collect();

    result
}

fn filter_files<P: AsRef<Path>>(path: P, name: &str, items: ReadDir) -> Vec::<PathBuf> {
    let suffix = &format!("_{}.tm2", name);
    let result = items
        .filter_map(|item| item.ok())
        .map(|item| item.file_name().into_string().unwrap())
        .filter(|item| item.ends_with(suffix))
        .filter(|item| !item.starts_with("dtown_"))
        .map(|item| path.as_ref().join(item))
        .collect();

    result
}

fn build_tile_map(name: &str, directories: &Vec::<PathBuf>) -> Result<HashMap<String, SubSection>> {
    let mut tiles = HashMap::new();
    let color_key = Pixel::from(0, 255, 0, 255);

    for dir in directories {
        let items = fs::read_dir(dir)?;
        let filtered = filter_files(&dir, &name, items);

        for file_path in filtered {
            let image = tim2::load(file_path)?;

            for frame in image.frames() {
                let raw = frame.to_raw(Some(color_key));

                for y in 0..(frame.height() / TILE_SIZE) {
                    for x in 0..(frame.width() / TILE_SIZE) {
                        let section_buffer = get_sub_section(x, y, frame.width(), &raw);
                        let hash = calc_hash(&section_buffer);

                        if !tiles.contains_key(&hash) {
                            tiles.insert(hash, section_buffer);
                        }
                    }
                }
            }
        }
    }

    Ok(tiles)
}

fn build_image(tiles: &HashMap<String, SubSection>) -> Image {
    let width = (tiles.len() as f32).sqrt().ceil() as usize;
    let height = (tiles.len() as f32 / width as f32).ceil() as usize;
    let pixel_width = width * TILE_SIZE;
    let mut raw = vec![0; width * height * TILE_SIZE * TILE_SIZE * 4];

    for (i, kvp) in tiles.iter().enumerate() {
        let x_start = (i % width) * TILE_SIZE;
        let y_start = (i / width) * TILE_SIZE;
        let buffer = kvp.1;

        println!("starts<{}  {}>", x_start, y_start);

        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                let local_index = ((y * TILE_SIZE) + x) * 4;
                let image_index = (((y + y_start) * pixel_width) + (x + x_start)) * 4;

                raw[image_index + 0] = buffer[local_index + 0];
                raw[image_index + 1] = buffer[local_index + 1];
                raw[image_index + 2] = buffer[local_index + 2];
                raw[image_index + 3] = buffer[local_index + 3];
            }
        }
    }

    Image {
        width: width * TILE_SIZE,
        height: height * TILE_SIZE,
        raw,
    }
}

pub fn process<P: AsRef<Path>>(input_path: P, output_path: P, name: &str) -> Result<()> {
    let output_filename = format!("tilemap_{}.png", name);
    let output_path = output_path.as_ref().join(&output_filename);
    let input_path = input_path.as_ref().join("output/data");

    println!("output_filename: {}", output_filename);
    println!("output_path: {:#?}", output_path);
    println!("input_path: {:#?}", input_path);

    let items = fs::read_dir(&input_path)?;
    let dirs = filter_directories(&input_path, items);
    let tiles = build_tile_map(name, &dirs)?;
    let image = build_image(&tiles);

    println!("hashed {} tiles", tiles.len());

    image::save_buffer(
        &output_path,
        &image.raw,
        image.width as u32,
        image.height as u32,
        ColorType::Rgba8
    ).unwrap();

    Ok(())
}

use crate::common::*;
use crate::error::{Result, Error};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tim2;
use tim2::Pixel;

use gl_toolkit::{
    SHADER_TEXTURE,
    BufferMode,
    PrimitiveKind,
    Texture,
    VBO,
    TextureVertex,
};

use vex::{
    Vector2,
    Vector3,
};

const TEXTURE_SIZE: usize = 512;
const ATLAS_WIDTH: usize = 1024;
const TEXEL: f32 = 1.0 / ATLAS_WIDTH as f32;
const TILE_SIZE: f32 = 32.0;
const TILE_MAG: f32 = TEXEL * TILE_SIZE;

const POS: [Vector2; 4] = [
    Vector2 { x: TILE_SIZE, y: 0.0 },
    Vector2 { x: 0.0, y: 0.0 },
    Vector2 { x: 0.0, y: TILE_SIZE },
    Vector2 { x: TILE_SIZE, y: TILE_SIZE },
];

const COORDS: [Vector2; 4] = [
    Vector2 { x: TILE_MAG, y: 0.0 },
    Vector2 { x: 0.0, y: 0.0 },
    Vector2 { x: 0.0, y: TILE_MAG },
    Vector2 { x: TILE_MAG, y: TILE_MAG },
];

pub type Tile = [Layer; 2];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileKind {
    Base,
    Var,
    Anm,
}

impl TileKind {
    fn new(index: u8) -> Result<TileKind> {
        match index {
            0 => Ok(TileKind::Base),
            1 => Ok(TileKind::Var),
            2 => Ok(TileKind::Anm),
            n => Err(Error::InvalidTilesetIndex(n)),
        }
    }

    fn get_suffix(&self) -> &'static str {
        match self {
            TileKind::Base => "base",
            TileKind::Var => "var",
            TileKind::Anm => "anm",
        }
    }

    fn get_atlas_offset(&self) -> Vector2 {
        let mag = TEXTURE_SIZE as f32 * TEXEL;

        match self {
            TileKind::Base => Vector2::new(),
            TileKind::Var => Vector2::make(mag, 0.0),
            TileKind::Anm => Vector2::make(0.0, mag),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TriggerKind {
    Passable,
    Blocker,
    UpperLowerDelta,
    LowerUpperDelta,
    Hidden,
    Bridge,
    Damage,
    BottomTransparent,
    BottomHidden,
    Treasure(u8),
    Exit(u8),
    Unknown(u8),
}

impl TriggerKind {
    fn new(v: u8) -> TriggerKind {
        match v {
            0x00 => TriggerKind::Passable,
            0x01 => TriggerKind::Blocker,
            0x02 => TriggerKind::UpperLowerDelta,
            0x03 => TriggerKind::LowerUpperDelta,
            0x04 => TriggerKind::Hidden,
            0x05 => TriggerKind::Bridge,
            0x06 => TriggerKind::Damage,
            0x10 => TriggerKind::BottomTransparent,
            0x11 => TriggerKind::BottomHidden,
            0x20..=0x3F => TriggerKind::Treasure(v & 0x3F),
            0x40..=0x5F => TriggerKind::Exit(v & 0x3F),
            n => TriggerKind::Unknown(n),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Layer {
    kind: TileKind,
    trigger: TriggerKind,
    index: u8,
}

pub struct Tilemap {
    width: usize,
    height: usize,
    frame_index: usize,
    texture: Texture,
    tiles: Vec::<Tile>,
    anim_verts: Vec::<TextureVertex>,
    base_vbo: Option<VBO>,
    anim_vbo: Option<VBO>,
}

impl Tilemap {
    fn load(texture: Texture, buffer: &Vec::<u8>) -> Tilemap {
        let mut offset = 0usize;
        let width = read_u16(&buffer, &mut offset) as usize;
        let height = read_u16(&buffer, &mut offset) as usize;
        let buffer = read_slice(&buffer, &mut offset, width * height * 6);
        let tiles = Tilemap::build_tiles(&buffer, width, height);

        let (base_vbo, _) = Tilemap::build_vbo(width, &tiles, false);
        let (anim_vbo, anim_verts) = Tilemap::build_vbo(width, &tiles, true);

        Tilemap {
            frame_index: 0,
            width,
            height,
            texture,
            tiles,
            base_vbo,
            anim_vbo,
            anim_verts,
        }
    }

    fn build_tiles(buffer: &[u8], width: usize, height: usize) -> Vec<Tile> {
        let upper_offset = width * height * 2;
        let trigger_offset = upper_offset * 2;

        (0..width * height).map(|i| [
            Layer {
                kind: TileKind::new(buffer[i * 2 + 1]).unwrap(),
                trigger: TriggerKind::new(buffer[i * 2 + trigger_offset]),
                index: buffer[i * 2],
            },
            Layer {
                kind: TileKind::new(buffer[i * 2 + 1 + upper_offset]).unwrap(),
                trigger: TriggerKind::new(buffer[i * 2 + 1 + trigger_offset]),
                index: buffer[i * 2 + upper_offset],
            },
        ]).collect()
    }

    fn build_vbo(width: usize, tiles: &Vec::<Tile>, animated: bool) -> (Option<VBO>, Vec<TextureVertex>) {
        let build = |index| Tilemap::build_vertices(width, tiles, index, animated);
        let vertices = [&build(0)[..], &build(1)[..]].concat();
        let vbo = if vertices.len() > 0 {
            let mode = Tilemap::get_mode(animated);
            let indices = Tilemap::build_indices(vertices.len() as u16);
            let vbo = VBO::make(mode, PrimitiveKind::Triangles, &vertices, Some(&indices));

            Some(vbo)
        } else {
            None
        };

        (vbo, vertices)
    }

    fn build_indices(count: u16) -> Vec<u16> {
        let indices = vec![0u16, 1, 2, 0, 2, 3];

        (0..count).map(|i| {
            let offset = i * 6;
            let result: Vec::<u16> = indices.iter().map(|e| *e + offset).collect();

            result
        })
        .flatten()
        .collect()
    }

    fn build_vertices(width: usize, tiles: &Vec::<Tile>, layer_index: usize, animated: bool) -> Vec<TextureVertex> {
        tiles
            .iter()
            .enumerate()
            .filter_map(|item| Tilemap::should_process_layer(item.1, layer_index, animated))
            .map(|(i, item)| {
                let cell_pos = Vector2::make((i % width) as f32, (i / width) as f32);
                let tile_pos = Vector2::make((item.index % 16) as f32, (item.index / 16) as f32);
                let verts: Vec::<TextureVertex> = (0..4).map(|e| Tilemap::build_vertex(item.kind, layer_index, e, cell_pos, tile_pos)).collect();

                verts
            })
            .flatten()
            .collect()
    }

    fn build_vertex(kind: TileKind, layer_index: usize, corner_index: usize, cell_pos: Vector2, tile_pos: Vector2) -> TextureVertex {
        let pos = POS[corner_index] + (cell_pos * TILE_SIZE);
        let pos = Vector3::make(pos.x, pos.y, layer_index as f32 * 2.0);
        let coord = COORDS[corner_index] + (tile_pos * TILE_MAG) + kind.get_atlas_offset();
    
        TextureVertex::make_from_parts(Vector2::from(pos), coord)
    }

    fn should_process_layer(tile: &Tile, index: usize, animated: bool) -> Option<(usize, Layer)> {
        let layer = tile[index];

        let can = if animated {
            layer.kind == TileKind::Anm
        } else {
            layer.kind != TileKind::Anm
        };

        if can {
            Some((index, layer))
        } else {
            None
        }
    }

    fn get_mode(animated: bool) -> BufferMode {
        if animated {
            BufferMode::DynamicDraw
        } else {
            BufferMode::StaticDraw
        }
    }

    #[inline]
    fn animate(&mut self, index: usize) {
        if let Some(vbo) = &mut self.anim_vbo {
            let mut cell_offset = 0;

            for i in 0..2 {
                for tile in &mut self.tiles {
                    let layer = tile[i];

                    if layer.kind == TileKind::Anm {
                        let vert_offset = cell_offset * 4;
                        let x_tile = ((layer.index as usize % 16) + index) as f32;

                        for e in 0..4 {
                            self.anim_verts[vert_offset + e].coord.x = COORDS[e].x + (x_tile * TILE_MAG);
                        }

                        cell_offset += 1;
                    }
                }
            }

            vbo.write_vertices(&self.anim_verts, 0);
        }
    }

    #[inline]
    fn draw_vbo(&self, vbo: &Option<VBO>) {
        if let Some(vbo) = vbo {
            vbo.draw();
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tiles(&self) -> &Vec::<Tile> {
        &self.tiles
    }

    pub fn get_px_width(&self) -> f32 {
        self.width as f32 * TILE_SIZE
    }

    pub fn get_px_height(&self) -> f32 {
        self.height as f32 * TILE_SIZE
    }

    pub fn update(&mut self, elapsed_time: f32) {
        let curr_frame = (elapsed_time * 4.0) as usize % 4;

        if curr_frame != self.frame_index {
            self.animate(0);
            self.animate(1);

            self.frame_index = curr_frame;
        }
    }

    pub fn render(&self) {
        SHADER_TEXTURE.bind();

        self.texture.bind(0);
        self.draw_vbo(&self.base_vbo);
        self.draw_vbo(&self.anim_vbo);
    }
}

fn load_frame<P: AsRef<Path>>(path: P, name: &str, kind: TileKind) -> Result<Vec::<u8>> {
    let color_key = Pixel::from(0, 255, 0, 255);
    let filename = format!("{}_{}.tm2", name, kind.get_suffix());
    let path = path.as_ref().join(filename);

    let image = tim2::load(path)?;
    let frame = image.get_frame(0);
    let pixels = frame.to_raw(Some(color_key));

    Ok(pixels)
}

fn build_texture<P: AsRef<Path>>(path: P, name: &str) -> Result<Texture> {
    let base_pixels = load_frame(&path, name, TileKind::Base)?;
    let var_pixels = load_frame(&path, name, TileKind::Var)?;
    let anm_pixels = load_frame(&path, name, TileKind::Anm)?;
    let tex = Texture::new(1024, 1024);

    tex.write(&base_pixels, 0, 0, TEXTURE_SIZE, TEXTURE_SIZE);
    tex.write(&var_pixels, TEXTURE_SIZE, 0, TEXTURE_SIZE, TEXTURE_SIZE);
    tex.write(&anm_pixels, 0, TEXTURE_SIZE, TEXTURE_SIZE, TEXTURE_SIZE);

    Ok(tex)
}

pub fn load<P: AsRef<Path>>(path: P, map_filename: &str, set_name: &str) -> Result<Tilemap> {
    let texture = build_texture(&path, set_name)?;
    let file_path = path.as_ref().join(map_filename);
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(Tilemap::load(texture, &buffer))
}

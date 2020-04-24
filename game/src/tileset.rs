use crate::common::*;
use crate::error::Result;

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
    Vertex,
    TextureVertex,
};

use vex::{
    Matrix,
    Matrix4,
    Vector2,
    Vector3,
};

const TEXTURE_SIZE: usize = 512;
const ATLAS_WIDTH: usize = 1024;
const TEXEL: f32 = 1.0 / ATLAS_WIDTH as f32;
const TILE_SIZE: f32 = 32.0;
const TILE_MAG: f32 = TEXEL * TILE_SIZE;

const POS: [f32; 8] = [
    TILE_SIZE, 0.0,
    0.0, 0.0,
    0.0, TILE_SIZE,
    TILE_SIZE, TILE_SIZE,
];

const COORDS: [f32; 8] = [
    TILE_MAG, 0.0,
    0.0, 0.0,
    0.0, TILE_MAG,
    TILE_MAG, TILE_MAG,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TextureKind {
    Base,
    Var,
    Anm,
}

impl TextureKind {
    fn make(index: usize) -> Result<TextureKind> {
        match index {
            0 => Ok(TextureKind::Base),
            1 => Ok(TextureKind::Var),
            2 => Ok(TextureKind::Anm),
            // n => Err(Error::InvalidTilesetIndex(n)),
            _ => Ok(TextureKind::Anm),
        }
    }

    fn get_native(&self) -> usize {
        match self {
            TextureKind::Base => 0,
            TextureKind::Var => 1,
            TextureKind::Anm => 2,
        }
    }

    fn get_suffix(&self) -> &'static str {
        match self {
            TextureKind::Base => "base",
            TextureKind::Var => "var",
            TextureKind::Anm => "anm",
        }
    }

    fn get_atlas_offset(&self) -> Vector2 {
        let mag = TEXTURE_SIZE as f32 * TEXEL;

        match self {
            TextureKind::Base => Vector2::new(),
            TextureKind::Var => Vector2::make(mag, 0.0),
            TextureKind::Anm => Vector2::make(0.0, mag),
        }
    }
}

pub struct Cell {
    index: u8,
    kind: TextureKind,
}

impl Cell {
    fn should_process(&self, animated: bool) -> bool {    
        if animated {
            self.kind == TextureKind::Anm
        } else {
            self.kind != TextureKind::Anm
        }
    }
}

pub struct Layer {
    base_vbo: VBO,
    anim_vbo: VBO,
    anim_verts: Vec::<TextureVertex>,
    cells: Vec::<Cell>,
}

impl Layer {
    fn update_anim_uvs(&mut self, frame_index: usize) {
        let mut cell_offset = 0;

        for cell in self.cells.iter() {
            if cell.kind == TextureKind::Anm {
                let vert_offset = cell_offset * 4;
                let x_tile = ((cell.index as usize % 16) + frame_index) as f32;

                for e in 0..4 {
                    self.anim_verts[vert_offset + e].coord.x = COORDS[e * 2] + (x_tile * TILE_MAG);
                }

                cell_offset += 1;
            }
        }

        self.anim_vbo.write_vertices(&self.anim_verts, 0);
    }
}

pub struct Tileset {
    width: usize,
    height: usize,
    frame_index: usize,
    layers: Vec::<Layer>,
    texture: Texture,
}

impl Tileset {
    fn load(texture: Texture, buffer: &Vec::<u8>) -> Tileset {
        let mut offset = 0usize;
        let width = read_u16(&buffer, &mut offset) as usize;
        let height = read_u16(&buffer, &mut offset) as usize;
        let cell_count = width * height;
        let layers = (0..2).map(|_| { /* TODO: figure out what Layer 3 does... */
            let slice = read_slice(&buffer, &mut offset, cell_count * 2);
            let cells = (0..(width * height)).map(|e| {
                Cell {
                    index: slice[e * 2 + 0],
                    kind: TextureKind::make(slice[e * 2 + 1] as usize).unwrap(),
                }
            }).collect();

            let (_, base_vbo) = Tileset::build_vbo(width, height, &cells, false);
            let (anim_verts, anim_vbo) = Tileset::build_vbo(width, height, &cells, true);

            Layer {
                base_vbo,
                anim_vbo,
                anim_verts,
                cells,
            }
        }).collect();

        Tileset {
            frame_index: 0,
            width,
            height,
            layers,
            texture,
        }
    }

    fn build_vbo(width: usize, height: usize, cells: &Vec::<Cell>, animated: bool) -> (Vec::<TextureVertex>, VBO) {
        let map_width = width as f32 * TILE_SIZE;
        let map_height = height as f32 * TILE_SIZE;
        let mut vertices = vec![TextureVertex::new(); cells.len() * 4];
        let mut indices = vec![0; cells.len() * 6];
        let mut cell_offset = 0;

        let mode = if animated {
            BufferMode::StaticDraw
        } else {
            BufferMode::DynamicDraw
        };

        for (i, cell) in cells.iter().enumerate() {
            if cell.should_process(animated) {
                let vert_offset = cell_offset * 4;
                let index_offset = cell_offset * 6;
                let x_cell = (i % width) as f32;
                let y_cell = (i / width) as f32;
                let x_tile = (cell.index % 16) as f32;
                let y_tile = (cell.index / 16) as f32;
    
                vertices[vert_offset + 0] = Tileset::build_vertex(map_width, map_height, x_cell, y_cell, x_tile, y_tile, 0, cell.kind);
                vertices[vert_offset + 1] = Tileset::build_vertex(map_width, map_height, x_cell, y_cell, x_tile, y_tile, 1, cell.kind);
                vertices[vert_offset + 2] = Tileset::build_vertex(map_width, map_height, x_cell, y_cell, x_tile, y_tile, 2, cell.kind);
                vertices[vert_offset + 3] = Tileset::build_vertex(map_width, map_height, x_cell, y_cell, x_tile, y_tile, 3, cell.kind);
    
                indices[index_offset + 0] = vert_offset as u16 + 0;
                indices[index_offset + 1] = vert_offset as u16 + 1;
                indices[index_offset + 2] = vert_offset as u16 + 3;
                indices[index_offset + 3] = vert_offset as u16 + 1;
                indices[index_offset + 4] = vert_offset as u16 + 2;
                indices[index_offset + 5] = vert_offset as u16 + 3;

                cell_offset += 1;
            }
        }

        vertices.shrink_to_fit();
        indices.shrink_to_fit();

        let vbo = VBO::make(mode, PrimitiveKind::Triangles, &vertices, Some(&indices));

        (vertices, vbo)
    }

    fn build_vertex(map_width: f32, map_height: f32, x_cell: f32, y_cell: f32, x_tile: f32, y_tile: f32, corner_index: usize, kind: TextureKind) -> TextureVertex {
        let proj_mat = Matrix4::ortho(0.0, map_width, 0.0, map_height, 0.0, 1000.0);

        let x = POS[corner_index * 2 + 0] + (x_cell * TILE_SIZE);
        let y = POS[corner_index * 2 + 1] + (y_cell * TILE_SIZE);
        let u = COORDS[corner_index * 2 + 0] + (x_tile * TILE_MAG);
        let v = COORDS[corner_index * 2 + 1] + (y_tile * TILE_MAG);
    
        let pos = Vector2::from(proj_mat.transform_point(&Vector3::make(x, y, 0.0)));
        let coord = Vector2::make(u, v) + kind.get_atlas_offset();
    
        TextureVertex::make_from_parts(pos, coord)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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
            for layer in &mut self.layers {
                layer.update_anim_uvs(curr_frame);
            }

            self.frame_index = curr_frame;
        }
    }

    pub fn render(&self) {
        SHADER_TEXTURE.bind();

        self.texture.bind(0);
        
        for layer in &self.layers {
            layer.base_vbo.draw();
            layer.anim_vbo.draw();
        }
    }
}

fn load_frame<P: AsRef<Path>>(path: P, name: &str, kind: TextureKind) -> Result<Vec::<u8>> {
    let color_key = Pixel::from(0, 255, 0, 255);
    let filename = format!("{}_{}.tm2", name, kind.get_suffix());
    let path = path.as_ref().join(filename);

    let image = tim2::load(path)?;
    let frame = image.get_frame(0);
    let pixels = frame.to_raw(Some(color_key));

    Ok(pixels)
}

fn build_texture<P: AsRef<Path>>(path: P, name: &str) -> Result<Texture> {
    let base_pixels = load_frame(&path, name, TextureKind::Base)?;
    let var_pixels = load_frame(&path, name, TextureKind::Var)?;
    let anm_pixels = load_frame(&path, name, TextureKind::Anm)?;
    let tex = Texture::new(1024, 1024);

    tex.write(&base_pixels, 0, 0, TEXTURE_SIZE, TEXTURE_SIZE);
    tex.write(&var_pixels, TEXTURE_SIZE, 0, TEXTURE_SIZE, TEXTURE_SIZE);
    tex.write(&anm_pixels, 0, TEXTURE_SIZE, TEXTURE_SIZE, TEXTURE_SIZE);

    Ok(tex)
}

pub fn load<P: AsRef<Path>>(path: P, map_filename: &str, set_name: &str) -> Result<Tileset> {
    let texture = build_texture(&path, set_name)?;
    let file_path = path.as_ref().join(map_filename);
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(Tileset::load(texture, &buffer))
}

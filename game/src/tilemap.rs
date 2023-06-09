use cgmath::Vector2;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tim2::Pixel;
use wgpu::*;

use crate::common::*;
use crate::error::{Result, Error};

use crate::graphics::{
	Camera,
	Core as GraphicsCore,
	Mesh,
	TextureVertex,
	Texture,
};

const TEXTURE_SIZE: u32 = 512;
const ATLAS_WIDTH: u32 = 1024;
const TEXEL: f32 = 1.0 / ATLAS_WIDTH as f32;
const TILE_SIZE: f32 = 32.0;
const TILE_MAG: f32 = TEXEL * TILE_SIZE;
const TILE_INDEX_OFFSETS: [u16; 6] = [0, 1, 2, 0, 2, 3];

const POS: [Vector2<f32>; 4] = [
	Vector2 { x: TILE_SIZE, y: 0.0 },
	Vector2 { x: 0.0, y: 0.0 },
	Vector2 { x: 0.0, y: TILE_SIZE },
	Vector2 { x: TILE_SIZE, y: TILE_SIZE },
];

const UV: [Vector2<f32>; 4] = [
	Vector2 { x: TILE_MAG, y: 0.0 },
	Vector2 { x: 0.0, y: 0.0 },
	Vector2 { x: 0.0, y: TILE_MAG },
	Vector2 { x: TILE_MAG, y: TILE_MAG },
];

#[derive(Copy, Clone, Debug)]
enum TriggerKind {
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

#[derive(Copy, Clone, Debug)]
struct Layer {
	kind: TileKind,
	trigger: TriggerKind,
	index: u8,
}

type Tile = [Layer; 2];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TileKind {
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

	fn get_atlas_offset(&self) -> Vector2<f32> {
		let mag = TEXTURE_SIZE as f32 * TEXEL;

		match self {
			TileKind::Base => Vector2::new(0.0, 0.0),
			TileKind::Var => Vector2::new(mag, 0.0),
			TileKind::Anm => Vector2::new(0.0, mag),
		}
	}
}

pub struct Tilemap {
	width: u32,
	height: u32,
	frame_index: u32,
	texture: Texture,
	tiles: Vec<Tile>,
	meshes: Vec<Mesh<TextureVertex>>,
}

impl Tilemap {
	fn load(core: &GraphicsCore, texture: Texture, buffer: &Vec::<u8>) -> Tilemap {
		let mut offset = 0usize;
		let width = read_u16(&buffer, &mut offset) as u32;
		let height = read_u16(&buffer, &mut offset) as u32;
		let buffer_length = (width * height * 6) as usize;
		let buffer = read_slice(&buffer, &mut offset, buffer_length);
		let tiles = Self::build_tiles(&buffer, width, height);

		let meshes = [
			Self::build_layer_mesh(&core, &tiles, width, 0, false),
			Self::build_layer_mesh(&core, &tiles, width, 0, true),
			Self::build_layer_mesh(&core, &tiles, width, 1, false),
			Self::build_layer_mesh(&core, &tiles, width, 1, true),
		].into_iter().filter_map(std::convert::identity).collect();

		Tilemap {
			width,
			height,
			frame_index: 0,
			texture,
			tiles,
			meshes,
		}
	}

	fn build_tiles(buffer: &[u8], width: u32, height: u32) -> Vec<Tile> {
		let count = (width * height) as usize;
		let upper_offset = count * 2;
		let trigger_offset = upper_offset * 2;

		println!("dims<{}, {}>", width, height);
		println!("count: {}", count);

		(0..count).map(|i| [
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

	fn build_layer_mesh(
		core: &GraphicsCore,
		tiles: &Vec::<Tile>,
		width: u32,
		index: usize,
		animated: bool,
	) -> Option<Mesh<TextureVertex>> {
		let vertices = Self::build_layer_vertices(tiles, width, index, animated);
	
		if vertices.len() > 0 {
			let indices = Self::build_indices(tiles.len());
			let mesh = Mesh::make(core.device(), &vertices, Some(&indices), animated);

			Some(mesh)
		} else {
			None
		}
	}

	fn build_layer_vertices(
		tiles: &Vec::<Tile>,
		width: u32,
		index: usize,
		animated: bool
	) -> Vec<TextureVertex> {
		let width = width as usize;

		tiles
			.iter()
			.enumerate()
			.filter(|item| Self::should_process_layer(item.1, index, animated))
			.map(|(i, item)| {
				let layer = item[index];
				let cell_x = (i % width) as f32;
				let cell_y = (i / width) as f32;
				let tile_x = (layer.index % 16) as f32;
				let tile_y = (layer.index / 16) as f32;
				let cell_pos = Vector2::new(cell_x, cell_y);
				let tile_pos = Vector2::new(tile_x, tile_y);

				let verts: Vec::<TextureVertex> = (0..4)
					.map(|e| Self::build_vertex(
						layer.kind,
						index,
						e,
						cell_pos,
						tile_pos
					))
					.collect();

				verts
			})
			.flatten()
			.collect()
	}

	fn build_indices(count: usize) -> Vec<u16> {
		(0..count).map(|i| {
			let offset = (i * 4) as u16;
			let result = TILE_INDEX_OFFSETS.map(|e| e + offset);

			result
		})
		.flatten()
		.collect()
	}

	fn build_vertex(kind: TileKind, layer_index: usize, corner_index: usize, cell_pos: Vector2<f32>, tile_pos: Vector2<f32>) -> TextureVertex {
		let offset_pos = POS[corner_index] + (cell_pos * TILE_SIZE);
        let offset_uv = UV[corner_index] + (tile_pos * TILE_MAG) + kind.get_atlas_offset();

		TextureVertex {
			x: offset_pos.x,
			y: offset_pos.y,
			z: layer_index as f32 * -2.0,
			u: offset_uv.x,
			v: offset_uv.y,
		}
	}

	fn should_process_layer(tile: &Tile, layer_index: usize, animated: bool) -> bool {
		let layer = tile[layer_index];
		let valid_tile = layer.index > 0;

		let valid_animation = if animated {
			layer.kind == TileKind::Anm
		} else {
			layer.kind != TileKind::Anm
		};

		valid_tile && valid_animation
	}

	#[inline]
	fn animate(&mut self, index: u32) {
		// if let Some(mesh) = &mut self.anim_mesh {
		// 	let mut cell_offset = 0;

		// 	for i in 0..2 {
		// 		for tile in &mut self.tiles {
		// 			let layer = tile[i];

		// 			if layer.kind == TileKind::Anm {
		// 				let vert_offset = cell_offset * 4;
		// 				let x_tile = ((layer.index as u32 % 16) + index) as f32;

		// 				for e in 0..4 {
		// 					self.anim_verts[vert_offset + e].u = UV[e].x + (x_tile * TILE_MAG);
		// 				}

		// 				cell_offset += 1;
		// 			}
		// 		}
		// 	}

		// 	mesh.write_vertices(&self.anim_verts, 0);
		// }
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
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
		let curr_frame = (elapsed_time * 4.0) as u32 % 4;

		if curr_frame != self.frame_index {
			self.animate(0);
			self.animate(1);

			self.frame_index = curr_frame;
		}
	}

	pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, camera: &'a Camera) {
		render_pass.set_bind_group(0, camera.bind_group(), &[]);
		render_pass.set_bind_group(1, self.texture.bind_group(), &[]);

		self.meshes.iter().for_each(|mesh| mesh.render(render_pass));
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

fn build_texture<P: AsRef<Path>>(core: &GraphicsCore, path: P, name: &str) -> Result<Texture> {
	const ORIGIN_BASE: Origin3d = Origin3d::ZERO;

	const ORIGIN_VAR: Origin3d = Origin3d {
		x: TEXTURE_SIZE,
		y: 0,
		z: 0,
	};

	const ORIGIN_ANM: Origin3d = Origin3d {
		x: 0,
		y: TEXTURE_SIZE,
		z: 0,
	};

	let base_data = load_frame(&path, name, TileKind::Base)?;
	let var_data = load_frame(&path, name, TileKind::Var)?;
	let anm_data = load_frame(&path, name, TileKind::Anm)?;
	let layout = &core.pipeline().get_bind_group_layout(1);
	let result = Texture::new(core.device(), layout, 1024, 1024);

	result.write(core.queue(), &base_data, ORIGIN_BASE, TEXTURE_SIZE, TEXTURE_SIZE);
	result.write(core.queue(), &var_data, ORIGIN_VAR, TEXTURE_SIZE, TEXTURE_SIZE);
	result.write(core.queue(), &anm_data, ORIGIN_ANM, TEXTURE_SIZE, TEXTURE_SIZE);

	Ok(result)
}

pub fn load<P: AsRef<Path>>(core: &GraphicsCore, path: P, map_filename: &str, set_name: &str) -> Result<Tilemap> {
	let texture = build_texture(core, &path, set_name)?;
	let file_path = path.as_ref().join(map_filename);
	let mut file = File::open(file_path)?;
	let mut buffer = Vec::new();

	file.read_to_end(&mut buffer)?;

	Ok(Tilemap::load(core, texture, &buffer))
}

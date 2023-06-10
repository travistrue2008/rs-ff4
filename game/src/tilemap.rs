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
const INDEX_OFFSETS: [u16; 6] = [0, 1, 2, 0, 2, 3];

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CellKind {
	Base,
	Var,
	Anm,
}

impl CellKind {
	fn new(index: u8) -> Result<CellKind> {
		match index {
			0 => Ok(CellKind::Base),
			1 => Ok(CellKind::Var),
			2 => Ok(CellKind::Anm),
			n => Err(Error::InvalidCellKindIndex(n)),
		}
	}

	fn get_suffix(&self) -> &'static str {
		match self {
			CellKind::Base => "base",
			CellKind::Var => "var",
			CellKind::Anm => "anm",
		}
	}

	fn get_atlas_offset(&self) -> Vector2<f32> {
		let mag = TEXTURE_SIZE as f32 * TEXEL;

		match self {
			CellKind::Base => Vector2::new(0.0, 0.0),
			CellKind::Var => Vector2::new(mag, 0.0),
			CellKind::Anm => Vector2::new(0.0, mag),
		}
	}
}

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

	fn to_raw(&self) -> u8 {
		match self {
			TriggerKind::Passable => 0x00,
			TriggerKind::Blocker => 0x01,
			TriggerKind::UpperLowerDelta => 0x02,
			TriggerKind::LowerUpperDelta => 0x03,
			TriggerKind::Hidden => 0x04,
			TriggerKind::Bridge => 0x05,
			TriggerKind::Damage => 0x06,
			TriggerKind::BottomTransparent => 0x10,
			TriggerKind::BottomHidden => 0x11,
			TriggerKind::Treasure(v) => *v,
			TriggerKind::Exit(v) => *v,
			TriggerKind::Unknown(v) => *v,
		}
	}
}

#[derive(Copy, Clone, Debug)]
struct Cell {
	kind: CellKind,
	trigger: TriggerKind,
	tile_index: u8,
	position: Vector2<f32>,
}

impl Cell {
	fn get_tile_position(&self) -> Vector2<f32> {
		let tile_x = (self.tile_index % 16) as f32;
		let tile_y = (self.tile_index / 16) as f32;

		Vector2::new(tile_x, tile_y)
	}
}

struct Layer {
	cells: Vec<Cell>,
	static_mesh: Option<Mesh>,
	animated_mesh: Option<Mesh>,
	animated_vertices: Vec<TextureVertex>,
}

impl Layer {
	fn new(core: &GraphicsCore, cells: &Vec<Cell>, index: usize) -> Layer {
		let static_vertices = Self::build_vertices(cells, index, false);
		let animated_vertices = Self::build_vertices(cells, index, true);

		Layer {
			cells: cells.to_vec(),
			static_mesh: Self::build_mesh(core, cells, &static_vertices),
			animated_mesh: Self::build_mesh(core, cells, &animated_vertices),
			animated_vertices,
		}
	}

	fn build_mesh(
		core: &GraphicsCore,
		cells: &Vec<Cell>,
		vertices: &Vec<TextureVertex>,
	) -> Option<Mesh> {
		if vertices.len() > 0 {
			let indices = Self::build_indices(cells.len());
			let mesh = Mesh::make(core.device(), &vertices, Some(&indices));

			Some(mesh)
		} else {
			None
		}
	}

	fn build_vertices(cells: &Vec<Cell>, index: usize, animated: bool) -> Vec<TextureVertex> {
		let offset_z = index as f32 * -2.0;

		cells
			.iter()
			.filter(|cell| Self::has_verts_for_cell(cell, animated))
			.map(|cell| {
				let tile_position = cell.get_tile_position();

				let verts: Vec<TextureVertex> = (0..4)
					.map(|e| Self::build_vertex(cell, offset_z, e, tile_position))
					.collect();

				verts
			})
			.flatten()
			.collect()
	}

	fn build_vertex(cell: &Cell, offset_z: f32, corner_index: usize, tile_pos: Vector2<f32>) -> TextureVertex {
		let uv_offset = cell.kind.get_atlas_offset();
		let offset_pos = POS[corner_index] + (cell.position * TILE_SIZE);
        let offset_uv = UV[corner_index] + (tile_pos * TILE_MAG) + uv_offset;

		TextureVertex {
			x: offset_pos.x,
			y: offset_pos.y,
			z: offset_z,
			u: offset_uv.x,
			v: offset_uv.y,
		}
	}

	fn build_indices(count: usize) -> Vec<u16> {
		(0..count).map(|i| {
			let offset = (i * 4) as u16;
			let result = INDEX_OFFSETS.map(|e| e + offset);

			result
		})
		.flatten()
		.collect()
	}

	fn has_verts_for_cell(cell: &Cell, animated: bool) -> bool {
		if animated {
			cell.kind == CellKind::Anm
		} else {
			cell.kind != CellKind::Anm
		}
	}

	#[inline]
	pub fn update(&mut self, queue: &Queue, frame_index: usize) {
		if let Some(mesh) = self.animated_mesh.as_mut() {
			let x_atlas = CellKind::Anm.get_atlas_offset().x;

			self.cells
				.iter()
				.filter(|cell| cell.kind == CellKind::Anm)
				.enumerate()
				.for_each(|(i, cell)| {
					for e in 0..4 {
						let index = i * 4 + e;
						let x_cell = cell.get_tile_position().x;
						let x_position = x_cell + frame_index as f32;
						let result = UV[e].x + (x_position * TILE_MAG) + x_atlas;

						self.animated_vertices[index].u = result;
					}
				});

			mesh.write_vertices(queue, &self.animated_vertices);
		}
	}

	pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
		if let Some(mesh) = &self.static_mesh {
			mesh.render(render_pass);
		}

		if let Some(mesh) = &self.animated_mesh {
			mesh.render(render_pass);
		}
	}
}

pub struct Tilemap {
	width: u16,
	height: u16,
	frame_index: usize,
	texture: Texture,
	layers: [Layer; 2],
	collision: Vec<u8>,
}

impl Tilemap {
	fn load(
		core: &GraphicsCore,
		texture: Texture,
		map_buffer: &Vec<u8>,
		collision_buffer: &Vec<u8>
	) -> Tilemap {
		let mut offset = 0usize;
		let width = read_u16(&map_buffer, &mut offset);
		let height = read_u16(&map_buffer, &mut offset);
		let buffer_length = (width * height * 6) as usize;
		let buffer = read_slice(&map_buffer, &mut offset, buffer_length);
		let (lower_cells, upper_cells) = Self::build_cells(&buffer, width, height);

		let layers = [
			Layer::new(core, &lower_cells, 0),
			Layer::new(core, &upper_cells, 1),
		];

		Tilemap {
			width,
			height,
			frame_index: 0,
			texture,
			layers,
			collision: collision_buffer.to_vec(),
		}
	}

	fn build_cells(buffer: &[u8], width: u16, height: u16) -> (Vec<Cell>, Vec<Cell>) {
		let count = (width * height) as usize;
		let upper_offset = count * 2;
		let trigger_offset = upper_offset * 2;
		let mut lower_cells = Vec::new();
		let mut upper_cells = Vec::new();

		for i in 0..count {
			let x = (i as u16 % width) as f32;
			let y = (i as u16 / width) as f32;

			lower_cells.push(Cell {
				kind: CellKind::new(buffer[i * 2 + 1]).unwrap(),
				trigger: TriggerKind::new(buffer[i * 2 + trigger_offset]),
				tile_index: buffer[i * 2],
				position: Vector2::new(x, y),
			});

			upper_cells.push(Cell {
				kind: CellKind::new(buffer[i * 2 + 1 + upper_offset]).unwrap(),
				trigger: TriggerKind::new(buffer[i * 2 + 1 + trigger_offset]),
				tile_index: buffer[i * 2 + upper_offset],
				position: Vector2::new(x, y),
			});
		}

		(
			Self::filter_layer_cells(lower_cells),
			Self::filter_layer_cells(upper_cells),
		)
	}

	fn filter_layer_cells(cells: Vec<Cell>) -> Vec<Cell> {
		cells
			.into_iter()
			.filter(|cell| cell.kind == CellKind::Anm || cell.tile_index > 0)
			.collect()
	}

	pub fn width(&self) -> u16 {
		self.width
	}

	pub fn height(&self) -> u16 {
		self.height
	}

	pub fn get_px_width(&self) -> f32 {
		self.width as f32 * TILE_SIZE
	}

	pub fn get_px_height(&self) -> f32 {
		self.height as f32 * TILE_SIZE
	}

	pub fn update(&mut self, queue: &Queue, elapsed_time: f32) {
		let curr_frame = (elapsed_time * 4.0) as usize % 4;

		if curr_frame != self.frame_index {
			self.frame_index = curr_frame;
			self.layers[0].update(queue, self.frame_index);
			self.layers[1].update(queue, self.frame_index);
		}
	}

	pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, camera: &'a Camera) {
		render_pass.set_bind_group(0, camera.bind_group(), &[]);
		render_pass.set_bind_group(1, self.texture.bind_group(), &[]);

		self.layers[0].render(render_pass);
		self.layers[1].render(render_pass);
	}

	pub fn print_triggers(&self, layer_index: usize) {
		let layer = &self.layers[layer_index];

		for y in 0..self.height {
			for x in 0..self.height {
				let cell = layer.cells
					.iter()
					.find(|cell| x == cell.position.x as u16 && y == cell.position.x as u16);

				let trigger = if let Some(cell) = cell {
					cell.trigger.to_raw()
				} else {
					0
				};

				print!("{:02x?} ", trigger);
			}

			println!("");
		}
	}

	pub fn print_collision(&self, layer_index: usize) {
		let layer_size = self.collision.len() / 2;
		let layer_offset = layer_size * layer_index;

		for y in 0..self.height {
			for x in 0..self.width {
				let index = (y * self.width + x) as usize + layer_offset;
				let flag = self.collision[index];

				print!("{:02x?} ", flag);
			}

			println!("");
		}
	}
}

fn load_frame<P: AsRef<Path>>(path: P, name: &str, kind: CellKind) -> Result<Vec<u8>> {
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

	let base_data = load_frame(&path, name, CellKind::Base)?;
	let var_data = load_frame(&path, name, CellKind::Var)?;
	let anm_data = load_frame(&path, name, CellKind::Anm)?;
	let layout = &core.pipeline().get_bind_group_layout(1);
	let result = Texture::new(core.device(), layout, 1024, 1024);

	result.write(core.queue(), &base_data, ORIGIN_BASE, TEXTURE_SIZE, TEXTURE_SIZE);
	result.write(core.queue(), &var_data, ORIGIN_VAR, TEXTURE_SIZE, TEXTURE_SIZE);
	result.write(core.queue(), &anm_data, ORIGIN_ANM, TEXTURE_SIZE, TEXTURE_SIZE);

	Ok(result)
}

fn build_map_buffer<P: AsRef<Path>>(path: P, name: &str) -> Result<Vec<u8>> {
	let filename = format!("{}.cn2", name);
	let file_path = path.as_ref().join(filename);
	let mut file = File::open(file_path)?;
	let mut buffer = Vec::new();

	file.read_to_end(&mut buffer)?;

	Ok(buffer)
}

fn build_collision_buffer<P: AsRef<Path>>(path: P, name: &str) -> Result<Vec<u8>> {
	let filename = format!("{}_hit.cns", name);
	let file_path = path.as_ref().join(filename);
	let mut file = File::open(file_path)?;
	let mut buffer = Vec::new();

	file.read_to_end(&mut buffer)?;

	Ok(buffer)
}

pub fn load<P: AsRef<Path>>(core: &GraphicsCore, path: P, map_name: &str, set_name: &str) -> Result<Tilemap> {
	let texture = build_texture(core, &path, set_name)?;
	let map_buffer = build_map_buffer(&path, map_name)?;
	let collision_buffer = build_collision_buffer(&path, map_name)?;

	Ok(Tilemap::load(core, texture, &map_buffer, &collision_buffer))
}

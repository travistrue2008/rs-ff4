use wgpu::*;
use wgpu::util::DeviceExt;
use super::super::error::Error;
use super::vertex::Vertex;

pub struct Mesh<TVertex: Vertex> {
	index_count: u32,
	vertex_count: u32,
	index_buffer: Option<Buffer>,
	vertex_buffer: Buffer,
	vertices: Option<Vec<TVertex>>,
}

impl<TVertex: Vertex> Mesh<TVertex> {
	pub fn make(
		device: &Device,
		vertices: &Vec::<TVertex>,
		indices: Option<&Vec::<u16>>,
		store_vertices: bool,
	) -> Mesh<TVertex> {
		let index_count = match indices {
			Some(indices) => indices.len() as u32,
			None => 0,
		};

		let index_buffer = match indices {
			Some(indices) => Some(
				device.create_buffer_init(
					&wgpu::util::BufferInitDescriptor {
						label: Some("Mesh"),
						usage: BufferUsages::INDEX,
						contents: bytemuck::cast_slice(indices),
					}
				)
			),
			None => None,
		};

		let vertex_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Mesh"),
				usage: BufferUsages::VERTEX,
				contents: bytemuck::cast_slice(vertices),
			}
		);

		let verts = if store_vertices {
			Some(vertices.clone())
		} else {
			None
		};

		Mesh {
			index_count,
			vertex_count: vertices.len() as u32,
			index_buffer,
			vertex_buffer,
			vertices: verts,
		}
	}

	pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

		match &self.index_buffer {
			Some(index_buffer) => {
				render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
				render_pass.draw_indexed(0..self.index_count, 0, 0..1);
			},
			None => {
				render_pass.draw(0..self.vertex_count, 0..1);
			},
		}
	}

	pub fn write(&self, queue: &Queue) -> Result<(), Error> {
		if let Some(vertices) = self.vertices.as_ref() {
			let buffer = bytemuck::cast_slice(vertices);

			queue.write_buffer(&self.vertex_buffer, 0, buffer);

			Ok(())
		} else {
			Err(Error::MeshWriteWithNoVertices)
		}
	}

	pub fn vertices(&self) -> &Option<Vec<TVertex>> {
		&self.vertices
	}

	pub fn vertices_mut(&mut self) -> &mut Option<Vec<TVertex>> {
		&mut self.vertices
	}
}

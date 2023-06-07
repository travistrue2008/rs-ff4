use bytemuck::*;
use wgpu::*;
use wgpu::util::DeviceExt;
use super::vertex::Vertex;

pub struct Mesh {
	index_count: u32,
	vertex_count: u32,
	index_buffer: Option<Buffer>,
	vertex_buffer: Buffer,
}

impl Mesh {
	pub fn make<T: Vertex + Pod + Zeroable>(device: &Device, vertices: &Vec::<T>, indices: Option<&Vec::<u16>>) -> Mesh {
		let index_count = match indices {
			Some(indices) => indices.len() as u32,
			None => 0,
		};

		let index_buffer = match indices {
			Some(indices) => Some(
				device.create_buffer_init(
					&wgpu::util::BufferInitDescriptor {
						label: None,
						usage: BufferUsages::INDEX,
						contents: bytemuck::cast_slice(indices),
					}
				)
			),
			None => None,
		};

		let vertex_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: None,
				usage: BufferUsages::VERTEX,
				contents: bytemuck::cast_slice(vertices),
			}
		);

		Mesh {
			index_count,
			vertex_count: vertices.len() as u32,
			index_buffer,
			vertex_buffer,
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
}

use bytemuck::*;

pub trait Vertex: Pod + Zeroable {
	fn layout() -> wgpu::VertexBufferLayout<'static>;
}

type PositionVertex = [f32; 3];

impl Vertex for PositionVertex {
	fn layout() -> wgpu::VertexBufferLayout<'static> {
		const ATTRS: [wgpu::VertexAttribute; 1] =
			wgpu::vertex_attr_array![0 => Float32x3];
	
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<PositionVertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &ATTRS,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TextureVertex {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub u: f32,
	pub v: f32,
}

impl Vertex for TextureVertex {
	fn layout() -> wgpu::VertexBufferLayout<'static> {
		const ATTRS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
			0 => Float32x3,
			1 => Float32x2,
		];
	
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &ATTRS,
		}
	}
}

use cgmath::{Matrix4, SquareMatrix};
use wgpu::*;
use wgpu::util::DeviceExt;

#[rustfmt::skip]
pub const MATRIX_OPENGL_TO_WGPU: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
	1.0, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 0.5, 0.0,
	0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
	proj_matrix: Matrix4<f32>,
	view_matrix: Matrix4<f32>,
	clip_matrix: Matrix4<f32>,
	buffer: Buffer,
	bind_group: BindGroup,
}

impl Camera {
	pub fn new(device: &Device, layout: &BindGroupLayout) -> Self {
		let uniform: [[f32; 4]; 4] = Matrix4::identity().into();

		let buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Camera"),
				contents: bytemuck::cast_slice(&[uniform]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
		);

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("Camera"),
			layout: &layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: buffer.as_entire_binding(),
				}
			],
		});

		Camera {
			proj_matrix: Matrix4::identity(),
			view_matrix: Matrix4::identity(),
			clip_matrix: Matrix4::identity(),
			buffer,
			bind_group,
		}
	}

	fn compute(&mut self) {
		self.clip_matrix =
			MATRIX_OPENGL_TO_WGPU *
			self.proj_matrix *
			self.view_matrix;
	}

	pub fn ortho(&mut self, width: f32, height: f32) {
		self.proj_matrix = cgmath::ortho(0.0, width, height, 0.0, 0.0, 1000.0);
		self.compute();
	}

	pub fn update(&self, queue: &Queue) {
		let uniform: [[f32; 4]; 4] = self.clip_matrix.into();

		queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
	}

	pub fn bind_group(&self) -> &BindGroup {
		&self.bind_group
	}
}

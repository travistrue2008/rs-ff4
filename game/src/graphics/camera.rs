use cgmath::{Matrix4};
use lazy_static::lazy_static;

#[rustfmt::skip]
pub const MATRIX_OPENGL_TO_WGPU: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
	1.0, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 0.5, 0.0,
	0.0, 0.0, 0.5, 1.0,
);

lazy_static! {
	pub static ref MATRIX_PROJECTION: Matrix4<f32> =
		MATRIX_OPENGL_TO_WGPU *
		cgmath::ortho::<f32>(0.0, 480.0, 0.0, 272.0, 0.0, 1000.0);
}

pub trait Uniform {
}

pub type UniformMatrix = [[f32; 4]; 4];

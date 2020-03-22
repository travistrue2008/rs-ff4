extern crate gl;

use gl::types::*;
use std::mem;
use std::ptr;

pub fn make() {
	const vertices: [f32;8] = [
		-1.0,  1.0,
		 1.0,  1.0,
		 1.0, -1.0,
		-1.0, -1.0,
	];

	let (mut vbo, mut vao) = (0, 0);
	let vert_ptr = &vertices[0] as *const f32 as *const c_void;
	let buffer_size = (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
	let pos_size = 3 * mem::size_of::<GLfloat>() as GLsizei;

	unsafe {
		gl::GenVertexArrays(1, &mut vao);
		gl::GenBuffers(1, &mut vbo);
		gl::BindVertexArray(vao);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(gl::ARRAY_BUFFER, buffer_size, vert_ptr, gl::STATIC_DRAW);

		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, pos_size, ptr::null());
		gl::EnableVertexAttribArray(0);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);

		vao
	}
}

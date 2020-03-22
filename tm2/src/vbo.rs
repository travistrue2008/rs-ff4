extern crate gl;

use gl::types::*;
use std::mem;
use std::os::raw::c_void;

pub enum AttributeKind {
	Byte,
	Short,
	Int,
	UnsignedByte,
	UnsignedShort,
	UnsignedInt,
	Half,
	Float,
	Double,
	Fixed,
}

impl AttributeKind {
	pub fn to_raw_enum(&self) -> GLenum {
		match self {
			AttributeKind::Byte => gl::BYTE,
			AttributeKind::Short => gl::SHORT,
			AttributeKind::Int => gl::INT,
			AttributeKind::UnsignedByte => gl::UNSIGNED_BYTE,
			AttributeKind::UnsignedShort => gl::UNSIGNED_SHORT,
			AttributeKind::UnsignedInt => gl::UNSIGNED_INT,
			AttributeKind::Half => gl::HALF_FLOAT,
			AttributeKind::Float => gl::FLOAT,
			AttributeKind::Double => gl::DOUBLE,
			AttributeKind::Fixed => gl::FIXED,
		}
	}

	pub fn size(&self) -> usize {
		match self {
			AttributeKind::Byte => mem::size_of::<GLchar>(),
			AttributeKind::Short => mem::size_of::<GLshort>(),
			AttributeKind::Int => mem::size_of::<GLint>(),
			AttributeKind::UnsignedByte => mem::size_of::<GLbyte>(),
			AttributeKind::UnsignedShort => mem::size_of::<GLushort>(),
			AttributeKind::UnsignedInt => mem::size_of::<GLuint>(),
			AttributeKind::Half => mem::size_of::<GLhalf>(),
			AttributeKind::Float => mem::size_of::<GLfloat>(),
			AttributeKind::Double => mem::size_of::<GLdouble>(),
			AttributeKind::Fixed => mem::size_of::<GLfixed>(),
		}
	}
}

pub struct VBO {
	handle: GLuint,
	vertex_count: usize,
}

impl VBO {
	pub fn make<T>(vertices: &Vec<T>, attrs: &Vec<(bool, usize, AttributeKind)>) -> VBO {
		let stride = mem::size_of::<T>() as GLsizei;
		let total_size = (vertices.len() * stride as usize) as GLsizeiptr;
		let root_ptr = &vertices[0] as *const T as *const c_void;

		let handle = unsafe {
			let mut vbo = 0;
			let mut vao = 0;
			let mut offset = 0;

			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);

			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(gl::ARRAY_BUFFER, total_size, root_ptr, gl::STATIC_DRAW);

			for (i, attr) in attrs.iter().enumerate() {
				let offset_ptr = offset as *const c_void;
				let normalized = match attr.0 {
					false => gl::FALSE,
					true => gl::TRUE,
				};

				gl::EnableVertexAttribArray(i as u32);
				gl::VertexAttribPointer(
					i as GLuint,
					attr.1 as GLint,
					attr.2.to_raw_enum(),
					normalized,
					stride,
					offset_ptr,
				);

				offset += attr.2.size() * attr.1;
			}

			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);

			vao as GLuint
		};

		VBO {
			handle,
			vertex_count: vertices.len(),
		}
	}

	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.handle);
			gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.vertex_count as i32);
		};
	}
}

impl Drop for VBO {
	fn drop(&mut self) {
		unsafe { gl::DeleteVertexArrays(1, &self.handle) };
		self.handle = 0;
	}
}

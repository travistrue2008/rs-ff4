extern crate gl;

use gl::types::*;
use std::ffi::CString;
use std::ptr;

pub enum ShaderKind {
  Vertex,
  Fragment,
}

fn resolve_kind (kind: ShaderKind) -> gl::GLenum {
	match kind {
		Vertex => gl::VERTEX_SHADER,
		Fragment => gl::FRAGMENT_SHADER,
	}
}

pub fn compile_shader(kind: ShaderKind, src: &str) -> Result<GLuint, &str> {
	unsafe {
		let stage = resolve_kind(kind);
		let handle = gl::CreateShader(stage);
		let src_c_str = CString::new(src.as_bytes()).unwrap();

		gl::ShaderSource(handle, 1, &src_c_str.as_ptr(), ptr::null());
		gl::CompileShader(handle);

		let mut success = gl::FALSE as GLint;
		let mut log = Vec::with_capacity(512);

		infoLog.set_len(511);
		gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut success);

		if success != gl::TRUE as GLint {
			let log_ptr = log.as_mut_ptr() as *mut GLchar;

			gl::GetShaderInfoLog(handle, 512, ptr::null_mut(), log_ptr);
			Err(str::from_utf8(&log).unwrap())
		}

		Ok(handle)
	}
}

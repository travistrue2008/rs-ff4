extern crate gl;

use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

enum ShaderKind {
  Vertex,
  Fragment,
}

struct Stage {
	handle: GLuint,
}

impl Stage {
	fn make(kind: ShaderKind, src: &str) -> Result<GLuint, String> {
		unsafe {
			let src_c_str = CString::new(src.as_bytes()).unwrap();
			let handle = gl::CreateShader(match kind {
				ShaderKind::Vertex => gl::VERTEX_SHADER,
				ShaderKind::Fragment => gl::FRAGMENT_SHADER,
			});
	
			gl::ShaderSource(handle, 1, &src_c_str.as_ptr(), ptr::null());
			gl::CompileShader(handle);
	
			let mut success = gl::FALSE as GLint;
			let mut log = Vec::with_capacity(512);
	
			log.set_len(511);
			gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut success);
	
			if success != gl::TRUE as GLint {
				let log_ptr = log.as_mut_ptr() as *mut GLchar;
	
				gl::GetShaderInfoLog(handle, 512, ptr::null_mut(), log_ptr);

				Err(str::from_utf8(&log).unwrap().into())
			} else {
				Ok(handle)
			}
		}
	}
}

impl Drop for Stage {
	fn drop(&mut self) {
		println!("Stage::drop()");

		unsafe { gl::DeleteShader(self.handle) };
		self.handle = 0;
	}
}

pub struct Program {
	handle: GLuint,
}

impl Program {
	pub fn make(vert_src: &str, frag_src: &str) -> Result<GLuint, String> {
		let vert_handle = Stage::make(ShaderKind::Vertex, vert_src)?;
		let frag_handle = Stage::make(ShaderKind::Fragment, frag_src)?;

		unsafe {
			let handle = gl::CreateProgram();
			gl::AttachShader(handle, vert_handle);
			gl::AttachShader(handle, frag_handle);
			gl::LinkProgram(handle);
	
			let mut success = gl::FALSE as GLint;
			gl::GetProgramiv(handle, gl::LINK_STATUS, &mut success);

			if success != gl::TRUE as GLint {
				let mut log = Vec::with_capacity(512);
				log.set_len(511);

				let log_ptr = log.as_mut_ptr() as *mut GLchar;
	
				gl::GetShaderInfoLog(handle, 512, ptr::null_mut(), log_ptr);

				Err(str::from_utf8(&log).unwrap().into())
			} else {
				Ok(handle)
			}
		}
	}

	pub fn handle(&self) -> GLuint {
		self.handle
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		println!("Program::drop()");

		unsafe { gl::DeleteProgram(self.handle) };
		self.handle = 0;
	}
}

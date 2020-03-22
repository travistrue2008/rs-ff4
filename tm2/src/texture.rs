extern crate gl;
extern crate glfw;

use super::tm2;

use gl::types::*;
use std::os::raw::c_void;

pub struct Texture {
	mipmaps: bool,
	handle: GLuint,
}

impl Texture {
	pub fn make(image: &tm2::Image, mipmaps: bool) -> Texture {
		let mut handle = 0 as GLuint;
		let data = image.to_raw();
	
		unsafe {
			gl::GenTextures(1, &mut handle);
			gl::BindTexture(gl::TEXTURE_2D, handle);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
	
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA as i32,
				image.width() as i32,
				image.height() as i32,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				&data[0] as *const u8 as *const c_void,
			);

			if mipmaps {
				gl::GenerateMipmap(gl::TEXTURE_2D);
			}

			Texture {
				mipmaps,
				handle,
			}
		}
	}

	pub fn bind(&self, unit: GLenum) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + unit);
			gl::BindTexture(gl::TEXTURE_2D, self.handle);
		}
	}
}

impl Drop for Texture {
	fn drop(&mut self) {
		unsafe { gl::DeleteTextures(1, &self.handle) };
		self.handle = 0;
	}
}

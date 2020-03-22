extern crate gl;
extern crate glfw;
extern crate image;

mod tm2;
mod shader;
mod texture;

use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;
use shader::Shader;
use texture::Texture;

use glfw::{
	Action,
	Context,
	Key,
	Glfw,
	Window,
	WindowEvent,
	WindowHint,
	WindowMode,
};

const SRC_VERTEX: &str = r#"
	#version 330 core

	layout (location = 0) in vec2 a_pos;
	layout (location = 1) in vec2 a_coord;

	out vec2 v_coord;

	void main() {
		v_coord = a_coord;
		gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0);
	}
"#;

const SRC_FRAGMENT: &str = r#"
  #version 330 core

  uniform sampler2D u_tex;

  in vec2 v_coord;

  out vec4 out_color;

  void main() {
	//   out_color = vec4(v_coord.x, 0.0, v_coord.y, 1.0);

	out_color = texture(u_tex, v_coord);
  }
"#;

fn make_vao() -> GLuint {
	const VERTICES: [f32;16] = [
		 1.0,  1.0, 1.0, 0.0,
		-1.0,  1.0, 0.0, 0.0,
		-1.0, -1.0, 0.0, 1.0,
		 1.0, -1.0, 1.0, 1.0,
	];

	let (mut vbo, mut vao) = (0, 0);
	let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
	let vert_ptr = &VERTICES[0] as *const f32 as *const c_void;
	let buffer_size = (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;

	unsafe {
		gl::GenVertexArrays(1, &mut vao);
		gl::GenBuffers(1, &mut vbo);
		gl::BindVertexArray(vao);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(gl::ARRAY_BUFFER, buffer_size, vert_ptr, gl::STATIC_DRAW);

		gl::EnableVertexAttribArray(0);
		gl::EnableVertexAttribArray(1);

		gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
		gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const c_void);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);

		vao as GLuint
	}
}

fn draw(shader: &Shader, texture: &Texture, vao: GLuint) {
	unsafe {
		shader.bind();
		texture.bind(0);

		gl::BindVertexArray(vao);
		gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
	}
}

fn init_glfw() -> Glfw {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	glfw.window_hint(WindowHint::ContextVersion(4, 1));
	glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
	glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

	glfw
}

fn init_window(glfw: &Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
	let (mut window, events) = glfw.create_window(
		128,
		126,
		"TM2 Viewer",
		WindowMode::Windowed,
	).expect("Failed to create GLFW window.");

	window.make_current();
	window.set_key_polling(true);
	window.set_framebuffer_size_polling(true);

	(window, events)
}

fn init_gl(window: &mut Window) {
	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	unsafe {
		gl::ClearColor(0.2, 0.3, 0.3, 1.0);
		gl::ActiveTexture(gl::TEXTURE0);
	}
}

fn process_events(window: &mut Window, events: &Receiver<(f64, WindowEvent)>) {
	for (_, event) in glfw::flush_messages(&events) {
		match event {
			WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
				window.set_should_close(true)
			},
			WindowEvent::FramebufferSize(width, height) => {
				unsafe {
					gl::Viewport(0, 0, width, height);
				}
			},
			_ => {},
		}
	}
}

fn process_frame() {
	unsafe {
		gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
	}
}

fn main() {
	let mut glfw = init_glfw();
	let (mut window, events) = init_window(&glfw);

	init_gl(&mut window);

	let shader = Shader::make(SRC_VERTEX, SRC_FRAGMENT).unwrap();
	let vao = make_vao();

	let data = tm2::load("./assets/cave1_c_base.tm2").unwrap();
	let image = data.get_image(0);
	let texture =  Texture::make(&image, false);

	window.set_size(image.width() as i32, image.height() as i32);
	while !window.should_close() {
		process_events(&mut window, &events);
		process_frame();
		draw(&shader, &texture, vao);

		window.swap_buffers();
		glfw.poll_events();
	}

	unsafe {
		gl::DeleteVertexArrays(1, &vao);
	}
}

extern crate gl;
extern crate glfw;
extern crate image;

mod tm2;
mod shader;
mod texture;
mod vbo;

use shader::Shader;
use std::sync::mpsc::Receiver;
use std::vec;
use texture::Texture;
use vbo::{AttributeKind, VBO};

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

const COLOR_KEY: [u8;3] = [0, 255, 0];

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
	out_color = texture(u_tex, v_coord);
  }
"#;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
	x: f32,
	y: f32,
	u: f32,
	v: f32,
}

impl Vertex {
	pub fn attrs() -> Vec<(bool, usize, AttributeKind)> {
		vec![
			(false, 2, AttributeKind::Float),
			(false, 2, AttributeKind::Float),
		]
	}

	pub fn new() -> Vertex {
		Vertex { x: 0.0, y: 0.0, u: 0.0, v: 0.0 }
	}

	pub fn make(x: f32, y: f32, u: f32, v: f32) -> Vertex {
		Vertex { x, y, u, v }
	}
}

fn draw(shader: &Shader, texture: &Texture, vbo: &VBO) {
	unsafe {
		shader.bind();
		texture.bind(0);
		vbo.draw();
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
		gl::Enable(gl::BLEND);
		gl::ClearColor(0.2, 0.3, 0.3, 1.0);
		gl::ActiveTexture(gl::TEXTURE0);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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
	let vbo = VBO::make(&vec![
		Vertex::make( 1.0,  1.0, 1.0, 0.0),
		Vertex::make(-1.0,  1.0, 0.0, 0.0),
		Vertex::make(-1.0, -1.0, 0.0, 1.0),
		Vertex::make( 1.0, -1.0, 1.0, 1.0),
	], &Vertex::attrs());

	let data = tm2::load("./assets/cave1_c_base.tm2").unwrap();
	let image = data.to_raw(0, Some(COLOR_KEY));
	let texture = Texture::make(&image, false);

	window.set_size(image.width() as i32, image.height() as i32);
	while !window.should_close() {
		process_events(&mut window, &events);
		process_frame();

		draw(&shader, &texture, &vbo);

		window.swap_buffers();
		glfw.poll_events();
	}
}

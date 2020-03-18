extern crate gl;
extern crate glfw;
extern crate image;

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

use std::path::Path;
use std::sync::mpsc::Receiver;

mod defs;

const SHADER_VERTEX_SRC: &str = r#"
	#version 330 core

	layout (location = 0) in vec3 a_pos;

	void main() {
		gl_Position = vec4(a_pos.x, a_pos.y, a_pos.z, 1.0);
	}
"#;

const SHADER_FRAGMENT_SRC: &str = r#"
  #version 330 core

  out vec4 out_color;

  void main() {
	  out_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

// fn initDefs () {
// 	let definitions: defs::Definitions = defs::load();
// }

fn create_icon() -> Vec<glfw::PixelImage> {
	let icon_path = Path::new("./assets/icon.png");
	let icon_image = image::open(icon_path).unwrap().to_rgba();
	let mut icon_pixels = Vec::new();

	for pixel in icon_image.pixels() {
		icon_pixels.push(
			(pixel[3] as u32) << 24 |
			(pixel[2] as u32) << 16 |
			(pixel[1] as u32) << 8 |
			(pixel[0] as u32)
		);
	}

	vec![glfw::PixelImage {
		width: icon_image.width(),
		height: icon_image.height(),
		pixels: icon_pixels,
	}]
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
		640,
		480,
		"Final Fantasy IV",
		WindowMode::Windowed,
	).expect("Failed to create GLFW window.");

	window.make_current();
	window.set_key_polling(true);
	window.set_framebuffer_size_polling(true);
	window.set_icon_from_pixels(create_icon());

	(window, events)
}

fn init_gl(window: &mut Window) {
	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	unsafe {
		gl::ClearColor(0.2, 0.3, 0.3, 1.0);
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

	while !window.should_close() {
		process_events(&mut window, &events);
		process_frame();
		window.swap_buffers();
		glfw.poll_events();
	}
}

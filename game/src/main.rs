mod common;
mod error;
mod tilemap;

use lazy_static::lazy_static;
use std::cell::Cell;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Instant;
use std::vec;
use vex::Matrix4;

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

lazy_static! {
    static ref proj_mat: Matrix4 = Matrix4::ortho(0.0, 480.0, 0.0, 272.0, 0.0, 1000.0);
}

fn create_icon() -> Vec<glfw::PixelImage> {
    let icon_path = Path::new("./assets/icon.png");
    let icon_image = image::open(icon_path).unwrap().to_rgba();
    let mut icon_pixels = Vec::new();

    for pixel in icon_image.pixels() {
        icon_pixels.push(
            (pixel[3] as u32) << 24
                | (pixel[2] as u32) << 16
                | (pixel[1] as u32) << 8
                | (pixel[0] as u32),
        );
    }

    vec![glfw::PixelImage {
        width: icon_image.width(),
        height: icon_image.height(),
        pixels: icon_pixels,
    }]
}

fn init_glfw() -> Glfw {
    let mut glfw = glfw::init(Some(glfw::Callback {
		f: error_callback,
		data: Cell::new(0),
    })).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 1));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    glfw
}

fn init_window(glfw: &Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
    let (mut window, events) = glfw
        .create_window(480, 272, "Final Fantasy IV", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_icon_from_pixels(create_icon());

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

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
	println!("GLFW error ({}): {}", error_count.get(), description);
	error_count.set(error_count.get() + 1);
}

fn process_events(window: &mut Window, events: &Receiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            _ => {}
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

    let path = Path::new("./assets/tileset");
    let mut tilemap = tilemap::load(path, "castle1_baron_castle_01.cn2", "castle1_b").unwrap();

    let start_time = Instant::now();
    while !window.should_close() {
        let elapsed = start_time.elapsed().as_secs_f32();

        process_events(&mut window, &events);
        process_frame();

        tilemap.update(elapsed);
        tilemap.render();

        window.swap_buffers();
        glfw.poll_events();
    }
}

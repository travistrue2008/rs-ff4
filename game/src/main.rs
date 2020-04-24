mod common;
mod defs;
mod error;
mod tileset;

use std::cell::Cell;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::vec;
use tim2;

use gl_toolkit::{
    SHADER_TEXTURE,
    PrimitiveKind,
	Texture,
	VBO,
	TextureVertex,
};

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

// fn initDefs () {
// 	let definitions: defs::Definitions = defs::load();
// }

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
        .create_window(640, 480, "Final Fantasy IV", WindowMode::Windowed)
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
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
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

fn draw(texture: &Texture, vbo: &VBO) {
	SHADER_TEXTURE.bind();
	texture.bind(0);
	vbo.draw();
}

fn main() {
    let mut glfw = init_glfw();
    let (mut window, events) = init_window(&glfw);

    init_gl(&mut window);

    let vbo = VBO::make(PrimitiveKind::TriangleFan, &vec![
        TextureVertex::make( 1.0,  1.0, 1.0, 0.0),
        TextureVertex::make(-1.0,  1.0, 0.0, 0.0),
        TextureVertex::make(-1.0, -1.0, 0.0, 1.0),
        TextureVertex::make( 1.0, -1.0, 1.0, 1.0),
    ], None);

    let image = tim2::load("./assets/tileset/castle1_b_base.tm2").unwrap();
    let frame = image.get_frame(0);
    let pixels = frame.to_raw(None);
    let texture = Texture::make(&pixels, frame.width(), frame.height(), false).unwrap();
    let tileset = tileset::load("./assets/tileset/castle1_baron_castle_01.cn2").unwrap();

    println!("tileset_dims: <{}, {}>", tileset.width, tileset.height);
    for i in 0..(tileset.cells.len() / tileset.width) {
        let start_index = i * tileset.width;
        let end_index = start_index + tileset.width;
        let slice = &tileset.cells[start_index..end_index];
        let indices: Vec::<usize> = slice.to_vec().iter().map(|cell| { cell.v1 }).collect();

        println!("{:?}", &indices);
    }

    window.set_size(frame.width() as i32, frame.height() as i32);
    while !window.should_close() {
        process_events(&mut window, &events);
        process_frame();

        draw(&texture, &vbo);

        window.swap_buffers();
        glfw.poll_events();
    }
}

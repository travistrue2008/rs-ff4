mod common;
mod error;
mod graphics;
mod tilemap;

use std::path::Path;
use std::time::Instant;
use tilemap::Tilemap;

use crate::graphics::{
	Camera,
	Core as GraphicsCore,
};

use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::{Icon, Window, WindowBuilder},
};

const WINDOW_SIZE: winit::dpi::LogicalSize<f64> =
	winit::dpi::LogicalSize::new(480.0, 272.0);

const CLEAR_COLOR: wgpu::Color = wgpu::Color {
	r: 0.1,
	g: 0.2,
	b: 0.3,
	a: 1.0,
};

fn create_icon() -> Icon {
	let path = Path::new("./assets/icon.png");

	let image = image::open(path)
		.expect("Failed to load icon")
		.to_rgba8();

	let (width, height) = image.dimensions();
	let buffer = image.into_raw();

	Icon::from_rgba(buffer, width, height).unwrap()
}

pub struct App {
	last_time: f32,
	start_time: Instant,
	graphics_core: GraphicsCore,
	camera: Camera,
	level: Tilemap,
}

impl App {
	async fn new(window: &mut Window) -> Self {
		let path = Path::new("./assets/tilemap");
		let graphics_core = GraphicsCore::new(window).await;

		let layout = &graphics_core.pipeline().get_bind_group_layout(0);
		let mut camera = Camera::new(graphics_core.device(), layout);

		let level = tilemap::load(
			&graphics_core,
			path,
			"castle1_baron_castle_01.cn2",
			"castle1_b",
		).unwrap();

		println!("dims<{}, {}>", level.get_px_width(), level.get_px_height());

		camera.ortho(480.0, 272.0);

		window.set_inner_size(
			winit::dpi::LogicalSize::new(
				level.get_px_width() / window.scale_factor() as f32,
				level.get_px_height() / window.scale_factor() as f32,
			)
		);

		Self {
			last_time: 0.0,
			start_time: Instant::now(),
			graphics_core,
			camera,
			level,
		}
	}

	fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
		self.graphics_core.resize(size);
		self.camera.ortho(size.width as f32, size.height as f32);
	}

	fn input(&mut self, _event: &WindowEvent) -> bool {
		false
	}

	fn update(&mut self) {
		let elapsed_time = self.start_time.elapsed().as_secs_f32();
		// let frame_time = elapsed_time - self.last_time;

		// println!("frame_time: {}", frame_time);
		// self.last_time = elapsed_time;

		self.camera.update(&self.graphics_core.queue());
		self.level.update(&self.graphics_core.queue(), elapsed_time);
	}

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.graphics_core.surface().get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self.graphics_core.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				depth_stencil_attachment: None,
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						store: true,
						load: wgpu::LoadOp::Clear(CLEAR_COLOR),
					},
				})],
			});

			render_pass.set_pipeline(self.graphics_core.pipeline());
			self.level.render(&mut render_pass, &self.camera);
		}
	
		self.graphics_core.queue().submit(std::iter::once(encoder.finish()));
		output.present();
	
		Ok(())
	}
}

#[tokio::main]
async fn main() {
	env_logger::init();

	let event_loop = EventLoop::new();

	let mut window = WindowBuilder::new()
		.with_title("Final Fantasy IV")
		.with_window_icon(Some(create_icon()))
		.with_resizable(false)
		.with_inner_size(WINDOW_SIZE)
		.build(&event_loop).unwrap();

	let mut screen_size = winit::dpi::PhysicalSize::new(0, 0);
	let mut app = App::new(&mut window).await;

	event_loop.run(move |event, _, control_flow| match event {
		Event::WindowEvent {
			ref event,
			window_id,
		} if window_id == window.id() => if !app.input(event) {
			match event {
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit
				},
				WindowEvent::Resized(physical_size) => {
					screen_size = *physical_size;

					app.resize(screen_size);
				},
				WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
					screen_size = **new_inner_size;

					app.resize(screen_size);
				},
				_ => {},
			}
		},
		Event::RedrawRequested(window_id) if window_id == window.id() => {
			app.update();

			match app.render() {
				Ok(_) => {}
				Err(wgpu::SurfaceError::Lost) => app.resize(screen_size),
				Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
				Err(e) => eprintln!("{:?}", e),
			};
		},
		Event::MainEventsCleared => {
			window.request_redraw();
		},
		_ => {},
	});
}

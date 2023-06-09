use wgpu::*;
use winit::window::Window;

use super::pipelines;

pub struct Core {
	config: SurfaceConfiguration,
	device: Device,
	queue: Queue,
	surface: Surface,
	pipeline: RenderPipeline,
}

impl Core {
	fn build_instance() -> Instance {
		Instance::new(InstanceDescriptor {
			backends: Backends::all(),
			dx12_shader_compiler: Default::default(),
		})
	}

	fn build_surface(window: &Window, instance: &Instance) -> Surface {
		unsafe {
			instance.create_surface(&window)
		}.unwrap()
	}

	async fn build_adapter(instance: &Instance, surface: &Surface) -> Adapter {
		instance.request_adapter(
			&RequestAdapterOptions {
				power_preference: PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			},
		).await.unwrap()
	}

	async fn build_device_queue(adapter: &Adapter) -> (Device, Queue) {
		adapter.request_device(
			&DeviceDescriptor {
				label: Some("Device"),
				features: Features::empty(),
				limits: Limits::default(),
			},
			None,
		).await.unwrap()
	}

	fn build_surface_config(
		window_size: winit::dpi::PhysicalSize<u32>,
		surface: &Surface,
		adapter: &Adapter,
		device: &Device,
	) -> SurfaceConfiguration {
		let surface_caps = surface.get_capabilities(&adapter);

		let surface_format = surface_caps.formats
			.iter()
			.copied()
			.find(|f| f.describe().srgb)
			.unwrap_or(surface_caps.formats[0]);

		let config = SurfaceConfiguration {
			usage: TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: window_size.width,
			height: window_size.height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
		};

		surface.configure(&device, &config);

		config
	}

	pub async fn new(window: &Window) -> Self {
		let window_size = window.inner_size();
		let instance = Self::build_instance();
		let surface = Self::build_surface(&window, &instance);
		let adapter = Self::build_adapter(&instance, &surface).await;
		let (device, queue) = Self::build_device_queue(&adapter).await;
		let config = Self::build_surface_config(window_size, &surface, &adapter, &device);
		let pipeline = pipelines::textured::build(&device, &config);

		println!("adapter info: {:#?}", adapter.get_info());

		Self {
			config,
			device,
			queue,
			surface,
			pipeline,
		}
	}

	pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
		if size.width == 0 || size.height == 0 {
			panic!("Resize must not be zero<{}, {}>", size.width, size.height)
		}

		self.config.width = size.width;
		self.config.height = size.height;
		self.surface.configure(&self.device, &self.config);
	}

	pub fn config(&self) -> &SurfaceConfiguration {
		&self.config
	}

	pub fn device(&self) -> &Device {
		&self.device
	}

	pub fn queue(&self) -> &Queue {
		&self.queue
	}

	pub fn surface(&self) -> &Surface {
		&self.surface
	}

	pub fn pipeline(&self) -> &RenderPipeline {
		&self.pipeline
	}
}

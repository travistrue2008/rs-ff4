use wgpu::*;
use winit::window::Window;

use super::pipeline;

pub struct Core {
	config: SurfaceConfiguration,
	device: Device,
	queue: Queue,
	surface: Surface,
	layout: BindGroupLayout,
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
				power_preference: PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			},
		).await.unwrap()
	}

	async fn build_device_queue(adapter: &Adapter) -> (Device, Queue) {
		adapter.request_device(
			&DeviceDescriptor {
				features: Features::empty(),
				limits: Limits::default(),
				label: None,
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

	fn build_textured_bind_group(device: &Device) -> BindGroupLayout {
		device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: None,
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					count: None,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						multisampled: false,
						view_dimension: TextureViewDimension::D2,
						sample_type: TextureSampleType::Float { filterable: true },
					},
				},
				BindGroupLayoutEntry {
					binding: 1,
					count: None,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
				},
			],
		})
	}

	pub async fn new(window: &Window) -> Self {
		let window_size = window.inner_size();
		let instance = Self::build_instance();
		let surface = Self::build_surface(&window, &instance);
		let adapter = Self::build_adapter(&instance, &surface).await;
		let (device, queue) = Self::build_device_queue(&adapter).await;
		let config = Self::build_surface_config(window_size, &surface, &adapter, &device);
		let layout = Self::build_textured_bind_group(&device);
		let pipeline = pipeline::build(&device, &config, &layout);

		Self {
			config,
			device,
			queue,
			surface,
			layout,
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

	pub fn layout(&self) -> &BindGroupLayout {
		&self.layout
	}

	pub fn pipeline(&self) -> &RenderPipeline {
		&self.pipeline
	}
}

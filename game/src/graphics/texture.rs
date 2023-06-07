use tim2::Image;
use wgpu::*;

pub struct Texture {
	width: u32,
	height: u32,
	handle: wgpu::Texture,
	sampler: Sampler,
	view: TextureView,
}

impl Texture {
	pub fn new(device: &Device, width: u32, height: u32) -> Texture {
		let size = Extent3d {
			width,
			height,
			depth_or_array_layers: 1,
		};

		let handle = device.create_texture(
			&TextureDescriptor {
				label: None,
				size,
				mip_level_count: 1,
				sample_count: 1,
				dimension: TextureDimension::D2,
				format: TextureFormat::Rgba8UnormSrgb,
				usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
				view_formats: &[],
			}
		);

		let view = handle.create_view(&TextureViewDescriptor::default());
	
		let sampler = device.create_sampler(&SamplerDescriptor {
			address_mode_u: AddressMode::ClampToEdge,
			address_mode_v: AddressMode::ClampToEdge,
			address_mode_w: AddressMode::ClampToEdge,
			mag_filter: FilterMode::Linear,
			min_filter: FilterMode::Nearest,
			mipmap_filter: FilterMode::Nearest,
			..Default::default()
		});
	
		Texture {
			width,
			height,
			handle,
			sampler,
			view,
		}
	}

	pub fn from_image(device: &Device, queue: &Queue, image: &Image) -> Texture {
		let frame = image.get_frame(0);
		let buffer = frame.to_raw(None);
		let width = frame.width() as u32;
		let height = frame.height() as u32;
		let result = Self::new(device, width, height);

		result.write(queue, &buffer, Origin3d::ZERO, width, height);
		result
	}

	pub fn write(&self, queue: &Queue, buffer: &Vec<u8>, origin: Origin3d, width: u32, height: u32) {
		let size = Extent3d {
			width,
			height,
			depth_or_array_layers: 1,
		};

		queue.write_texture(
			ImageCopyTexture {
				texture: &self.handle,
				mip_level: 0,
				origin,
				aspect: TextureAspect::All,
			},
			&buffer,
			ImageDataLayout {
				offset: 0,
				bytes_per_row: std::num::NonZeroU32::new(4 * width),
				rows_per_image: std::num::NonZeroU32::new(height),
			},
			size,
		)
	}

	pub fn create_bind_group(&self, device: &Device, layout: &BindGroupLayout) -> BindGroup {
		device.create_bind_group(
			&BindGroupDescriptor {
				label: None,
				layout,
				entries: &[
					BindGroupEntry {
						binding: 0,
						resource: BindingResource::TextureView(&self.view),
					},
					BindGroupEntry {
						binding: 1,
						resource: BindingResource::Sampler(&self.sampler),
					}
				],
			}
		)
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn handle(&self) -> &wgpu::Texture {
		&self.handle
	}

	pub fn sampler(&self) -> &Sampler {
		&self.sampler
	}

	pub fn view(&self) -> &TextureView {
		&self.view
	}
}

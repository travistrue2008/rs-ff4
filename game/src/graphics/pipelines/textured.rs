use wgpu::*;

use super::super::vertex::*;

fn build_camera_group_layout(device: &Device) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("Textured Pipeline"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				count: None,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: None,
				},
			}
		],
	})
}

fn build_texture_group_layout(device: &Device) -> BindGroupLayout {
	device.create_bind_group_layout(&BindGroupLayoutDescriptor {
		label: Some("Textured Pipeline"),
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

fn build_layout(device: &Device) -> PipelineLayout {
	device.create_pipeline_layout(&PipelineLayoutDescriptor {
		label: Some("Textured Pipeline"),
		push_constant_ranges: &[],
		bind_group_layouts: &[
			&build_camera_group_layout(device),
			&build_texture_group_layout(device),
		],
	})
}

pub fn build(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline {
	let source = include_wgsl!("../shaders/textured.wgsl");
	let shader = device.create_shader_module(source);
	let layout = build_layout(device);

	device.create_render_pipeline(&RenderPipelineDescriptor {
		label: Some("Textured Pipeline"),
		layout: Some(&layout),
		vertex: VertexState {
			module: &shader,
			entry_point: "vs_main",
			buffers: &[TextureVertex::layout()],
		}, 
		fragment: Some(FragmentState {
			module: &shader,
			entry_point: "fs_main",
			targets: &[Some(ColorTargetState {
				format: config.format,
				blend: Some(BlendState::ALPHA_BLENDING),
				write_mask: ColorWrites::ALL,
			})],
		}),
		primitive: PrimitiveState {
			topology: PrimitiveTopology::TriangleList,
			strip_index_format: None,
			front_face: FrontFace::Ccw,
			cull_mode: Some(Face::Back),
			polygon_mode: PolygonMode::Fill,
			unclipped_depth: false,
			conservative: false,
		},
		depth_stencil: None,
		multisample: MultisampleState {
			count: 1,
			mask: !0,
			alpha_to_coverage_enabled: false,
		},
		multiview: None,
	})
}

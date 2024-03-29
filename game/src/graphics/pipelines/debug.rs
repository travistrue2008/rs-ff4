use wgpu::*;

use super::super::vertex::*;

fn build_camera_group_layout(device: &Device) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("Debug Pipeline"),
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

fn build_layout(device: &Device) -> PipelineLayout {
	device.create_pipeline_layout(&PipelineLayoutDescriptor {
		label: Some("Debug Pipeline"),
		push_constant_ranges: &[],
		bind_group_layouts: &[&build_camera_group_layout(device)],
	})
}

pub fn build(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline {
	let source = include_wgsl!("../shaders/debug.wgsl");
	let shader = device.create_shader_module(source);
	let layout = build_layout(device);

	device.create_render_pipeline(&RenderPipelineDescriptor {
		label: Some("Debug Pipeline"),
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
				blend: Some(BlendState::REPLACE),
				write_mask: ColorWrites::ALL,
			})],
		}),
		primitive: PrimitiveState {
			topology: PrimitiveTopology::TriangleStrip,
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

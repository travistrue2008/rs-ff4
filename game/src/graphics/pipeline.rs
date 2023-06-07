use wgpu::*;

use super::vertex::*;

pub fn build(device: &Device, config: &SurfaceConfiguration, bind_group_layout: &BindGroupLayout) -> RenderPipeline {
	let source = include_wgsl!("./shaders/textured.wgsl");
	let shader = device.create_shader_module(source);

	let render_pipeline_layout =
		device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("Textured Render Pipeline Layout"),
			push_constant_ranges: &[],
			bind_group_layouts: &[bind_group_layout],
		});

	device.create_render_pipeline(&RenderPipelineDescriptor {
		label: Some("Textured Render Pipeline"),
		layout: Some(&render_pipeline_layout),
		vertex: VertexState {
			module: &shader,
			entry_point: "vs_main",
			buffers: &[TextureVertex::get_layout()],
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

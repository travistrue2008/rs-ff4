@group(0) @binding(0)
var<uniform> mat_pvm: mat4x4<f32>;

@group(1) @binding(0)
var diffuse_t: texture_2d<f32>;

@group(1) @binding(1)
var diffuse_s: sampler;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) uv: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
	return VertexOutput(
		mat_pvm * vec4<f32>(model.position, 1.0),
		model.uv,
	);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return textureSample(diffuse_t, diffuse_s, in.uv);
}

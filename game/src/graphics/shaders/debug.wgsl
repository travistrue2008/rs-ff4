struct VertexInput {
	@location(0) position: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
}


@group(0) @binding(0)
var<uniform> mat_pvm: mat4x4<f32>;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
	return VertexOutput(mat_pvm * vec4<f32>(model.position, 1.0));
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
	return vec4(1.0, 0.0, 1.0, 1.0)
}

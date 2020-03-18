extern crate gl;

enum ShaderKind {
  Vertex,
  Fragment,
}

fn resolve_kind (kind: ShaderKind) -> gl::GLenum {
	match kind {
		Vertex => gl::VERTEX_SHADER,
		Fragment => gl::FRAGMENT_SHADER,
	}
}

fn compile_shader(kind: ShaderKind, src: &str) {
	unsafe {
	}
}

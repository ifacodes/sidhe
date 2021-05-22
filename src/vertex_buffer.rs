#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    color: [f32; 3],
    opacity: f32,
}

impl Vertex {
    fn new(position: [f32; 3], uv: [f32; 2], color: [f32; 3], opacity: f32) -> Self {
        Self {
            position,
            uv,
            color,
            opacity,
        }
    }
}

struct VertexBuffer {
    pub size: u32,
    buffer: wgpu::Buffer,
}

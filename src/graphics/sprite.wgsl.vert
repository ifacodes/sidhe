// Vertex

struct VertexInput {
    [[location(0)]] vertex: vec4<f32>;
};

struct VertexOutput {
    [[location(0)]] coords: vec2<f32>;
};
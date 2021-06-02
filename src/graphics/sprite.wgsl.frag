// Fragment
[[block]] struct Uniforms {
    spriteColor: vec3<f32>;
};
[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;
[[group(0), binding(1)]] var sampler: sampler;
[[group(0), binding(2)]] var texture: texture_2d<f32>;

[[stage(fragment)]]
fn main([[location(0)]] coords: vec2<f32>,) -> [[location(0)]] vec4<f32> {

    return vec4<f32>(uniforms.spriteColor, 1.0) * textureSample(texture, sampler, coords);
}
use anyhow::*;
use image::GenericImageView;

use super::Device;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn load(device: &Device, image: &image::DynamicImage, label: Option<&str>) -> Result<Self> {
        let rgba = image.as_rgba8().unwrap();
        let dimensions = image.dimensions();

        device.create_texture(rgba, dimensions, label)
    }
}

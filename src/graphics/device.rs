use anyhow::*;
use std::num::NonZeroU32;

use image::RgbaImage;
use wgpu::{util::DeviceExt, BindGroupLayout, PushConstantRange};

use crate::graphics::Swapchain;

use super::texture::Texture;

pub struct Device {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}

impl Device {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, surface: wgpu::Surface) -> Self {
        Self {
            device,
            queue,
            surface,
        }
    }
    #[allow(dead_code)]
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }
    #[allow(dead_code)]
    pub fn wgpu_device_mut(&mut self) -> &mut wgpu::Device {
        &mut self.device
    }

    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Command Encoder"),
            })
    }
    pub fn create_swap_chain(&self, width: u32, height: u32) -> wgpu::SwapChain {
        let desc = Swapchain::descriptor(width, height);
        self.device.create_swap_chain(&self.surface, &desc)
    }
    pub fn create_shader(&self) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Shader Module"),
                flags: wgpu::ShaderFlags::all(),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
            })
    }
    pub fn create_buffer(
        &self,
        name: &str,
        usage: wgpu::BufferUsage,
        contents: &[u8],
    ) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(name),
                contents,
                usage,
            })
    }

    pub fn create_texture(
        &self,
        rgba: &RgbaImage,
        dimensions: (u32, u32),
        label: Option<&str>,
    ) -> Result<Texture> {
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        Ok(Texture {
            texture,
            view,
            sampler,
        })
    }

    pub fn submit<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = wgpu::CommandBuffer>,
    {
        self.queue.submit(command_buffers);
    }
}

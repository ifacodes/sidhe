use anyhow::*;
use raw_window_handle::HasRawWindowHandle;

use crate::graphics::{Device, Pipeline, Swapchain, VertexBuffer};

use super::Vertex;

pub struct GraphicSystem {
    pub device: Device,
}

impl GraphicSystem {
    pub async fn new<W>(window: &W) -> Result<Self>
    where
        W: HasRawWindowHandle,
    {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        Ok(Self {
            device: Device::new(device, queue, surface),
        })
    }

    pub fn swap_chain(&self, size: winit::dpi::PhysicalSize<u32>) -> Swapchain {
        Swapchain {
            size,
            swap_chain: self.device.create_swap_chain(size.width, size.height),
        }
    }
    pub fn command_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_encoder()
    }
    pub fn pipeline_layout(
        &self,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        push_constant_ranges: &[wgpu::PushConstantRange],
    ) -> wgpu::PipelineLayout {
        self.device
            .create_pipeline_layout(bind_group_layouts, push_constant_ranges)
    }
    pub fn pipeline(
        &self,
        layout: &wgpu::PipelineLayout,
        vertex: wgpu::VertexState,
        fragment: wgpu::FragmentState,
    ) -> Pipeline {
        let desc = Pipeline::pipeline_descriptor(layout, vertex, fragment);
        Pipeline {
            pipeline: self.device.create_pipeline(&desc),
        }
    }
    pub fn shader(&self) -> wgpu::ShaderModule {
        self.device.create_shader()
    }
    pub fn buffer(&self, name: &str, usage: wgpu::BufferUsage, contents: &[u8]) -> VertexBuffer {
        VertexBuffer {
            buffer: self.device.create_buffer(name, usage, contents),
        }
    }
    pub fn submit<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = wgpu::CommandBuffer>,
    {
        self.device.submit(command_buffers);
    }
}

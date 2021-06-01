use wgpu::{util::DeviceExt, BindGroupLayout, PushConstantRange};

use crate::graphics::{Pipeline, Swapchain};

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

    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }

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
    pub fn create_pipeline_layout(
        &self,
        bind_group_layouts: &[&BindGroupLayout],
        push_constant_ranges: &[PushConstantRange],
    ) -> wgpu::PipelineLayout {
        self.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges,
            })
    }
    pub fn create_pipeline(&self, desc: &wgpu::RenderPipelineDescriptor) -> wgpu::RenderPipeline {
        self.device.create_render_pipeline(desc)
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
    pub fn submit<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = wgpu::CommandBuffer>,
    {
        self.queue.submit(command_buffers);
    }
}

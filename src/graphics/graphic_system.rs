use crate::graphics::{Buffer, Device, Pipeline, Swapchain, Vertex};
use anyhow::*;
use winit::dpi::PhysicalSize;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5],
        coord: [0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        coord: [0.5, 0.5],
    },
    Vertex {
        position: [0.0, -0.5],
        coord: [0.0, 0.5],
    },
    Vertex {
        position: [0.0, 0.5],
        coord: [0.0, -0.5],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub struct GraphicSystem {
    pub device: Device,
    pub swap_chain: Swapchain,
}

impl GraphicSystem {
    pub async fn new(window: &winit::window::Window) -> Result<Self> {
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
        let size = window.inner_size();
        let swap_chain =
            device.create_swap_chain(&surface, &Swapchain::descriptor(size.width, size.height));
        Ok(Self {
            device: Device::new(device, queue, surface),
            swap_chain: Swapchain { size, swap_chain },
        })
    }
    pub fn size(&self) -> PhysicalSize<u32> {
        self.swap_chain.size()
    }

    pub fn swap_chain(&mut self, new_size: PhysicalSize<u32>) {
        self.swap_chain = Swapchain {
            size: new_size,
            swap_chain: self.device.wgpu_device().create_swap_chain(
                self.device.wgpu_surface(),
                &Swapchain::descriptor(new_size.width, new_size.height),
            ),
        }
    }
    pub fn vertex_buffer(&self, name: &str, usage: wgpu::BufferUsage) -> Buffer {
        Buffer {
            buffer: self
                .device
                .create_buffer(name, usage, bytemuck::cast_slice(VERTICES)),
            size: VERTICES.len(),
        }
    }
    pub fn index_buffer(&self, name: &str, usage: wgpu::BufferUsage) -> Buffer {
        Buffer {
            buffer: self
                .device
                .create_buffer(name, usage, bytemuck::cast_slice(INDICES)),
            size: INDICES.len(),
        }
    }
    pub fn draw(&self, vertices: &Buffer, indices: &Buffer) -> Result<(), wgpu::SwapChainError> {
        // Get the latest frame from the swapchain
        let frame = self.swap_chain.get_current_frame().unwrap();
        // Create a command encoder
        let mut encoder = self.device.create_encoder();
        // create shader module
        // TODO: make this changable
        let shader = self.device.create_shader();

        let vertex = wgpu::VertexState {
            module: &shader,
            entry_point: "main",
            buffers: &[Buffer::desc()],
        };
        let fragment = wgpu::FragmentState {
            module: &shader,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: self.swap_chain.format(),
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrite::all(),
            }],
        };

        let pipeline_layout =
            self.device
                .wgpu_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let render_pipeline_desc =
            Pipeline::pipeline_descriptor(&pipeline_layout, vertex, fragment);
        let render_pipeline = self
            .device
            .wgpu_device()
            .create_render_pipeline(&render_pipeline_desc);
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.87,
                            g: 0.69,
                            b: 0.79,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_vertex_buffer(0, vertices.slice(..));
            render_pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.size() as u32, 0, 0..1);
        }

        self.device.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

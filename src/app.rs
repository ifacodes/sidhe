use crate::graphics::{GraphicSystem, Swapchain, Vertex, VertexBuffer};
use wgpu::SwapChainError;
use winit::{dpi::PhysicalSize, window::Window};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, 0.0],
        coord: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        coord: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.25, -0.5, 0.0],
        coord: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.25, 0.5, 0.0],
        coord: [0.0, 1.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
pub struct App {
    graphic_system: GraphicSystem,
    swap_chain: Swapchain,
    vertex_buffer: VertexBuffer,
    index_buffer: VertexBuffer,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let graphic_system = GraphicSystem::new(window).await.unwrap();
        let size = window.inner_size();
        let swap_chain = graphic_system.swap_chain(size);
        let vertex_buffer = graphic_system.buffer(
            "Vertex Buffer",
            wgpu::BufferUsage::VERTEX,
            bytemuck::cast_slice(VERTICES),
        );
        let index_buffer = graphic_system.buffer(
            "Index Buffer",
            wgpu::BufferUsage::INDEX,
            bytemuck::cast_slice(INDICES),
        );
        Self {
            graphic_system,
            swap_chain,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.swap_chain.size
    }

    // TODO: This should eventually be in a InputSystem
    #[allow(dead_code)]
    pub fn input(&mut self) -> bool {
        todo!()
    }
    #[allow(dead_code)]
    pub fn update(&mut self) {
        todo!()
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.swap_chain = self.graphic_system.swap_chain(new_size);
    }

    pub fn render(&mut self) -> Result<(), SwapChainError> {
        let frame = self.swap_chain.get_current_frame().unwrap();
        let mut encoder = self.graphic_system.command_encoder();

        let shader = self.graphic_system.shader();
        let vertex = wgpu::VertexState {
            module: &shader,
            entry_point: "main",
            buffers: &[VertexBuffer::desc()],
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

        let pipeline_layout = self.graphic_system.pipeline_layout(&[], &[]);
        let render_pipeline = self
            .graphic_system
            .pipeline(&pipeline_layout, vertex, fragment);

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

            render_pass.set_pipeline(&render_pipeline.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        self.graphic_system
            .submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

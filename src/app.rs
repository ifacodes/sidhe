use crate::graphics::{GraphicSystem, Swapchain};
use wgpu::{SwapChainDescriptor, SwapChainError};
use winit::{dpi::PhysicalSize, window::Window};

pub struct App {
    graphic_system: GraphicSystem,
    swap_chain: Swapchain,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let graphic_system = GraphicSystem::new(window).await.unwrap();
        let size = window.inner_size();
        let swap_chain = graphic_system.swap_chain(size);
        Self {
            graphic_system,
            swap_chain,
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.swap_chain.size
    }

    // TODO: This should eventually be in a InputSystem
    pub fn input(&mut self) -> bool {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.swap_chain = self.graphic_system.swap_chain(new_size);
    }

    pub fn render(&mut self) -> Result<(), SwapChainError> {
        let frame = self.swap_chain.get_current_frame().unwrap();
        let mut encoder = self.graphic_system.command_encoder();

        {
            let _renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        }

        self.graphic_system
            .submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

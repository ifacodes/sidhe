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
        let swap_chain = graphic_system.swap_chain(size.width, size.height);
        Self {
            graphic_system,
            swap_chain,
        }
    }

    // TODO: This should eventually be in a InputSystem
    pub fn input(&mut self) -> bool {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.swap_chain = self
            .graphic_system
            .swap_chain(new_size.width, new_size.height);
    }

    pub fn render(&mut self) -> Result<(), SwapChainError> {
        let frame = self.swap_chain.get_current_frame();
        let mut encoder = self.graphic_system.command_encoder();

        Ok(())
    }
}

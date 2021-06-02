use crate::graphics::{Buffer, GraphicSystem};
use wgpu::SwapChainError;
use winit::{dpi::PhysicalSize, window::Window};
pub struct App {
    graphic_system: GraphicSystem,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let graphic_system = GraphicSystem::new(window).await.unwrap();
        let vertex_buffer =
            graphic_system.vertex_buffer("Vertex Buffer", wgpu::BufferUsage::VERTEX);
        let index_buffer = graphic_system.index_buffer("Index Buffer", wgpu::BufferUsage::INDEX);
        Self {
            graphic_system,
            vertex_buffer,
            index_buffer,
        }
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
    pub fn size(&self) -> PhysicalSize<u32> {
        self.graphic_system.size()
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.graphic_system.swap_chain(new_size);
    }

    pub fn render(&self) -> Result<(), SwapChainError> {
        self.graphic_system
            .draw(&self.vertex_buffer, &self.index_buffer)
    }
}

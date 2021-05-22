use crate::graphics::GraphicSystem;
use raw_window_handle::HasRawWindowHandle;

pub struct App {
    graphic_system: GraphicSystem,
}

impl App {
    pub async fn new<W>(window: &W) -> Self
    where
        W: HasRawWindowHandle,
    {
        let graphic_system = GraphicSystem::new(window).await.unwrap();
        Self { graphic_system }
    }

    pub fn input(&mut self) -> bool {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self) {
        todo!()
    }
}

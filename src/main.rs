mod app;
mod game_window;
mod graphics;
mod vertex_buffer;

use app::App;
use futures::executor::block_on;
use game_window::GameWindow;
use winit::event_loop::ControlFlow;

fn main() {
    env_logger::init();
    let window = GameWindow::new("Test");
    let app = block_on(App::new(window.window()));

    window.run(app, move |app, control_flow| {
        // calculate delta time stuff,
        // call app update
        // call app render
        //app.update();
        match app.render() {
            Ok(_) => {}
            Err(wgpu::SwapChainError::Lost) => app.resize(app.size()),
            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        }
    });
}

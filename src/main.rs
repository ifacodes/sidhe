mod app;
mod game_window;
mod graphics;
mod vertex_buffer;

use app::App;
use futures::executor::block_on;
use game_window::GameWindow;

fn main() {
    env_logger::init();
    let window = GameWindow::new("Test");
    let app = block_on(App::new(window.window()));

    window.run(app, move |app| {
        // calculate delta time stuff,
        // call app update
        // call app render
    });
}

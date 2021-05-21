mod app;
mod game_window;
mod graphics;

use futures::executor::block_on;
use game_window::GameWindow;

fn main() {
    env_logger::init();
    let window = GameWindow::new("Test");
    let app = app::App {};

    window.run(app, move |app| {});

    //let mut state = block_on(State::new(&window));
}

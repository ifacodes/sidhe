use crate::app;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct GameWindow {
    window: winit::window::Window,
    event_loop: Option<EventLoop<()>>,
    window_size: winit::dpi::PhysicalSize<u32>,
}

impl GameWindow {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        let window_size = window.inner_size();
        let event_loop = Some(event_loop);
        Self {
            window,
            event_loop,
            window_size,
        }
    }

    pub fn run<F>(mut self, mut app: app::App, mut game_loop: F)
    where
        F: 'static + FnMut(&mut app::App),
    {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, controlflow| {
                *controlflow = ControlFlow::Poll;

                match event {
                    Event::RedrawRequested(_) => game_loop(&mut app),
                    Event::MainEventsCleared => self.window.request_redraw(),
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == self.window.id() => match event {
                        WindowEvent::CloseRequested => {
                            *controlflow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(physical_size) => {
                            self.window_size = *physical_size;
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.window_size = **new_inner_size;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            })
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_size
    }
}

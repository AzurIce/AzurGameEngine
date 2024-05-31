use std::sync::Arc;
use std::time::Instant;

use azurge_core::Core;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    core: Option<Core>,
    last_tick: Option<Instant>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        let window = Arc::new(window);

        let core = Core::new(window.clone());
        // window.request_redraw();

        self.window = Some(window);
        self.core = Some(core)
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let time_delta = self.last_tick.unwrap_or(Instant::now()).elapsed();
                self.last_tick = Some(Instant::now());
                self.core.as_mut().unwrap().tick(time_delta);
                self.core.as_ref().unwrap().render();
            }
            WindowEvent::Resized(size) => {
                self.core.as_mut().unwrap().handle_resize(size);
                // self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { .. }
            | WindowEvent::CursorMoved { .. }
            | WindowEvent::MouseInput { .. } => {
                self.core.as_mut().unwrap().handle_input_event(event);
                // self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("run app error");
}

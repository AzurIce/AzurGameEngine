use std::{sync::Arc, time::Duration};

use input::InputSystem;
use render::{camera::Camera, Renderer};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub mod entity;
pub mod input;
pub mod render;
pub mod runtime;

pub struct Core {
    renderer: render::Renderer,
    input_system: InputSystem,
    camera: Camera,
}

impl Core {
    pub fn new(window: Arc<Window>) -> Self {
        let renderer = Renderer::new(window);
        let camera = Camera::new(
            glam::Vec3 {
                x: -3.0,
                y: 0.0,
                z: -3.0,
            },
            90.0,
            16.0 / 9.0,
            0.1,
            100.0,
        );

        let input_system = InputSystem::default();
        Core { renderer, camera, input_system }
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.handle_resize(size);
        self.camera.set_ratio(size.width as f32 / size.height as f32);
    }
    
    pub fn handle_input_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.input_system.handle_keyboard_input(event);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input_system.handle_cursor_moved(position);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.input_system.handle_mouse_input(button, state)
            }
            _ => (),
        }
    
    }
}

impl Core {
    pub fn tick(&mut self, delta_time: Duration) {
        self.camera.tick(delta_time, self.input_system.game_command(), self.input_system.cursor_delta());
        self.input_system.reset_cursor_delta();
    }

    pub fn render(&self) {
        self.renderer.render(&self.camera);
    }
}

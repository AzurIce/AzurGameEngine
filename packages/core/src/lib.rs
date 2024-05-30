use std::{cell::RefCell, sync::Arc};

use render::{pipeline::CubePipeline, Renderer};
use winit::{dpi::PhysicalSize, window::Window};

pub mod entity;
pub mod render;
pub mod runtime;

pub struct Core {
    renderer: render::Renderer,
}

impl Core {
    pub fn new(window: Arc<Window>) -> Self {
        let renderer = Renderer::new(window);
        Core { renderer }
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.handle_resize(size);
    }
}

impl Core {
    pub fn render(&self) {
        self.renderer.render();
    }
}

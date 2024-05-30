use std::sync::Arc;

use wgpu::Surface;
use winit::{dpi::PhysicalSize, window::Window};

pub mod pipeline;
pub mod render;
pub mod runtime;

pub struct Core {
    ctx: render::wgpu_context::WgpuContext,
}

impl Core {
    pub fn new(window: Arc<Window>) -> Self {
        let ctx = pollster::block_on(render::wgpu_context::WgpuContext::new(window));
        Core { ctx }
    }

    pub fn handle_resize(&mut self, mut size: PhysicalSize<u32>) {
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        println!("[core]: handle_resize: {size:?}");
        self.ctx.update_size(size);
        // self.ctx.handle_resize();
    }
}

impl Core {
    pub fn render(&self) {
        println!("[core]: render");
        self.ctx.render().unwrap();
    }
}

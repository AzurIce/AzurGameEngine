pub mod pipeline;
pub mod wgpu_context;
pub mod camera;
pub mod scene;
pub mod primitive;
// pub mod resource;

use std::{cell::RefCell, sync::Arc};

use camera::Camera;
use pipeline::CubePipeline;
use scene::Scene;
use wgpu_context::WgpuContext;
use winit::{dpi::PhysicalSize, window::Window};

use primitive::{mesh::Mesh, CUBE_VERTEX, CUBE_VERTEX_INDEX};

pub struct Renderer {
    ctx: WgpuContext,
    pub pipeline: RefCell<CubePipeline>,
    scene: Scene
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let ctx = pollster::block_on(WgpuContext::new(window));
        let pipeline = CubePipeline::new(&ctx);
        let pipeline = RefCell::new(pipeline);
        let mut scene = Scene::new();
        scene.add_mesh(Mesh::new(&ctx, &CUBE_VERTEX, &CUBE_VERTEX_INDEX));
        Self { ctx, pipeline, scene }
    }

    pub fn handle_resize(&mut self, mut size: PhysicalSize<u32>) {
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        println!("[core]: handle_resize: {size:?}");
        self.ctx.update_size(size);
    }

    pub fn render(&self, camera: &Camera) {
        // println!("[core]: render");
        let output = self.ctx.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.pipeline.borrow_mut().render(&self.ctx, &view, camera, &self.scene);
        output.present();
    }
}

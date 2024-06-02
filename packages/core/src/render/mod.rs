pub mod camera;
pub mod pipeline;
pub mod primitive;
pub mod resource;
pub mod scene;
pub mod wgpu_context;

use std::sync::Arc;

use camera::Camera;
use pipeline::CubePipeline;
use primitive::entity::cube::Cube;
use resource::Resource;
use scene::Scene;
use wgpu_context::WgpuContext;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer {
    ctx: Arc<WgpuContext>,
    resource: Resource,
    // pub pipeline: RefCell<Box<dyn Pipeline>>,
    scene: Scene,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let ctx = pollster::block_on(WgpuContext::new(window));
        let ctx = Arc::new(ctx);

        let mut resource = Resource::new(ctx.clone());
        resource.init();
        // let pipeline = CubePipeline::new(&ctx);
        // let pipeline = HelloTrianglePipeline::new(&ctx);
        let mut scene = Scene::new();
        scene.add_render_object(Arc::new(resource.create_render_object::<Cube>(
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 0.0].into(),
        )));
        scene.add_render_object(Arc::new(resource.create_render_object::<Cube>(
            [3.0, 3.0, 3.0].into(),
            [0.5, 1.0, 2.0].into(),
            [1.0, 2.0, 3.0].into(),
        )));
        scene.add_render_object(Arc::new(resource.create_render_object::<Cube>(
            [3.0, 0.0, 3.0].into(),
            [0.5, 0.0, 3.14].into(),
            [1.0, 2.0, 3.0].into(),
        )));

        Self {
            ctx,
            resource,
            // pipeline: RefCell::new(Box::new(pipeline)),
            scene,
        }
    }

    pub fn handle_resize(&mut self, mut size: PhysicalSize<u32>) {
        println!("[core/renderer]: handle_resize: {size:?}");
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        self.ctx.update_surface_size(size);
    }

    pub fn render(&self, camera: &Camera) {
        // println!("[core]: render");
        let output = self.ctx.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.resource
            .get_pipeline::<CubePipeline>()
            .unwrap()
            .render(&self.ctx, &view, camera, &self.scene, &self.resource);
        // self.pipeline
        //     .borrow_mut()
        //     .render(&self.ctx, &view, camera, &self.scene);
        output.present();
    }
}

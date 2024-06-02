use std::sync::Arc;

use super::primitive::Renderable;

#[derive(Default)]
pub struct Scene {
    render_objects: Vec<Arc<dyn Renderable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { render_objects: Vec::new() }
    }

    pub fn render_objects(&self) -> &Vec<Arc<dyn Renderable>> {
        &self.render_objects
    }

    pub fn add_render_object(&mut self, render_object: Arc<dyn Renderable>) {
        self.render_objects.push(render_object);
    }
}

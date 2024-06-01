use std::sync::Arc;

use super::primitive::Render;

#[derive(Default)]
pub struct Scene {
    meshes: Vec<Arc<dyn Render>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { meshes: Vec::new() }
    }

    pub fn meshes(&self) -> &Vec<Arc<dyn Render>> {
        &self.meshes
    }

    pub fn add_mesh(&mut self, mesh: Arc<dyn Render>) {
        self.meshes.push(mesh);
    }
}

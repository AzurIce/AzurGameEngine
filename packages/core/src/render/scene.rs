#[derive(Default)]
pub struct Scene {
    meshes: Vec<String>,
}

impl Scene {
    pub fn new() -> Self {
        Self { meshes: Vec::new() }
    }

    pub fn meshes(&self) -> &Vec<String> {
        &self.meshes
    }

    pub fn add_mesh(&mut self, mesh: String) {
        self.meshes.push(mesh);
    }
}

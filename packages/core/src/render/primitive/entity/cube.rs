use std::sync::Arc;

use crate::render::{primitive::Render, resource::Resource};

pub struct Cube {
    mesh: Arc<dyn Render>,
    position: glam::Vec3,
    model: glam::Mat4,
}

impl Cube {
    pub fn new(
        resource: &Resource,
        position: glam::Vec3,
        rotation: glam::Vec3,
        scale: glam::Vec3,
    ) -> Self {
        let scale_matrix = glam::Mat4::from_scale(scale);

        let rotation_matrix_x = glam::Mat4::from_rotation_x(rotation.x);
        let rotation_matrix_y = glam::Mat4::from_rotation_y(rotation.y);
        let rotation_matrix_z = glam::Mat4::from_rotation_z(rotation.z);
        let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

        let translation_matrix = glam::Mat4::from_translation(position);

        let model = translation_matrix * rotation_matrix * scale_matrix;
        Self {
            mesh: resource.get_mesh("cube").unwrap(),
            position,
            model,
        }
    }
}

impl Render for Cube {
    fn model_matrix(&self) -> glam::Mat4 {
        self.model
    }

    fn vertex_buf(&self) -> &wgpu::Buffer {
        self.mesh.vertex_buf()
    }
    fn index_buf(&self) -> &wgpu::Buffer {
        self.mesh.index_buf()
    }
    fn vertex_cnt(&self) -> usize {
        self.mesh.vertex_cnt()
    }
}

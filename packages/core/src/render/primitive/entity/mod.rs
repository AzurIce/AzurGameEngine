use std::sync::Arc;

use crate::render::resource::{RenderResource, Resource};

use super::Renderable;

pub mod cube;


pub struct RenderObject {
    resource: Arc<RenderResource>,
    position: glam::Vec3,
    model: glam::Mat4,
}

impl Renderable for RenderObject {
    fn index_buf(&self) -> &wgpu::Buffer {
        &self.resource.index_buf
    }
    fn vertex_buf(&self) -> &wgpu::Buffer {
        &self.resource.vertex_buf
    }
    fn vertex_cnt(&self) -> usize {
        self.resource.vertex_cnt
    }
    
    fn model_matrix(&self) -> glam::Mat4 {
        self.model
    }

}

impl RenderObject {
    pub fn new(
        resource: Arc<RenderResource>,
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
            resource,
            position,
            model,
        }
    }
}

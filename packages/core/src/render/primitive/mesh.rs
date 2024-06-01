use wgpu::{util::DeviceExt, Buffer};

use crate::render::{resource::Resource, wgpu_context::WgpuContext};

use super::{Render, Vertex};

pub struct Mesh {
    vertex_cnt: usize,
    vertex_buf: Buffer,
    index_buf: Buffer,
}

impl Mesh {
    pub fn new(context: &WgpuContext, vertex_arr: &[Vertex], index_arr: &[u16]) -> Self {
        let vertex_cnt = index_arr.len();
        let vertex_buf = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex_arr),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        let index_buf = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(index_arr),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });
        Self {
            vertex_cnt,
            vertex_buf,
            index_buf,
        }
    }
}

impl Render for Mesh {
    fn vertex_buf(&self) -> &Buffer {
        &self.vertex_buf
    }
    fn index_buf(&self) -> &Buffer {
        &self.index_buf
    }
    fn vertex_cnt(&self) -> usize {
        self.vertex_cnt
    }
    fn model_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale(glam::Vec3::new(1.0, 1.0, 1.0))
    }
}

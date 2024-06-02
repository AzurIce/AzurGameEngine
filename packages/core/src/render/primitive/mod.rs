pub mod mesh;
pub mod entity;

use bytemuck::{Pod, Zeroable};
use wgpu::Buffer;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

pub const fn vertex(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        _tex_coord: [tc[0] as f32, tc[1] as f32],
    }
}

pub trait RenderData {
    fn identifier() -> &'static str;
    fn vertex_data() -> &'static [Vertex];
    fn index_data() -> &'static [u16];
}

pub trait Renderable {
    fn vertex_buf(&self) -> &Buffer;
    fn index_buf(&self) -> &Buffer;
    fn vertex_cnt(&self) -> usize;

    fn model_matrix(&self) -> glam::Mat4;
}

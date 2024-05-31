pub mod cube_pipeline;
pub mod hello_triangle_pipeline;

use crate::render::wgpu_context::WgpuContext;

pub use cube_pipeline::CubePipeline;
pub use hello_triangle_pipeline::HelloTrianglePipeline;

use super::{camera::Camera, scene::Scene};

pub trait Pipeline {
    fn new(context: &WgpuContext) -> Self where Self: Sized;

    fn render(
        &mut self,
        context: &WgpuContext,
        view: &wgpu::TextureView,
        camera: &Camera,
        scene: &Scene,
    );
}

// pub enum PipelineResources {
//     AzurPipeline,
// }

// impl Pipeline for PipelineResources {
//     fn create(&self, context: &WgpuContext) -> RenderPipeline {
//         match self {
//             PipelineResources::AzurPipeline => HelloTrianglePipeline.create(context),
//         }
//     }
// }

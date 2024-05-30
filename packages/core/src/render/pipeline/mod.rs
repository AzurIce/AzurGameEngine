pub mod hello_triangle_pipeline;
pub mod cube_pipeline;

use crate::render::wgpu_context::WgpuContext;
use wgpu::RenderPipeline;

pub use hello_triangle_pipeline::HelloTrianglePipeline;
pub use cube_pipeline::CubePipeline;

pub trait Pipeline {
    fn create(context: &WgpuContext) -> Box<RenderPipeline>;
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
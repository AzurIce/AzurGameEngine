mod pipeline_resource;

use std::{any::TypeId, collections::HashMap, sync::Arc};

use pipeline_resource::{pipeline_resource, PipelineResource};
use wgpu::RenderPipeline;

use super::{
    pipeline::{HelloTrianglePipeline, Pipeline},
    wgpu_context::WgpuContext,
};

// pub enum Pipeline {
//     HelloTrianglePipeline(HelloTrianglePipeline),
// }

#[derive(Default)]
pub struct Resource {
    pipelines: HashMap<TypeId, Box<dyn PipelineResource>>,
    // pipeline:
}

impl Resource {
    pub fn init(&mut self, context: Arc<WgpuContext>) {
        self.pipelines.insert(
            TypeId::of::<HelloTrianglePipeline>(),
            pipeline_resource::<HelloTrianglePipeline>(context),
        );
    }
    // pub fn get_pipeline<T: Pipeline>(&self) -> Box<dyn PipelineResource> {
        // self.pipelines.get(&TypeId::of::<T>())
    // }
}

use std::sync::Arc;

use wgpu::RenderPipeline;

use crate::render::{pipeline::Pipeline, wgpu_context::WgpuContext};

#[cfg(test)]
mod test {
    use std::any::Any;

    use crate::render::pipeline::{HelloTrianglePipeline, CubePipeline};

    use super::PipelineWrapper;

    #[test]
    fn f() {
        // let a = PipelineWrapper::<HelloTrianglePipeline>::new();
        // println!("{:?}", a.type_id());
        // let a = PipelineWrapper::<HelloTrianglePipeline2>::new();
        // println!("{:?}", a.type_id());
    }
}

pub trait PipelineResource {
    fn reload(&mut self);
    fn pipeline(&mut self) -> Arc<Box<RenderPipeline>>;
}

pub fn pipeline_resource<T: Pipeline + 'static>(context: Arc<WgpuContext>) -> Box<dyn PipelineResource> {
    Box::new(PipelineWrapper::<T>::new(context))
}

pub struct PipelineWrapper<T: Pipeline> {
    context: Arc<WgpuContext>,
    pipeline: Option<Arc<Box<RenderPipeline>>>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Pipeline> PipelineWrapper<T> {
    pub fn new(context: Arc<WgpuContext>) -> Self {
        Self {
            context,
            pipeline: None,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn init(&mut self, context: &WgpuContext) {
        self.pipeline = Some(Arc::new(T::create(context)));
    }
}

impl<T: Pipeline> PipelineResource for PipelineWrapper<T> {
    fn reload(&mut self) {
        self.init(&self.context.clone())
    }
    fn pipeline(&mut self) -> Arc<Box<RenderPipeline>> {
        if self.pipeline.is_none() {
            self.init(&self.context.clone())
        }
        self.pipeline.clone().unwrap()
    }
}

impl<T: Pipeline> Pipeline for PipelineWrapper<T> {
    fn create(context: &WgpuContext) -> Box<RenderPipeline> {
        T::create(context)
    }
}
use super::{
    pipeline::{CubePipeline, HelloTrianglePipeline, Pipeline},
    primitive::{mesh::Mesh, Render, CUBE_VERTEX, CUBE_VERTEX_INDEX},
    wgpu_context::WgpuContext,
};
use std::{any::TypeId, collections::HashMap, sync::Arc};

pub trait Reload {
    fn reload(&mut self);
}
pub struct Resource {
    context: Arc<WgpuContext>,
    pipelines: HashMap<TypeId, Box<dyn Pipeline>>,
    meshes: HashMap<String, Arc<dyn Render>>,
}

impl Resource {
    pub fn new(context: Arc<WgpuContext>) -> Self {
        Self {
            context,
            pipelines: HashMap::new(),
            meshes: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        self.pipelines.insert(
            TypeId::of::<HelloTrianglePipeline>(),
            Box::new(HelloTrianglePipeline::new(&self.context)),
        );
        self.pipelines.insert(
            TypeId::of::<CubePipeline>(),
            Box::new(CubePipeline::new(&self.context)),
        );

        self.meshes.insert(
            "cube".to_string(),
            Arc::new(Mesh::new(&self.context, &CUBE_VERTEX, CUBE_VERTEX_INDEX)),
        );
    }

    pub fn get_pipeline<T: Pipeline + 'static>(&self) -> Option<&dyn Pipeline> {
        self.pipelines.get(&TypeId::of::<T>()).map(|b| &**b)
    }
    pub fn get_mesh(&self, name: &str) -> Option<Arc<dyn Render>> {
        self.meshes.get(name).map(|m| m.clone())
    }
}

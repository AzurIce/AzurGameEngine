use wgpu::{naga::proc::index, util::DeviceExt};

use super::{
    pipeline::{CubePipeline, HelloTrianglePipeline, Pipeline},
    primitive::{entity::{cube::Cube, RenderObject}, mesh::Mesh, RenderData, Renderable},
    wgpu_context::WgpuContext,
};
use std::{any::TypeId, collections::HashMap, sync::Arc};

pub trait Reload {
    fn reload(&mut self);
}
pub struct Resource {
    context: Arc<WgpuContext>,
    pipelines: HashMap<TypeId, Box<dyn Pipeline>>,
    meshes: HashMap<String, Arc<dyn Renderable>>,
    render_resources: HashMap<String, Arc<RenderResource>>,
}

pub struct RenderResource {
    pub vertex_cnt: usize,
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
}

impl Resource {
    pub fn new(context: Arc<WgpuContext>) -> Self {
        Self {
            context,
            pipelines: HashMap::new(),
            meshes: HashMap::new(),
            render_resources: HashMap::new(),
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

        self.load_render_resource::<Cube>();
    }

    pub fn load_render_resource<M: RenderData>(&mut self) -> Arc<RenderResource> {
        let (vertex_data, index_data) = (M::vertex_data(), M::index_data());

        let vertex_cnt = index_data.len();
        let vertex_buf =
            self.context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertex_data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        let index_buf = self
            .context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(index_data),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });
        let render_resource = RenderResource {
            vertex_cnt: vertex_cnt,
            vertex_buf,
            index_buf,
        };

        let render_resource = Arc::new(render_resource);
        self.render_resources
            .insert(M::identifier().to_string(), render_resource.clone());
        render_resource
    }

    pub fn create_render_object<M: RenderData>(
        &mut self,
        position: glam::Vec3,
        rotation: glam::Vec3,
        scale: glam::Vec3,
    ) -> RenderObject {
        let render_resource = match self.render_resources.get(M::identifier()) {
            Some(resource) => resource.clone(),
            None => self.load_render_resource::<M>(),
        };

        RenderObject::new(render_resource, position, rotation, scale)
    }

    pub fn get_pipeline<T: Pipeline + 'static>(&self) -> Option<&dyn Pipeline> {
        self.pipelines.get(&TypeId::of::<T>()).map(|b| &**b)
    }
    pub fn get_mesh(&self, name: &str) -> Option<Arc<dyn Renderable>> {
        self.meshes.get(name).map(|m| m.clone())
    }
}

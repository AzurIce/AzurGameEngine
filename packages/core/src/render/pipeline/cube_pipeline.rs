// use std::f32::consts;

use wgpu::{
    BindGroup, BindGroupLayoutDescriptor, PipelineCompilationOptions,
    PipelineLayoutDescriptor, RenderPipeline,
};

use super::Pipeline;
use crate::render::{
    camera::Camera, primitive::{mesh::Mesh, Render, Vertex}, resource::Resource, scene::Scene, wgpu_context::WgpuContext
};

fn create_texels(size: usize) -> Vec<u8> {
    (0..size * size)
        .map(|id| {
            // get high five for recognizing this ;)
            let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
            let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
            let (mut x, mut y, mut count) = (cx, cy, 0);
            while count < 0xFF && x * x + y * y < 4.0 {
                let old_x = x;
                x = x * x - y * y + cx;
                y = 2.0 * old_x * y + cy;
                count += 1;
            }
            count
        })
        .collect()
}

pub struct CubePipeline {
    pipeline: RenderPipeline,
    bind_group: BindGroup,

    // vertex_buf: wgpu::Buffer,
    // index_buf: wgpu::Buffer,
    uniform_buf: wgpu::Buffer,
}

// fn generate_matrix(aspect_ratio: f32) -> glam::Mat4 {
//     let projection = glam::Mat4::perspective_rh(consts::FRAC_PI_4, aspect_ratio, 1.0, 10.0);
//     let view = glam::Mat4::look_at_rh(
//         glam::Vec3::new(1.5f32, -5.0, 3.0),
//         glam::Vec3::ZERO,
//         glam::Vec3::Z,
//     );
//     projection * view
// }

impl Pipeline for CubePipeline {
    fn new(context: &WgpuContext) -> Self {
        // ? Bind group layout
        let bind_group_layout =
            context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Cube Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(64),
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Uint,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                    ],
                });

        // ? Pipeline layout
        let pipeline_layout = context
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout], // ? Bind group layout
                push_constant_ranges: &[],
            });

        // Create texture
        let size = 256u32;
        let texels = create_texels(size as usize);
        let texture_extent = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        };
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        context.queue.write_texture(
            texture.as_image_copy(),
            &texels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(size),
                rows_per_image: None,
            },
            texture_extent,
        );

        // Create other resources
        // let (w, h) = context.get_surface_size();
        // let mx_total = generate_matrix(w as f32 / h as f32);
        // let mx_ref: &[f32; 16] = mx_total.as_ref();
        let uniform_buf = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: 4 * 4 * std::mem::size_of::<f32>() as u64,
            // contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                ],
                label: None,
            });

        // ? Shader module
        let shader_module = context
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/cube_pipeline/shader.wgsl"));

        let vertex_size = std::mem::size_of::<Vertex>();
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 4 * 4,
                    shader_location: 1,
                },
            ],
        };

        let swapchain_capabilities = context.surface.get_capabilities(&context.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout), // ? Pipeline layout
                vertex: wgpu::VertexState {
                    module: &shader_module, // ? Shader module
                    entry_point: "vs_main",
                    buffers: &[vertex_buffer_layout],
                    compilation_options: PipelineCompilationOptions {
                        ..Default::default()
                    },
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module, // ? Shader modyle
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())],
                    compilation_options: PipelineCompilationOptions {
                        ..Default::default()
                    },
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Self {
            pipeline,
            bind_group,
            // vertex_buf,
            // index_buf,
            uniform_buf,
        }
    }

    fn render(
        &self,
        context: &WgpuContext,
        view: &wgpu::TextureView,
        camera: &Camera,
        scene: &Scene,
        resource: &Resource
    ) {
        {
            // TODO: move these things out of [`render`] to optimize performance (maybe add a method called [`update`]?)
            // Camera -> view_projection_mat
            let mx_total = camera.view_projection_mat();
            let mx_ref: &[f32; 16] = mx_total.as_ref();
            context
                .queue
                .write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(mx_ref));
        }

        let render = |renderable: &Box<dyn Render>| {
            let mut encoder = context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rpass.push_debug_group("Prepare data for draw.");
                rpass.set_pipeline(&self.pipeline);
                rpass.set_bind_group(0, &self.bind_group, &[]);
                rpass.set_index_buffer(renderable.index_buf().slice(..), wgpu::IndexFormat::Uint16);
                rpass.set_vertex_buffer(0, renderable.vertex_buf().slice(..));
                rpass.pop_debug_group();
                rpass.insert_debug_marker("Draw!");
                rpass.draw_indexed(0..renderable.vertex_cnt() as u32, 0, 0..1);
                // if let Some(ref pipe) = self.pipeline_wire {
                //     rpass.set_pipeline(pipe);
                //     rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
                // }
            }

            context.queue.submit(Some(encoder.finish()));
        };

        for mesh in scene.meshes() {
            if let Some(mesh) = resource.get_mesh(&mesh) {
                render(mesh);
            }
        }
    }
}

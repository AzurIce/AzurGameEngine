use wgpu::{PipelineCompilationOptions, PipelineLayoutDescriptor, RenderPipeline};

use super::Pipeline;
use crate::render::wgpu_context::WgpuContext;

pub struct HelloTrianglePipeline;

impl Pipeline for HelloTrianglePipeline {
    fn create(context: &WgpuContext) -> Box<RenderPipeline> {
        // ? Pipeline layout
        let pipeline_layout = context
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // ? Shader module
        let shader_module = context
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout), // ? Pipeline layout
                vertex: wgpu::VertexState {
                    module: &shader_module, // ? Shader module
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: PipelineCompilationOptions {
                        ..Default::default()
                    },
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module, // ? Shader modyle
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: context.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: PipelineCompilationOptions {
                        ..Default::default()
                    },
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        Box::new(pipeline)
    }
}

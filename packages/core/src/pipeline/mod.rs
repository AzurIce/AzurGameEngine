use wgpu::{PipelineCompilationOptions, PipelineLayout, RenderPipeline};

use crate::render::wgpu_context::WgpuContext;

pub trait Pipeline {
    fn get(&self, context: &WgpuContext, layout: PipelineLayout) -> RenderPipeline;
}

pub struct AzurPipeline;

impl Pipeline for AzurPipeline {
    fn get(&self, context: &WgpuContext, layout: PipelineLayout) -> RenderPipeline {
        let shader_module = context
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: PipelineCompilationOptions {
                        ..Default::default()
                    },
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
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

        pipeline
    }
}

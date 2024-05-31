use wgpu::{PipelineCompilationOptions, PipelineLayoutDescriptor, RenderPipeline};

use super::Pipeline;
use crate::render::wgpu_context::WgpuContext;

pub struct HelloTrianglePipeline {
    pipeline: RenderPipeline,
}

impl Pipeline for HelloTrianglePipeline {
    fn new(context: &WgpuContext) -> Self {
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
            .create_shader_module(wgpu::include_wgsl!("../shaders/hello_triangle_pipeline/shader.wgsl"));

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

        Self { pipeline }
    }
    fn render(
        &mut self,
        context: &WgpuContext,
        view: &wgpu::TextureView,
        _camera: &crate::render::camera::Camera,
        _scene: &crate::render::scene::Scene,
    ) {
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&self.pipeline);
            // render_pass.set_bind_group(0, &fragment_texture_bind_group, &[]);
            // render_pass.set_bind_group(1, &state_bind_group, &[]);
            // render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            // render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // render_pass.draw_indexed(0..3, 0, 0..1);
            render_pass.draw(0..3, 0..1);
        }

        context.queue.submit(Some(encoder.finish()));
    }
}

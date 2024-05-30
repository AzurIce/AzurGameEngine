use std::sync::Arc;

use wgpu::{
    DeviceDescriptor, InstanceDescriptor, PipelineLayoutDescriptor, RenderPipeline,
    RequestAdapterOptions, SurfaceConfiguration,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::pipeline::{AzurPipeline, Pipeline};

pub struct WgpuContext {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    pub pipeline: Option<RenderPipeline>, // pub size: winit::dpi::PhysicalSize<u32>,
                                          // pub egui_dev: EguiDev,
                                          // pub egui_rpass: egui_wgpu_backend::RenderPass,
                                          // shaders: HashMap<&'static str, ShaderModule>,
                                          // _textures: HashMap<&'static str, (Texture, BindGroup, BindGroupLayout)>,
}

impl WgpuContext {
    pub fn update_size(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}

impl WgpuContext {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(InstanceDescriptor {
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let mut context = Self {
            surface,
            device,
            queue,
            config,
            pipeline: None,
        };

        let pipeline = AzurPipeline.get(&context, render_pipeline_layout);
        context.pipeline = Some(pipeline);

        context
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
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
            render_pass.set_pipeline(self.pipeline.as_ref().unwrap());
            // render_pass.set_bind_group(0, &fragment_texture_bind_group, &[]);
            // render_pass.set_bind_group(1, &state_bind_group, &[]);
            // render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            // render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // render_pass.draw_indexed(0..3, 0, 0..1);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));

        output.present();

        Ok(())
    }
}

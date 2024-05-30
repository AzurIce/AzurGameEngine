use std::sync::Arc;

use wgpu::{DeviceDescriptor, InstanceDescriptor, RequestAdapterOptions};
use winit::{dpi::PhysicalSize, window::Window};

pub struct WgpuContext {
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
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

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);
        surface.configure(&device, &config);

        Self {
            surface,
            adapter,
            device,
            queue,
            config,
        }
    }
}

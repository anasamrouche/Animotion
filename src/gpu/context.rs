use std::sync::Arc;

use wgpu::{
    Device, Queue, Surface, SurfaceConfiguration, Texture, TextureFormat, TextureUsages,
    TextureView,
    wgt::{TextureDescriptor, TextureViewDescriptor},
};
use winit::window::Window;

pub struct GPUContext<'a> {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'a>,
    pub surface_config: SurfaceConfiguration,
    pub depth_texture: TextureView,
}

impl<'a> GPUContext<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        Self {
            device,
            queue,
            surface,
            surface_config,
            depth_texture: depth_texture.create_view(&TextureViewDescriptor::default()),
        }
    }
}

use wgpu::*;

use super::Frame;

pub struct GraphicsCtx<'w> {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'w>,
    pub surface_texture_format: TextureFormat,
    pub surface_capabilities: SurfaceCapabilities,

    window_size: (u32, u32),
}

pub struct RenderCtx {
    pub view: TextureView,
    pub encoder: CommandEncoder,

    surface_texture: SurfaceTexture,
}

impl<'w> GraphicsCtx<'w> {
    pub fn new(window_size: (u32, u32), target: impl Into<SurfaceTarget<'w>>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: util::backend_bits_from_env().unwrap_or(Backends::all()),
            ..Default::default()
        });
        let surface = instance
            .create_surface(target)
            .unwrap_or_else(|e| panic!("Could not create graphics surface: {e}"));
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap_or_else(|e| panic!("Could not acquire graphics device: {e}"));

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_texture_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let mut _self = Self {
            device,
            queue,
            surface,
            surface_capabilities,
            surface_texture_format,
            window_size,
        };

        _self.resize(window_size);

        _self
    }

    pub fn resize(&mut self, window_size: (u32, u32)) {
        if window_size.0 > 0 && window_size.1 > 0 {
            self.surface.configure(
                &self.device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: self.surface_texture_format,
                    width: window_size.0,
                    height: window_size.1,
                    present_mode: self.surface_capabilities.present_modes[0],
                    alpha_mode: self.surface_capabilities.alpha_modes[0],
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                },
            );

            self.window_size = window_size;
        }
    }

    pub fn next_frame<'a>(&'a mut self) -> Option<Frame<'a>> {
        let surface_texture = self.surface.get_current_texture().map_err(|e| match e {
            wgpu::SurfaceError::OutOfMemory => {
                panic!("The system is out of memory for rendering!")
            }
            _ => format!("An error occured during surface texture acquisition: {e}"),
        });

        if surface_texture.is_err() {
            return None;
        }
        let surface_texture = surface_texture.unwrap();

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        Some(Frame {
            render: RenderCtx {
                surface_texture,
                encoder,
                view,
            },
            ctx: self,
        })
    }

    pub fn window_size(&self) -> (u32, u32) {
        self.window_size
    }
}

impl<'a> Frame<'a> {
    pub fn present(self) {
        self.ctx
            .queue
            .submit(std::iter::once(self.render.encoder.finish()));
        self.render.surface_texture.present();
    }
}

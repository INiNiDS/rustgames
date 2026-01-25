use std::sync::Arc;
use glam::Vec2;
use wgpu::{Device, Queue, StoreOp, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::graphics::camera::Camera;
use crate::graphics::sprite_renderer::SpriteRenderer;
use crate::graphics::texture::Texture;
use crate::text::{Font, TextSpeed, TextStyle};
use crate::text::text::TextSystem;
use crate::text::typewriter::TypewriterInstance;

pub struct Renderer {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub camera: Camera,
    sprite_renderer: SpriteRenderer,
    text_system: TextSystem,
    text_position: Vec2,
    typewriter_instance: TypewriterInstance
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    experimental_features: Default::default(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                },
            )
            .await
            .expect("Failed to create device");

        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface isn't supported by the adapter.");

        surface.configure(&device, &config);

        let size = window.inner_size();
        let camera = Camera::new(size.width, size.height);

        let sprite_renderer = SpriteRenderer::new(&device, &config);

        let text_system = TextSystem::new(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            camera,
            sprite_renderer,
            text_system,
            text_position: Vec2::ZERO,
            typewriter_instance: TypewriterInstance::new(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.camera.resize(new_size.width, new_size.height);
            self.text_system.resize(new_size.width, new_size.height, &self.queue);
        }
    }

    pub fn draw(&mut self) {
        self.sprite_renderer.update_camera(&self.queue, &self.camera);
        let bytes = include_bytes!("../mistral.png");
        let texture = Texture::from_bytes(&self.device, &self.queue, bytes, None).unwrap();

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );
        if !self.typewriter_instance.is_empty() {
            for typewriter in self.typewriter_instance.get_typewriter_effects() {
                self.text_system.queue_text(&self.device, &self.queue, typewriter, self.text_position.x, self.text_position.y);
            }
        }
         {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                                store: StoreOp::Store,
                            },
                        })
                    ],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

            self.sprite_renderer.render(&mut render_pass, &self.device, &texture);

        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn set_font(&mut self, font: Font) {
        self.text_system.set_font(&self.device, self.config.clone(), font);
    }

    pub fn set_text_style(&mut self, style: TextStyle) {
        self.text_system.set_style(style);
    }

    pub fn set_text(&mut self, text: &str, x: f32, y: f32, speed: TextSpeed) -> usize {
        self.text_position = Vec2::new(x, y);
        self.typewriter_instance.add_typewriter_effect(text, speed)
    }
    
    
}
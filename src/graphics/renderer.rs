use crate::controllers::text_controller::TextController;
use crate::controllers::texture_controller::TextureController;
use crate::controllers::typewriter_controller::TypewriterController;
use crate::core::{CameraController, RenderContext};
use crate::graphics::camera::Camera;
use crate::graphics::sprite_renderer::SpriteRenderer;
use crate::graphics::{AnimationController, VisualState};
use crate::text::font::DEFAULT_NORMAL_FONT;
use crate::text::text::TextSystem;
use crate::text::typewriter::TypewriterInstance;
use glam::Vec2;
use std::sync::Arc;
use wgpu::{PresentMode, StoreOp};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Renderer {
    render_context: RenderContext,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let typewriter_controller = TypewriterController::new(TypewriterInstance::new());

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

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface isn't supported by the adapter.");

        config.present_mode = PresentMode::Fifo;

        surface.configure(&device, &config);

        let size = window.inner_size();
        let camera = Camera::new(size.width, size.height);

        let camera_controller = CameraController::new(camera);

        let sprite_renderer = SpriteRenderer::new(&device, &config);

        let text_system = TextSystem::new(&device, &config, DEFAULT_NORMAL_FONT, None, None, None, None);

        let text_controller = TextController::new(text_system);

        let device_arc = Arc::new(device);
        let queue_arc = Arc::new(queue);

        Self {
            render_context: RenderContext {
                texture_controller: TextureController::new(Arc::clone(&device_arc), Arc::clone(&queue_arc)),
                camera_controller,
                sprite_renderer,
                text_controller,
                typewriter_controller,
                animation_controller: AnimationController::new(),
                base: VisualState::default(),
                max_height_text: config.height as f32,
                max_width_text: config.width as f32,
                surface,
                device: device_arc,
                queue: queue_arc,
                config,
            },
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.render_context.config.width = new_size.width;
            self.render_context.config.height = new_size.height;
            self.render_context.surface.configure(&self.render_context.device, &self.render_context.config);

            self.render_context.camera_controller.resize(new_size.width, new_size.height);
            self.render_context.text_controller.resize(new_size.width, new_size.height, &self.render_context.queue);
        }
    }

    pub fn draw(&mut self) {
        self.render_context.sprite_renderer.update_camera(&self.render_context.queue, &self.render_context.camera_controller);
        let output = self.render_context.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.render_context.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );
        if !self.render_context.typewriter_controller.is_empty() {
            for typewriter in self.render_context.typewriter_controller.effects() {
                self.render_context.text_controller.queue_text(typewriter.visible_text(), typewriter.x, typewriter.y, Vec2::new(self.render_context.max_width_text, self.render_context.max_height_text));
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
             for (texture, position, size) in self.render_context.texture_controller.get_textures_in_use() {
                 self.render_context.sprite_renderer.render(
                     &mut render_pass,
                     &self.render_context.device,
                     &texture,
                     position,
                     size
                 );
             }
             self.render_context.text_controller.draw(&self.render_context.device, &self.render_context.queue, &mut render_pass);
        }
        self.render_context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
    
    pub fn set_vsync(&mut self, enabled: bool) {
        self.render_context.config.present_mode = if enabled { PresentMode::Fifo } else { PresentMode::Immediate };

        self.render_context.surface.configure(&self.render_context.device, &self.render_context.config);
    }

    pub fn get_camera_controller(&mut self) -> &mut CameraController {
        &mut self.render_context.camera_controller
    }


    pub fn get_animation_controller_mut(&mut self) -> &mut AnimationController {
        &mut self.render_context.animation_controller
    }

    pub fn get_animation_controller(&self) -> &AnimationController {
        &self.render_context.animation_controller
    }

    pub fn get_texture_controller(&mut self) -> &mut TextureController {
        &mut self.render_context.texture_controller
    }

    pub fn get_typewriter_controller(&mut self) -> &mut TypewriterController {
        &mut self.render_context.typewriter_controller
    }

    pub fn get_text_controller(&mut self) -> &mut TextController {
        &mut self.render_context.text_controller
    }
}
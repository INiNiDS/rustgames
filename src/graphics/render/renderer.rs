use crate::controllers::text_controller::TextController;
use crate::controllers::texture_controller::TextureController;
use crate::controllers::typewriter_controller::TypewriterController;
use crate::graphics::{SpriteRenderer, Camera, Color, Texture, SpriteInstance};
use crate::controllers::{AnimationController, CameraController};
use crate::graphics::effects::VisualState;
use crate::text::font::DEFAULT_NORMAL_FONT;
use crate::text::text::TextSystem;
use crate::text::typewriter::TypewriterInstance;
use glam::Vec2;
use std::sync::Arc;
use wgpu::{PresentMode, StoreOp};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::graphics::render::render_settings::RenderSettings;

/// The main renderer. Initialises the WGPU device and surface, performs
/// per-frame drawing of sprites and text.
pub struct Renderer;

impl Renderer {
    pub async fn set(window: Arc<Window>) -> RenderSettings {
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

        let camera = Camera::new(size.width, size.height);
        let camera_controller = CameraController::new(camera);
        let sprite_renderer = SpriteRenderer::new(&device, &config);
        let text_system = TextSystem::new(&device, &config, DEFAULT_NORMAL_FONT, None, None, None, None);
        let text_controller = TextController::new(text_system);

        let device_arc = Arc::new(device);
        let queue_arc = Arc::new(queue);

        RenderSettings::new(
            camera_controller,
            sprite_renderer,
            text_controller,
            typewriter_controller,
            AnimationController::new(),
            VisualState::default(),
            config.width as f32,
            config.height as f32,
            surface,
            Arc::clone(&device_arc),
            Arc::clone(&queue_arc),
            window,
            config,
            TextureController::new(Arc::clone(&device_arc), Arc::clone(&queue_arc)),
            Color::BLUE,
        )
    }

    pub fn resize(render_settings: &mut RenderSettings, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            render_settings.config.width = new_size.width;
            render_settings.config.height = new_size.height;

            render_settings.max_width_text = new_size.width as f32;
            render_settings.max_height_text = new_size.height as f32;

            let device = render_settings.get_device();
            render_settings.surface.configure(device, render_settings.get_config());

            render_settings.get_camera_controller_mut().resize(new_size.width, new_size.height);

            let queue = Arc::clone(render_settings.get_queue());
            render_settings.get_text_controller_mut().resize(new_size.width, new_size.height, &queue);
        }
    }

    pub fn draw(render_settings: &mut RenderSettings) {
        render_settings.get_sprite_renderer().update_camera(&render_settings.queue, render_settings.get_camera_controller());

        let output = render_settings.surface.get_current_texture().expect("Failed to acquire next surface texture");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = render_settings.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let max_w = render_settings.get_max_width_text();
        let max_h = render_settings.get_max_height_text();

        if !render_settings.get_typewriter_controller_mut().is_empty() {
            let typewriter_texts: Vec<_> = render_settings
                .get_typewriter_controller_mut()
                .effects()
                .map(|tw| (tw.visible_text().to_string(), tw.x, tw.y))
                .collect();

            for (text, x, y) in typewriter_texts {
                render_settings.get_text_controller_mut().queue_text(
                    &text, x, y, Vec2::new(max_w, max_h)
                );
            }
        }

        {
            let device = &render_settings.device;
            let queue = &render_settings.queue;
            let sprite_renderer = &mut render_settings.sprite_renderer;
            let texture_controller = &render_settings.texture_controller;
            let text_controller = &mut render_settings.text_controller;

            let background_color = render_settings.background_color.to_wgpu_color();

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(background_color),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            let batches: Vec<(&Texture, &[SpriteInstance])> = texture_controller.get_batched_instances();

            for (texture, instances) in batches {
                if !instances.is_empty() {
                    sprite_renderer.render(
                        &mut render_pass,
                        device,
                        queue,
                        texture,
                        instances,
                    );
                }
            }

            text_controller.draw(device, queue, &mut render_pass);
        }

        render_settings.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        render_settings.get_texture_controller_mut().clear_instances();
    }
}
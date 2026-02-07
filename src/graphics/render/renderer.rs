use crate::graphics::render::render_settings::RenderSettings;
use crate::graphics::{SpriteRenderer, TextureSystem};
use crate::prelude::Color;
use crate::text::TextSystem;
use std::sync::Arc;
use wgpu::{LoadOp, StoreOp};
use winit::dpi::PhysicalSize;


/// The main renderer. Initialize the WGPU device and surface, performs
/// per-frame drawing of sprites and text.
pub struct Renderer;

impl Renderer {
    pub fn resize(render_settings: &mut RenderSettings, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            render_settings.config.width = new_size.width;
            render_settings.config.height = new_size.height;

            render_settings.max_width_text = new_size.width as f32;
            render_settings.max_height_text = new_size.height as f32;

            let device = render_settings.get_device();
            render_settings.surface.configure(device, render_settings.get_config());

            render_settings.get_camera_mut().resize(new_size.width, new_size.height);

            let queue = Arc::clone(render_settings.get_queue());
            render_settings.get_text_system_mut().resize(new_size.width, new_size.height, &queue);
        }
    }

    pub fn draw(settings: &mut RenderSettings) {
        settings.sprite_renderer.update_camera(&settings.queue, settings.get_camera());
        Self::queue_typewriter_text(settings);

        let output = settings.surface.get_current_texture().expect("Surface error");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = settings.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        Self::record_render_commands(settings, &mut encoder, &view);

        settings.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        settings.texture_controller.clear_instances();
    }

    fn record_render_commands(
        settings: &mut RenderSettings,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView
    ) {
        let RenderSettings {
            device, queue, sprite_renderer, texture_controller, text_system, background_color, ..
        } = settings;

        Self::execute_render_pass(encoder, view, *background_color, |pass| {
            Self::draw_scene_layers(pass, device, queue, sprite_renderer, texture_controller, text_system);
        });
    }

    fn draw_scene_layers(
        pass: &mut wgpu::RenderPass<'_>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sprites: &mut SpriteRenderer,
        textures: &TextureSystem,
        text: &mut TextSystem,
    ) {
        for (texture, instances) in textures.get_batched_instances() {
            if !instances.is_empty() {
                sprites.render(pass, device, queue, texture, instances);
            }
        }

        text.draw(device, queue, pass);
    }

    fn queue_typewriter_text(render_settings: &mut RenderSettings) {
        if render_settings.get_text_system().is_empty() {
            return;
        }

        let max_w = render_settings.get_max_width_text();
        let max_h = render_settings.get_max_height_text();

        let typewriter_texts: Vec<_> = render_settings
            .get_text_system()
            .effects()
            .map(|tw| (tw.visible_text().to_string(), tw.x, tw.y))
            .collect();

        for (text, x, y) in typewriter_texts {
            render_settings.get_text_system_mut().queue_text(
                &text, x, y, max_w, max_h
            );
        }
    }

    fn execute_render_pass<F>(
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: Color,
        render_logic: F,
    ) where
        F: FnOnce(&mut wgpu::RenderPass),
    {
        let color_attachment = wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: LoadOp::Clear(color.to_wgpu_color()),
                store: StoreOp::Store,
            },
            depth_slice: None,
        };

        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &[Some(color_attachment)],
            ..Default::default()
        });

        render_logic(&mut rp);
    }
}
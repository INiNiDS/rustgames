use crate::graphics::render::render_settings::RenderSettings;
use crate::graphics::{SpriteInstance, Texture};
use std::sync::Arc;
use wgpu::StoreOp;
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

    pub fn draw(render_settings: &mut RenderSettings) {
        render_settings.get_sprite_renderer().update_camera(&render_settings.queue, render_settings.get_camera());

        let output = render_settings.surface.get_current_texture().expect("Failed to acquire next surface texture");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = render_settings.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        Self::queue_typewriter_text(render_settings);
        Self::execute_render_pass(render_settings, &mut encoder, &view);

        render_settings.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        render_settings.get_texture_controller_mut().clear_instances();
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

    fn execute_render_pass(
        render_settings: &mut RenderSettings,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) {
        let device = &render_settings.device;
        let queue = &render_settings.queue;
        let sprite_renderer = &mut render_settings.sprite_renderer;
        let texture_controller = &render_settings.texture_controller;
        let text_controller = &mut render_settings.text_system;
        let background_color = render_settings.background_color.to_wgpu_color();

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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
}
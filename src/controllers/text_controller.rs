use glam::Vec2;
use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};
use crate::prelude::Font;
use crate::text::TextSystem;

/// Wraps `TextSystem` to queue and render styled text each frame.
pub struct TextController {
    text_system: TextSystem,
}

impl TextController {
    pub fn new(text_system: TextSystem) -> Self {
        Self { text_system }
    }

    pub fn queue_text(&mut self, text: &str, x: f32, y: f32, size: Vec2) {
        self.text_system.queue_text(text, x, y, size.x, size.y);
    }


    pub fn update_normal_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.text_system.update_normal_font(device, config, path);
    }

    pub fn update_bold_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.text_system.update_bold_font(device, config, path);
    }

    pub fn update_italic_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.text_system.update_italic_font(device, config, path);
    }
    
    pub fn update_medium_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.text_system.update_medium_font(device, config, path);
    }
    
    pub fn update_semibold_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.text_system.update_semibold_font(device, config, path);
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &Queue) {
        self.text_system.resize(width, height, queue);
    }

    pub fn draw(&mut self, device: &Device, queue: &Queue, rpass: &mut RenderPass) {
        self.text_system.draw(device, queue, rpass);
    }

    pub fn set_style(&mut self, style: crate::text::TextStyle) {
        self.text_system.set_style(style);
    }

    pub fn set_font_by_id(&mut self, device: &Device, config: &SurfaceConfiguration, font: Font, id: usize) {
        self.text_system.set_font_by_id(device, config, font, id);
    }
}
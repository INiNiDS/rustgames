use wgpu::{Device, Queue};
use wgpu_text::glyph_brush::{Section, Text};

pub struct TextSystem {
    brush: wgpu_text::TextBrush<wgpu_text::glyph_brush::ab_glyph::FontArc>
}

impl TextSystem {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let font = wgpu_text::glyph_brush::ab_glyph::FontArc::try_from_slice(include_bytes!("../caveat.ttf")).unwrap();
        let brush = wgpu_text::BrushBuilder::using_font(font)
            .build(device, config.width, config.height, config.format);
        Self { brush }
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &wgpu::Queue) {
        self.brush.resize_view(width as f32, height as f32, queue);
    }

    pub fn queue_text(&mut self, device: &Device, queue: &Queue, text: &str, x: f32, y: f32,  size: f32, color: [f32; 4],) {
        let section = Section {
            screen_position: (x, y),
            bounds: (f32::INFINITY, f32::INFINITY),
            text: vec![Text::new(text)
                .with_color(color)
                .with_scale(size)],
            ..Section::default()
        };
        self.brush.queue(device, queue, vec![section]).unwrap();
    }

    pub fn draw(&mut self,  rpass: &mut wgpu::RenderPass<'_>) {
        self.brush.draw(rpass);
    }
}
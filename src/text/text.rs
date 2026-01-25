use wgpu::{Device, Queue, SurfaceConfiguration};
use wgpu_text::glyph_brush::{FontId, Section, Text};
use wgpu_text::glyph_brush::ab_glyph::FontArc;
use crate::text::{Font, RichTextParser, TextSegment, TextStyle, TypewriterEffect};

pub struct TextSystem {
    brush: wgpu_text::TextBrush<FontArc>,
    style: TextStyle
}

impl TextSystem {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let brush = wgpu_text::BrushBuilder::using_font(Font::default_font().to_font_arc())
            .build(device, config.width, config.height, config.format);
        Self { brush, style: Default::default()  }
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &Queue) {
        self.brush.resize_view(width as f32, height as f32, queue);
    }

    pub fn queue_text(
        &mut self,
        device: &Device,
        queue: &Queue,
        text: &TypewriterEffect,
        x: f32,
        y: f32,
    ) {
        let segments = RichTextParser::parse(text.visible_text());

        let segment_data: Vec<(String, [f32; 4], f32)> = segments
            .into_iter()
            .map(|segment| match segment {
                TextSegment::Normal(s) => (s, self.style.color.to_array(), self.style.size),
                TextSegment::Bold(s) => (s, self.style.color.to_array(), self.style.size * 1.15),
                TextSegment::Italic(s) => (s, self.style.color.to_array(), self.style.size),
                TextSegment::Colored { text, color } => (text, color.to_array(), self.style.size),
            })
            .collect();

        let wgpu_texts: Vec<Text> = segment_data
            .iter()
            .map(|(s, color, scale)| {
                Text::new(s)
                    .with_color(*color)
                    .with_scale(*scale)
            })
            .collect();

        let section = Section {
            screen_position: (x, y),
            bounds: (f32::INFINITY, f32::INFINITY),
            text: wgpu_texts,
            ..Section::default()
        };

        self.brush
            .queue(device, queue, vec![section])
            .unwrap();
    }
    pub fn set_style(&mut self, style: TextStyle) {
        self.style = style;
    }

    pub fn set_font(&mut self, device: &Device, config: SurfaceConfiguration, font: Font) {
        let brush = wgpu_text::BrushBuilder::using_font(font.to_font_arc())
            .build(device, config.width, config.height, config.format);
        self.brush = brush;
    }
}

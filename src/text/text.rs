use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};
use wgpu_text::glyph_brush::ab_glyph::FontArc;
use wgpu_text::glyph_brush::{
    BuiltInLineBreaker, FontId, HorizontalAlign, Layout, Section, Text, VerticalAlign,
};

use crate::text::font::{
    DEFAULT_BOLD_FONT, DEFAULT_MEDIUM_FONT, DEFAULT_NORMAL_FONT, DEFAULT_SEMIBOLD_FONT
};
use crate::text::{
    Font, FontWeight, RichTextParser, TextAlignment, TextStyle, VerticalAlignment
};

pub struct TextSystem {
    brush: wgpu_text::TextBrush<FontArc>,
    style: TextStyle,
    fonts: Vec<FontArc>,
    queued_sections: Vec<QueuedSection>,
}

impl TextSystem {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        italic_font: &str,
        bold_font: Option<&str>,
        medium_font: Option<&str>,
        semibold_font: Option<&str>,
        normal_font: Option<&str>,
    ) -> Self {
        let load_font = |path: Option<&str>, default: &str| -> FontArc {
            let actual_path = path.unwrap_or(default);
            Font::load(actual_path).expect("Failed to load font").to_font_arc()
        };

        let fonts = vec![
            load_font(normal_font, DEFAULT_NORMAL_FONT), 
            load_font(bold_font, DEFAULT_BOLD_FONT), 
            Font::load(italic_font).expect("Err").to_font_arc(), 
            load_font(medium_font, DEFAULT_MEDIUM_FONT),    
            load_font(semibold_font, DEFAULT_SEMIBOLD_FONT),
        ];

        let brush = wgpu_text::BrushBuilder::using_fonts(fonts.clone())
            .build(device, config.width, config.height, config.format);

        Self {
            brush,
            style: Default::default(),
            fonts,
            queued_sections: Vec::new(),
        }
    }


    fn rebuild_brush(&mut self, device: &Device, config: &SurfaceConfiguration) {
        self.brush = wgpu_text::BrushBuilder::using_fonts(self.fonts.clone())
            .build(device, config.width, config.height, config.format);
    }

    pub fn update_normal_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.fonts[0] = Font::load(path).expect("Failed to load font").to_font_arc();
        self.rebuild_brush(device, config);
    }

    pub fn update_bold_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.fonts[1] = Font::load(path).expect("Failed to load font").to_font_arc();
        self.rebuild_brush(device, config);
    }

    pub fn update_italic_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.fonts[2] = Font::load(path).expect("Failed to load font").to_font_arc();
        self.rebuild_brush(device, config);
    }

    pub fn update_medium_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.fonts[3] = Font::load(path).expect("Failed to load font").to_font_arc();
        self.rebuild_brush(device, config);
    }

    pub fn update_semibold_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.fonts[4] = Font::load(path).expect("Failed to load font").to_font_arc();
        self.rebuild_brush(device, config);
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &Queue) {
        self.brush.resize_view(width as f32, height as f32, queue);
    }

    pub fn queue_text(
        &mut self,
        visible_content: &str,
        x: f32,
        y: f32,
        max_width: f32,
        max_height: f32,
    ) {
        let segments = RichTextParser::parse(visible_content);

        let mut text_data = Vec::new();
        for segment in segments {
            let font_id = match (segment.attrs.weight, segment.attrs.italic) {
                (_, true) => FontId(2),
                (FontWeight::Bold, _) => FontId(1),
                (FontWeight::SemiBold, _) => FontId(4),
                (FontWeight::Medium, _) => FontId(3),
                (FontWeight::Normal, _) => FontId(0),
            };

            let color = segment.attrs.color
                .unwrap_or(self.style.color)
                .to_array();

            text_data.push((segment.text, font_id, color));
        }

        self.queued_sections.push(QueuedSection {
            text_data,
            x,
            y,
            bounds: (max_width, max_height),
        });
    }

    pub fn draw<'a>(
        &'a mut self,
        device: &Device,
        queue: &Queue,
        rpass: &mut RenderPass,
    ) {
        if !self.queued_sections.is_empty() {
            let mut sections = Vec::new();

            for q in &self.queued_sections {
                let mut wgpu_texts = Vec::new();
                for (string, font_id, color) in &q.text_data {
                    wgpu_texts.push(
                        Text::new(string)
                            .with_color(*color)
                            .with_scale(self.style.size)
                            .with_font_id(*font_id)
                    );
                }

                sections.push(Section {
                    screen_position: (q.x, q.y),
                    bounds: q.bounds,
                    text: wgpu_texts,
                    layout: Layout::default()
                        .h_align(self.map_h_alignment(self.style.alignment))
                        .v_align(self.map_v_alignment(self.style.vertical_alignment))
                        .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
                    ..Section::default()
                });
            }

            self.brush.queue(device, queue, sections).unwrap();

            self.queued_sections.clear();
        }

        self.brush.draw(rpass);
    }

    fn map_h_alignment(&self, align: TextAlignment) -> HorizontalAlign {
        match align {
            TextAlignment::Left => HorizontalAlign::Left,
            TextAlignment::Center => HorizontalAlign::Center,
            TextAlignment::Right => HorizontalAlign::Right,
            TextAlignment::Justify => HorizontalAlign::Left,
        }
    }

    fn map_v_alignment(&self, align: VerticalAlignment) -> VerticalAlign {
        match align {
            VerticalAlignment::Top => VerticalAlign::Top,
            VerticalAlignment::Middle => VerticalAlign::Center,
            VerticalAlignment::Bottom => VerticalAlign::Bottom,
        }
    }

    pub fn set_style(&mut self, style: TextStyle) {
        self.style = style;
    }

    pub fn set_font_by_id(&mut self, device: &Device, config: &SurfaceConfiguration, font: Font, id: usize) {
        if id < self.fonts.len() {
            self.fonts[id] = font.to_font_arc();
            self.rebuild_brush(device, config);
        }
    }
}

struct QueuedSection {
    text_data: Vec<(String, FontId, [f32; 4])>,
    x: f32,
    y: f32,
    bounds: (f32, f32),
}
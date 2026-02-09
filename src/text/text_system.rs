use std::slice::Iter;
use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};
use wgpu_text::glyph_brush::ab_glyph::FontArc;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, FontId, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use crate::prelude::{Font, TextAlignment, TextSpeed, TextStyle, TypewriterEffect, VerticalAlignment};
use crate::text::font::{DEFAULT_BOLD_FONT, DEFAULT_MEDIUM_FONT, DEFAULT_NORMAL_FONT, DEFAULT_SEMIBOLD_FONT};
use crate::text::{FontWeight, RichTextParser, TypewriterInstance};

pub struct TextSystem {
    brush: wgpu_text::TextBrush<FontArc>,
    style: TextStyle,
    fonts: Vec<FontArc>,
    queued_sections: Vec<QueuedSection>,
    typewriter_instance: TypewriterInstance
}

impl TextSystem {
    #[must_use]
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
            style: TextStyle::default(),
            fonts,
            queued_sections: Vec::new(),
            typewriter_instance: TypewriterInstance::new(),
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

    pub fn draw(
        &mut self,
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
                        .h_align(Self::map_h_alignment(self.style.alignment))
                        .v_align(Self::map_v_alignment(self.style.vertical_alignment))
                        .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
                });
            }

            self.brush.queue(device, queue, sections).unwrap();

            self.queued_sections.clear();
        }

        self.brush.draw(rpass);
    }

    pub const fn set_style(&mut self, style: TextStyle) {
        self.style = style;
    }

    pub fn set_font_by_id(&mut self, device: &Device, config: &SurfaceConfiguration, font: &Font, id: usize) {
        if id < self.fonts.len() {
            self.fonts[id] = font.to_font_arc();
            self.rebuild_brush(device, config);
        }
    }

    pub fn add_text(&mut self, text: impl Into<String>, speed: TextSpeed, x: f32, y: f32) -> usize {
        self.typewriter_instance.add_typewriter_effect(text, speed, x, y)
    }

    pub fn remove_text(&mut self, id: usize) {
        self.typewriter_instance.remove_typewriter_effect(id);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.typewriter_instance.update(delta_time);
    }

    pub fn skip(&mut self, id: usize) {
        self.typewriter_instance.skip_effect(id);
    }

    pub fn pause(&mut self, id: usize) {
        self.typewriter_instance.pause_effect(id);
    }

    pub fn resume(&mut self, id: usize) {
        self.typewriter_instance.resume_effect(id);
    }

    pub fn set_speed(&mut self, id: usize, speed: TextSpeed) {
        self.typewriter_instance.set_effect_speed(id, speed);
    }

    #[must_use]
    pub fn set_text(&mut self, id: usize, text: impl Into<String>, speed: TextSpeed) -> bool {
        self.typewriter_instance.set_text(id, text, speed)
    }


    #[must_use]
    pub fn get_visible_text(&self, id: usize) -> Option<&str> {
        self.typewriter_instance.get_effect(id).map(TypewriterEffect::visible_text)
    }

    #[must_use]
    pub fn is_complete(&self, id: usize) -> bool {
        self.typewriter_instance.get_effect(id).is_some_and(TypewriterEffect::is_complete)
    }

    #[must_use]
    pub fn get_progress(&self, id: usize) -> f32 {
        self.typewriter_instance.get_effect(id).map_or(0.0, TypewriterEffect::progress)
    }

    pub fn effects(&self) -> Iter<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects()
    }

    pub fn effects_mut(&mut self) -> std::slice::IterMut<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects_mut()
    }

    #[must_use]
    pub fn effect(&self, id: usize) -> Option<&TypewriterEffect> {
        self.typewriter_instance.get_effect(id)
    }

    pub fn effect_mut(&mut self, id: usize) -> Option<&mut TypewriterEffect> {
        self.typewriter_instance.get_effect_mut(id)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.typewriter_instance.is_empty()
    }

    const fn map_h_alignment(align: TextAlignment) -> HorizontalAlign {
        match align {
            TextAlignment::Left | TextAlignment::Justify => HorizontalAlign::Left,
            TextAlignment::Center => HorizontalAlign::Center,
            TextAlignment::Right => HorizontalAlign::Right,
        }
    }

    const fn map_v_alignment(align: VerticalAlignment) -> VerticalAlign {
        match align {
            VerticalAlignment::Top => VerticalAlign::Top,
            VerticalAlignment::Middle => VerticalAlign::Center,
            VerticalAlignment::Bottom => VerticalAlign::Bottom,
        }
    }
}

struct QueuedSection {
    text_data: Vec<(String, FontId, [f32; 4])>,
    x: f32,
    y: f32,
    bounds: (f32, f32),
}

// IT's so big, fix it 
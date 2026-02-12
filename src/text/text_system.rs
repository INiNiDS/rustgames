use crate::prelude::{Font, TextStyle};
use crate::text::font::{
    DEFAULT_BOLD_FONT, DEFAULT_MEDIUM_FONT, DEFAULT_NORMAL_FONT, DEFAULT_SEMIBOLD_FONT,
};
use crate::text::text_section::{QueuedSection, map_h_alignment, map_v_alignment, resolve_font_id};
use crate::text::text_style::TextWrapMode;
use crate::text::{RichTextParser, TypewriterInstance};
use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};
use wgpu_text::glyph_brush::ab_glyph::FontArc;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, FontId, Layout, Section, Text};

/// Manages text rendering including typewriter effects, font loading, and
/// styled text queueing for GPU draw calls.
pub struct TextSystem {
    pub(crate) fonts: Vec<FontArc>,
    brush: wgpu_text::TextBrush<FontArc>,
    queued_sections: Vec<QueuedSection>,
    pub(crate) typewriter_instance: TypewriterInstance,
}

impl TextSystem {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        italic_font: &str,
        bold_font: Option<&str>,
        medium_font: Option<&str>,
        semibold_font: Option<&str>,
        normal_font: Option<&str>,
        light_font: Option<&str>,
        extrabold_font: Option<&str>,
    ) -> Self {
        let load = |p: Option<&str>, d: &str| -> FontArc {
            Font::load(p.unwrap_or(d))
                .expect("Failed to load font")
                .to_font_arc()
        };
        let fonts = vec![
            load(normal_font, DEFAULT_NORMAL_FONT),
            load(bold_font, DEFAULT_BOLD_FONT),
            Font::load(italic_font).expect("Err").to_font_arc(),
            load(medium_font, DEFAULT_MEDIUM_FONT),
            load(semibold_font, DEFAULT_SEMIBOLD_FONT),
            load(light_font, DEFAULT_NORMAL_FONT),
            load(extrabold_font, DEFAULT_BOLD_FONT),
        ];
        let brush = wgpu_text::BrushBuilder::using_fonts(fonts.clone()).build(
            device,
            config.width,
            config.height,
            config.format,
        );
        Self {
            brush,
            fonts,
            queued_sections: Vec::new(),
            typewriter_instance: TypewriterInstance::new(),
        }
    }

    pub(crate) fn rebuild_brush(&mut self, device: &Device, config: &SurfaceConfiguration) {
        self.brush = wgpu_text::BrushBuilder::using_fonts(self.fonts.clone()).build(
            device,
            config.width,
            config.height,
            config.format,
        );
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &Queue) {
        self.brush.resize_view(width as f32, height as f32, queue);
    }

    pub fn queue_text(
        &mut self,
        content: &str,
        x: f32,
        y: f32,
        max_w: f32,
        max_h: f32,
        style: &TextStyle,
    ) {
        let segments = RichTextParser::parse(content);

        let text_data = segments
            .into_iter()
            .map(|seg| {
                let font_id = resolve_font_id(&seg.attrs);
                let color = seg.attrs.color.unwrap_or(style.color).to_array();
                (seg.text, font_id, color)
            })
            .collect();

        self.queued_sections.push(QueuedSection {
            text_data,
            x,
            y,
            bounds: (max_w, max_h),
            scale: style.size,
            h_align: map_h_alignment(style.alignment),
            v_align: map_v_alignment(style.vertical_alignment),
            shadow: style.shadow,
            wrap_mode: style.wrap_mode,
        });
    }

    pub fn draw(&mut self, device: &Device, queue: &Queue, rpass: &mut RenderPass) {
        if self.queued_sections.is_empty() {
            self.brush.draw(rpass);
            return;
        }

        let mut all_sections = Vec::new();
        for q in &self.queued_sections {
            Self::build_section_pair(&mut all_sections, q);
        }
        self.brush.queue(device, queue, all_sections).unwrap();
        self.queued_sections.clear();
        self.brush.draw(rpass);
    }

    fn build_section_pair<'a>(out: &mut Vec<Section<'a>>, q: &'a QueuedSection) {
        let (lb, bounds) = Self::resolve_wrap(q);
        let layout = Layout::default()
            .h_align(q.h_align)
            .v_align(q.v_align)
            .line_breaker(lb);

        if let Some(shadow) = q.shadow {
            let texts = Self::make_texts(&q.text_data, q.scale, Some(shadow.color.to_array()));
            out.push(Section {
                screen_position: (q.x + shadow.offset.0, q.y + shadow.offset.1),
                bounds,
                text: texts,
                layout,
            });
        }

        let texts = Self::make_texts(&q.text_data, q.scale, None);
        out.push(Section {
            screen_position: (q.x, q.y),
            bounds,
            text: texts,
            layout,
        });
    }

    const fn resolve_wrap(q: &QueuedSection) -> (BuiltInLineBreaker, (f32, f32)) {
        match q.wrap_mode {
            TextWrapMode::NoWrap => (
                BuiltInLineBreaker::AnyCharLineBreaker,
                (f32::INFINITY, q.bounds.1),
            ),
            TextWrapMode::Word => (BuiltInLineBreaker::UnicodeLineBreaker, q.bounds),
            TextWrapMode::Character => (BuiltInLineBreaker::AnyCharLineBreaker, q.bounds),
        }
    }

    fn make_texts(
        data: &[(String, FontId, [f32; 4])],
        scale: f32,
        color_override: Option<[f32; 4]>,
    ) -> Vec<Text<'_>> {
        data.iter()
            .map(|(s, fid, c)| {
                let color = color_override.unwrap_or(*c);
                Text::new(s)
                    .with_color(color)
                    .with_scale(scale)
                    .with_font_id(*fid)
            })
            .collect()
    }
}

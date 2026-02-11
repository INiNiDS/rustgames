use crate::prelude::{TextAlignment, VerticalAlignment};
use crate::text::StyledSegment;
use wgpu_text::glyph_brush::{HorizontalAlign, VerticalAlign};

/// Utilities for word-wrapping plain and rich text and measuring text bounds.
pub struct TextWrapper;

impl TextWrapper {
    #[must_use]
    pub fn wrap_text(text: &str, max_width: f32, char_width: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;

        for word in text.split_whitespace() {
            let word_width = word.len() as f32 * char_width;

            if current_width + word_width <= max_width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                    current_width += char_width;
                }
                current_line.push_str(word);
                current_width += word_width;
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                current_line = word.to_string();
                current_width = word_width;
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    #[must_use]
    pub fn wrap_rich_text(
        segments: Vec<StyledSegment>,
        max_width: f32,
        font_size: f32,
    ) -> Vec<Vec<StyledSegment>> {
        let mut lines = Vec::new();
        let mut current_line: Vec<StyledSegment> = Vec::new();
        let mut current_width = 0.0;

        for segment in segments {
            let words: Vec<&str> = segment.text.split_inclusive(' ').collect();

            for word in words {
                let word_width = word.len() as f32 * font_size * 0.55;

                if current_width + word_width > max_width && !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = Vec::new();
                    current_width = 0.0;
                }

                let word_text = word.to_string();
                current_width += word_width;

                if let Some(last_seg) = current_line.last_mut()
                    && last_seg.attrs.weight == segment.attrs.weight
                    && last_seg.attrs.italic == segment.attrs.italic
                    && last_seg.attrs.color == segment.attrs.color
                {
                    last_seg.text.push_str(&word_text);
                    continue;
                }

                current_line.push(StyledSegment {
                    text: word_text,
                    attrs: segment.attrs.clone(),
                });
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    #[must_use]
    pub const fn map_h_alignment(align: TextAlignment) -> HorizontalAlign {
        match align {
            TextAlignment::Left | TextAlignment::Justify => HorizontalAlign::Left,
            TextAlignment::Center => HorizontalAlign::Center,
            TextAlignment::Right => HorizontalAlign::Right,
        }
    }

    #[must_use]
    pub const fn map_v_alignment(align: VerticalAlignment) -> VerticalAlign {
        match align {
            VerticalAlignment::Top => VerticalAlign::Top,
            VerticalAlignment::Middle => VerticalAlign::Center,
            VerticalAlignment::Bottom => VerticalAlign::Bottom,
        }
    }

    pub fn measure_text(text: &str, font_size: f32) -> (f32, f32) {
        let lines: Vec<&str> = text.lines().collect();
        let max_width = lines
            .iter()
            .map(|line| line.len() as f32 * font_size * 0.6)
            .fold(0.0_f32, f32::max);
        let height = lines.len() as f32 * font_size * 1.2;

        (max_width, height)
    }
}

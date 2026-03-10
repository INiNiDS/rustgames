#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
use crate::prelude::{TextAlignment, VerticalAlignment};
use crate::text::StyledSegment;
use wgpu_text::glyph_brush::{HorizontalAlign, VerticalAlign};

/// Utility for wrapping plain or rich text at a maximum pixel width.
pub struct TextWrapper;

impl TextWrapper {
    const CHAR_WIDTH_RATIO: f64 = 0.55;
    const LINE_HEIGHT_RATIO: f64 = 1.2;

    /// Wraps `text` at `max_width` pixels, estimating character width from
    /// `char_width`. Returns a `Vec` of line strings.
    #[must_use]
    pub fn wrap_text(text: &str, max_width: f32, char_width: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0_f64;
        let max_w = f64::from(max_width);
        let char_w = f64::from(char_width);

        for word in text.split_whitespace() {
            Self::fit_word_into_line(
                word,
                char_w,
                max_w,
                &mut current_line,
                &mut current_width,
                &mut lines,
            );
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    fn fit_word_into_line(
        word: &str,
        char_w: f64,
        max_w: f64,
        current_line: &mut String,
        current_width: &mut f64,
        lines: &mut Vec<String>,
    ) {
        let word_width = word.len() as f64 * char_w;

        if !current_line.is_empty() && *current_width + char_w + word_width <= max_w {
            current_line.push(' ');
            current_line.push_str(word);
            *current_width += char_w + word_width;
        } else {
            if !current_line.is_empty() {
                lines.push(std::mem::take(current_line));
            }
            current_line.push_str(word);
            *current_width = word_width;
        }
    }

    /// Wraps a sequence of styled segments at `max_width` pixels, estimated
    /// from `font_size`. Returns lines, each a `Vec<StyledSegment>`.
    #[must_use]
    pub fn wrap_rich_text(
        segments: Vec<StyledSegment>,
        max_width: f32,
        font_size: f32,
    ) -> Vec<Vec<StyledSegment>> {
        let mut lines = Vec::new();
        let mut current_line: Vec<StyledSegment> = Vec::new();
        let mut current_width = 0.0_f64;
        let max_w = f64::from(max_width);
        let char_w_factor = f64::from(font_size) * Self::CHAR_WIDTH_RATIO;

        for segment in segments {
            for word in segment.text.split_inclusive(' ') {
                Self::fit_rich_word_into_line(
                    word,
                    &segment,
                    char_w_factor,
                    max_w,
                    &mut current_line,
                    &mut current_width,
                    &mut lines,
                );
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    // Medium Complexity
    fn fit_rich_word_into_line(
        word: &str,
        segment: &StyledSegment,
        char_w_factor: f64,
        max_w: f64,
        current_line: &mut Vec<StyledSegment>,
        current_width: &mut f64,
        lines: &mut Vec<Vec<StyledSegment>>,
    ) {
        let word_width = word.len() as f64 * char_w_factor;

        if *current_width + word_width > max_w && !current_line.is_empty() {
            lines.push(std::mem::take(current_line));
            *current_width = 0.0;
        }

        *current_width += word_width;

        if let Some(last_seg) = current_line.last_mut()
            && last_seg.attrs == segment.attrs
        {
            last_seg.text.push_str(word);
            return;
        }

        current_line.push(StyledSegment {
            text: word.to_string(),
            attrs: segment.attrs.clone(),
        });
    }

    pub fn measure_text(text: &str, font_size: f32) -> (f32, f32) {
        let lines: Vec<&str> = text.lines().collect();
        let fs = f64::from(font_size);

        let max_width = lines
            .iter()
            .map(|line| line.len() as f64 * fs * 0.6)
            .fold(0.0, f64::max);

        let height = lines.len() as f64 * fs * Self::LINE_HEIGHT_RATIO;

        (max_width as f32, height as f32)
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
}

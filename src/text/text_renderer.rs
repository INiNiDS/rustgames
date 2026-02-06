use std::cmp::PartialEq;
use wgpu_text::glyph_brush::{HorizontalAlign, VerticalAlign};
use crate::graphics::color::Color;

/// Horizontal text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

/// Controls size, colour, alignment, and line spacing of rendered text.
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub line_spacing: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            size: 16.0,
            color: Color::WHITE,
            alignment: TextAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            line_spacing: 1.2,
        }
    }
}

impl TextStyle {
    #[must_use] 
    pub fn new(size: f32) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    #[must_use] 
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    #[must_use] 
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }
}

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
    pub fn map_h_alignment(align: TextAlignment) -> HorizontalAlign {
        match align {
            TextAlignment::Left => HorizontalAlign::Left,
            TextAlignment::Center => HorizontalAlign::Center,
            TextAlignment::Right => HorizontalAlign::Right,
            TextAlignment::Justify => HorizontalAlign::Left, 
        }
    }

    #[must_use] 
    pub fn map_v_alignment(align: VerticalAlignment) -> VerticalAlign {
        match align {
            VerticalAlignment::Top => VerticalAlign::Top,
            VerticalAlignment::Middle => VerticalAlign::Center,
            VerticalAlignment::Bottom => VerticalAlign::Bottom,
        }
    }

    pub fn measure_text(text: &str, font_size: f32) -> (f32, f32) {
        let lines: Vec<&str> = text.lines().collect();
        let max_width = lines.iter()
            .map(|line| line.len() as f32 * font_size * 0.6)
            .fold(0.0_f32, f32::max);
        let height = lines.len() as f32 * font_size * 1.2;

        (max_width, height)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Medium,
    SemiBold,
    Bold,
}

#[derive(Debug, Clone)]
pub struct TextAttributes {
    pub weight: FontWeight,
    pub italic: bool,
    pub color: Option<Color>,
}

#[derive(Debug, Clone)]
pub struct StyledSegment {
    pub text: String,
    pub attrs: TextAttributes,
}

pub struct RichTextParser;

struct ParseState {
    segments: Vec<StyledSegment>,
    current_text: String,
    weight_stack: Vec<FontWeight>,
    italic_stack: Vec<bool>,
    color_stack: Vec<Option<Color>>,
}

impl ParseState {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
            current_text: String::new(),
            weight_stack: vec![FontWeight::Normal],
            italic_stack: vec![false],
            color_stack: vec![None],
        }
    }

    fn flush_segment(&mut self) {
        if self.current_text.is_empty() {
            return;
        }
        self.segments.push(StyledSegment {
            text: std::mem::take(&mut self.current_text),
            attrs: TextAttributes {
                weight: *self.weight_stack.last().unwrap(),
                italic: *self.italic_stack.last().unwrap(),
                color: *self.color_stack.last().unwrap(),
            },
        });
    }

    fn apply_open_tag(&mut self, tag: &str) {
        match tag {
            "b" => self.weight_stack.push(FontWeight::Bold),
            "m" => self.weight_stack.push(FontWeight::Medium),
            "sb" => self.weight_stack.push(FontWeight::SemiBold),
            "i" => self.italic_stack.push(true),
            _ if tag.starts_with("color=") => {
                self.color_stack.push(Color::from_hex(&tag[6..]));
            }
            _ => {}
        }
    }

    fn apply_close_tag(&mut self, tag: &str) {
        match tag {
            "b" | "m" | "sb" => {
                if self.weight_stack.len() > 1 { self.weight_stack.pop(); }
            }
            "i" => {
                if self.italic_stack.len() > 1 { self.italic_stack.pop(); }
            }
            "color" => {
                if self.color_stack.len() > 1 { self.color_stack.pop(); }
            }
            _ => {}
        }
    }
}

impl RichTextParser {
    #[must_use] 
    pub fn parse(text: &str) -> Vec<StyledSegment> {
        let mut state = ParseState::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c != '[' {
                state.current_text.push(c);
                continue;
            }

            let is_closing = chars.peek() == Some(&'/');
            if is_closing { chars.next(); }

            let mut tag_content = String::new();
            while let Some(&next_char) = chars.peek() {
                if next_char == ']' {
                    chars.next();
                    break;
                }
                tag_content.push(chars.next().unwrap());
            }

            state.flush_segment();

            if is_closing {
                state.apply_close_tag(&tag_content);
            } else {
                state.apply_open_tag(&tag_content);
            }
        }

        state.flush_segment();
        state.segments
    }
}
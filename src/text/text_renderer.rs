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
    pub fn new(size: f32) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }
}

/// Utilities for word-wrapping plain and rich text and measuring text bounds.
pub struct TextWrapper;

impl TextWrapper {
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

                if let Some(last_seg) = current_line.last_mut() {
                    if last_seg.attrs.weight == segment.attrs.weight
                        && last_seg.attrs.italic == segment.attrs.italic
                        && last_seg.attrs.color == segment.attrs.color
                    {
                        last_seg.text.push_str(&word_text);
                        continue;
                    }
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

    pub fn map_h_alignment(align: TextAlignment) -> HorizontalAlign {
        match align {
            TextAlignment::Left => HorizontalAlign::Left,
            TextAlignment::Center => HorizontalAlign::Center,
            TextAlignment::Right => HorizontalAlign::Right,
            TextAlignment::Justify => HorizontalAlign::Left, 
        }
    }

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

/// Parses markup tags (`[b]`, `[i]`, `[color=#hex]`) into styled segments.
pub struct RichTextParser;

impl RichTextParser {
    pub fn parse(text: &str) -> Vec<StyledSegment> {
        let mut segments = Vec::new();
        let mut current_text = String::new();
        let mut weight_stack = vec![FontWeight::Normal];
        let mut italic_stack = vec![false];
        let mut color_stack = vec![None];

        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '[' {
                let mut tag_content = String::new();
                let mut is_closing = false;

                if let Some(&'/') = chars.peek() {
                    is_closing = true;
                    chars.next();
                }

                while let Some(&next_char) = chars.peek() {
                    if next_char == ']' {
                        chars.next();
                        break;
                    }
                    tag_content.push(chars.next().unwrap());
                }

                if !current_text.is_empty() {
                    segments.push(StyledSegment {
                        text: current_text.clone(),
                        attrs: TextAttributes {
                            weight: *weight_stack.last().unwrap(),
                            italic: *italic_stack.last().unwrap(),
                            color: *color_stack.last().unwrap(),
                        },
                    });
                    current_text.clear();
                }

                if is_closing {
                    match tag_content.as_str() {
                        "b" | "m" | "sb" => { if weight_stack.len() > 1 { weight_stack.pop(); } }
                        "i" => { if italic_stack.len() > 1 { italic_stack.pop(); } }
                        "color" => { if color_stack.len() > 1 { color_stack.pop(); } }
                        _ => {}
                    }
                } else {
                    match tag_content.as_str() {
                        "b" => weight_stack.push(FontWeight::Bold),
                        "m" => weight_stack.push(FontWeight::Medium),
                        "sb" => weight_stack.push(FontWeight::SemiBold),
                        "i" => italic_stack.push(true),
                        _ if tag_content.starts_with("color=") => {
                            let hex = &tag_content[6..];
                            color_stack.push(Color::from_hex(hex));
                        }
                        _ => {}
                    }
                }
            } else {
                current_text.push(c);
            }
        }

        if !current_text.is_empty() {
            segments.push(StyledSegment {
                text: current_text,
                attrs: TextAttributes {
                    weight: *weight_stack.last().unwrap(),
                    italic: *italic_stack.last().unwrap(),
                    color: *color_stack.last().unwrap(),
                },
            });
        }
        segments
    }
}
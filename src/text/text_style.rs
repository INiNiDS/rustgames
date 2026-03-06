use crate::graphics::Color;
use crate::prelude::{TextAlignment, VerticalAlignment};

/// Controls the drop shadow of the text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextShadow {
    pub offset: (f32, f32),
    pub blur_radius: f32,
    pub color: Color,
}

impl Default for TextShadow {
    fn default() -> Self {
        Self {
            offset: (2.0, 2.0),
            blur_radius: 0.0,
            color: Color::BLACK,
        }
    }
}

/// Defines how text should wrap when it exceeds the container width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWrapMode {
    #[default]
    NoWrap,
    Word,
    Character,
}

/// Controls size, color, alignment, spacing, and visual effects of rendered text.
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub line_spacing: f32,
    pub letter_spacing: f32,
    pub shadow: Option<TextShadow>,
    pub wrap_mode: TextWrapMode,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            size: 16.0,
            color: Color::WHITE,
            alignment: TextAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            line_spacing: 1.2,
            letter_spacing: 0.0,
            shadow: None,
            wrap_mode: TextWrapMode::NoWrap,
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
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    #[must_use]
    pub const fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Builder: sets the vertical alignment within the bounding box.
    #[must_use]
    pub const fn with_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Builder: sets the line spacing multiplier (default `1.2`).
    #[must_use]
    pub const fn with_line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = spacing;
        self
    }

    /// Builder: sets additional space added between each character in pixels.
    #[must_use]
    pub const fn with_letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing;
        self
    }

    /// Builder: attaches a drop shadow to the text.
    #[must_use]
    pub const fn with_shadow(mut self, shadow: TextShadow) -> Self {
        self.shadow = Some(shadow);
        self
    }

    /// Builder: sets the word-wrap mode for this text block.
    #[must_use]
    pub const fn with_wrap(mut self, mode: TextWrapMode) -> Self {
        self.wrap_mode = mode;
        self
    }
}

/// The weight (thickness) of a rendered font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
    /// Thin / light weight.
    Light,
    /// Standard body weight (default).
    #[default]
    Normal,
    /// Mid-weight between normal and semi-bold.
    Medium,
    /// Between medium and bold.
    SemiBold,
    /// Heavy weight.
    Bold,
    /// Heaviest available weight.
    ExtraBold,
}

/// Inline text style attributes parsed from rich-text markup tags.
#[derive(Debug, Clone, Default)]
pub struct TextAttributes {
    /// Font weight for this segment.
    pub weight: FontWeight,
    /// Whether the segment is rendered in italics.
    pub italic: bool,
    /// Whether the segment has an underline decoration.
    pub underline: bool,
    /// Whether the segment has a strikethrough decoration.
    pub strikethrough: bool,
    /// Optional per-segment color override; falls back to the [`TextStyle`] color.
    pub color: Option<Color>,
}

impl PartialEq for TextAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
            && self.italic == other.italic
            && self.underline == other.underline
            && self.strikethrough == other.strikethrough
            && self.color == other.color
    }
}
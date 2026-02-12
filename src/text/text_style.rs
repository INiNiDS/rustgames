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

    #[must_use]
    pub const fn with_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    #[must_use]
    pub const fn with_line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = spacing;
        self
    }

    #[must_use]
    pub const fn with_letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing;
        self
    }

    #[must_use]
    pub const fn with_shadow(mut self, shadow: TextShadow) -> Self {
        self.shadow = Some(shadow);
        self
    }

    #[must_use]
    pub const fn with_wrap(mut self, mode: TextWrapMode) -> Self {
        self.wrap_mode = mode;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
    Light,
    #[default]
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
}

#[derive(Debug, Clone, Default)]
pub struct TextAttributes {
    pub weight: FontWeight,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub color: Option<Color>,
}

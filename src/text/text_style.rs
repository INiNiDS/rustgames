use crate::graphics::Color;
use crate::prelude::{TextAlignment, VerticalAlignment};

/// Controls size, color, alignment, and line spacing of rendered text.
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
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    #[must_use]
    pub const fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
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

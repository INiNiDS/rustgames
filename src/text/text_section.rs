use crate::text::FontWeight;
use crate::text::text_style::{TextShadow, TextWrapMode};
use wgpu_text::glyph_brush::{FontId, HorizontalAlign, VerticalAlign};

use crate::prelude::{TextAlignment, VerticalAlignment};

pub struct QueuedSection {
    pub text_data: Vec<(String, FontId, [f32; 4])>,
    pub x: f32,
    pub y: f32,
    pub bounds: (f32, f32),
    pub scale: f32,
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
    pub shadow: Option<TextShadow>,
    pub wrap_mode: TextWrapMode,
}

pub const fn resolve_font_id(attrs: &crate::text::TextAttributes) -> FontId {
    match (attrs.weight, attrs.italic) {
        (_, true) => FontId(2),
        (FontWeight::Bold, _) => FontId(1),
        (FontWeight::SemiBold, _) => FontId(4),
        (FontWeight::Medium, _) => FontId(3),
        (FontWeight::Normal, _) => FontId(0),
        (FontWeight::Light, _) => FontId(5),
        (FontWeight::ExtraBold, _) => FontId(6),
    }
}

pub const fn map_h_alignment(align: TextAlignment) -> HorizontalAlign {
    match align {
        TextAlignment::Left | TextAlignment::Justify => HorizontalAlign::Left,
        TextAlignment::Center => HorizontalAlign::Center,
        TextAlignment::Right => HorizontalAlign::Right,
    }
}

pub const fn map_v_alignment(align: VerticalAlignment) -> VerticalAlign {
    match align {
        VerticalAlignment::Top => VerticalAlign::Top,
        VerticalAlignment::Middle => VerticalAlign::Center,
        VerticalAlignment::Bottom => VerticalAlign::Bottom,
    }
}

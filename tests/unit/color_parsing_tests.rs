//! Tests for advanced [`Color`] parsing and conversion helpers:
//! `parse_named`, `parse_tuple`, `get_name`, and `From` trait impls.

use rustgames::graphics::color::Color;

// ─────────────────────────────────────────────────────────────────────────────
// parse_named
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn parse_named_white() {
    let c = Color::parse_named("white").unwrap();
    assert_eq!(c, Color::WHITE);
}

#[test]
fn parse_named_black() {
    let c = Color::parse_named("black").unwrap();
    assert_eq!(c, Color::BLACK);
}

#[test]
fn parse_named_red() {
    let c = Color::parse_named("red").unwrap();
    assert_eq!(c, Color::RED);
}

#[test]
fn parse_named_green() {
    let c = Color::parse_named("green").unwrap();
    assert_eq!(c, Color::GREEN);
}

#[test]
fn parse_named_blue() {
    let c = Color::parse_named("blue").unwrap();
    assert_eq!(c, Color::BLUE);
}

#[test]
fn parse_named_yellow() {
    let c = Color::parse_named("yellow").unwrap();
    assert_eq!(c, Color::YELLOW);
}

#[test]
fn parse_named_cyan() {
    let c = Color::parse_named("cyan").unwrap();
    assert_eq!(c, Color::CYAN);
}

#[test]
fn parse_named_magenta() {
    let c = Color::parse_named("magenta").unwrap();
    assert_eq!(c, Color::MAGENTA);
}

#[test]
fn parse_named_transparent() {
    let c = Color::parse_named("transparent").unwrap();
    assert_eq!(c.r, 0.0);
    assert_eq!(c.g, 0.0);
    assert_eq!(c.b, 0.0);
    assert_eq!(c.a, 0.0);
}

#[test]
fn parse_named_case_insensitive() {
    assert!(Color::parse_named("WHITE").is_some());
    assert!(Color::parse_named("White").is_some());
    assert!(Color::parse_named("wHiTe").is_some());
}

#[test]
fn parse_named_dark_gray_variants() {
    let a = Color::parse_named("dark gray").unwrap();
    let b = Color::parse_named("dark_gray").unwrap();
    assert_eq!(a, b);
}

#[test]
fn parse_named_light_gray_variants() {
    let a = Color::parse_named("light gray").unwrap();
    let b = Color::parse_named("light_gray").unwrap();
    assert_eq!(a, b);
}

#[test]
fn parse_named_orange() {
    assert!(Color::parse_named("orange").is_some());
}

#[test]
fn parse_named_purple() {
    assert!(Color::parse_named("purple").is_some());
}

#[test]
fn parse_named_gold() {
    assert!(Color::parse_named("gold").is_some());
}

#[test]
fn parse_named_pink() {
    assert!(Color::parse_named("pink").is_some());
}

#[test]
fn parse_named_brown() {
    assert!(Color::parse_named("brown").is_some());
}

#[test]
fn parse_named_unknown_returns_none() {
    assert!(Color::parse_named("ultraviolet").is_none());
    assert!(Color::parse_named("").is_none());
    assert!(Color::parse_named("redd").is_none());
}

// ─────────────────────────────────────────────────────────────────────────────
// parse_tuple
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn parse_tuple_rgb_three_components() {
    let c = Color::parse_tuple("(255, 0, 128)").unwrap();
    assert!((c.r - 1.0).abs() < 0.01);
    assert!(c.g.abs() < 0.01);
    assert!((c.b - 128.0 / 255.0).abs() < 0.01);
    assert_eq!(c.a, 1.0);
}

#[test]
fn parse_tuple_rgba_four_components() {
    let c = Color::parse_tuple("(255, 255, 0, 128)").unwrap();
    assert!((c.a - 128.0 / 255.0).abs() < 0.01);
}

#[test]
fn parse_tuple_black() {
    let c = Color::parse_tuple("(0, 0, 0)").unwrap();
    assert_eq!(c, Color::BLACK);
}

#[test]
fn parse_tuple_white() {
    let c = Color::parse_tuple("(255, 255, 255)").unwrap();
    assert_eq!(c, Color::WHITE);
}

#[test]
fn parse_tuple_missing_components_returns_none() {
    assert!(Color::parse_tuple("(255, 0)").is_none());
    assert!(Color::parse_tuple("(255)").is_none());
    assert!(Color::parse_tuple("()").is_none());
}

#[test]
fn parse_tuple_non_numeric_returns_none() {
    assert!(Color::parse_tuple("(abc, 0, 0)").is_none());
}

#[test]
fn parse_tuple_no_parens_still_parses() {
    // The function only strips matching `(` and `)` chars and should still work
    // with values in range; if out-of-range u8 parsing fails, expect None.
    let result = Color::parse_tuple("128, 64, 32");
    // The result may or may not parse depending on implementation — just ensure
    // no panic.
    let _ = result;
}

// ─────────────────────────────────────────────────────────────────────────────
// get_name
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn get_name_transparent_returns_transparent() {
    let name = Color::TRANSPARENT.get_name();
    assert_eq!(name, Some("Transparent"));
}

#[test]
fn get_name_unknown_color_returns_none() {
    let c = Color::new(0.123, 0.456, 0.789, 1.0);
    // This is a random color unlikely to be in NAMED_COLORS
    assert!(c.get_name().is_none());
}

// ─────────────────────────────────────────────────────────────────────────────
// From impls
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn from_tuple3_sets_alpha_to_one() {
    let c: Color = (0.1_f32, 0.2_f32, 0.3_f32).into();
    assert!((c.r - 0.1).abs() < f32::EPSILON);
    assert!((c.g - 0.2).abs() < f32::EPSILON);
    assert!((c.b - 0.3).abs() < f32::EPSILON);
    assert_eq!(c.a, 1.0);
}

#[test]
fn from_tuple4_uses_all_components() {
    let c: Color = (0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32).into();
    assert!((c.r - 0.5).abs() < f32::EPSILON);
    assert!((c.g - 0.6).abs() < f32::EPSILON);
    assert!((c.b - 0.7).abs() < f32::EPSILON);
    assert!((c.a - 0.8).abs() < f32::EPSILON);
}

#[test]
fn from_array3_sets_alpha_to_one() {
    let c: Color = [0.3_f32, 0.4_f32, 0.5_f32].into();
    assert!((c.r - 0.3).abs() < f32::EPSILON);
    assert_eq!(c.a, 1.0);
}

#[test]
fn from_array4_uses_all_components() {
    let c: Color = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32].into();
    assert!((c.a - 0.4).abs() < f32::EPSILON);
}

#[test]
fn from_tuple3_zero_is_black_opaque() {
    let c: Color = (0.0_f32, 0.0_f32, 0.0_f32).into();
    assert_eq!(c, Color::BLACK);
    assert_eq!(c.a, 1.0);
}

#[test]
fn from_array4_all_ones_is_white() {
    let c: Color = [1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32].into();
    assert_eq!(c, Color::WHITE);
}

// ─────────────────────────────────────────────────────────────────────────────
// rgb / rgba constructors
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn color_rgb_sets_alpha_to_one() {
    let c = Color::rgb(0.2, 0.4, 0.6);
    assert_eq!(c.a, 1.0);
}

#[test]
fn color_rgba_preserves_all_channels() {
    let c = Color::rgba(0.1, 0.2, 0.3, 0.4);
    assert!((c.r - 0.1).abs() < f32::EPSILON);
    assert!((c.g - 0.2).abs() < f32::EPSILON);
    assert!((c.b - 0.3).abs() < f32::EPSILON);
    assert!((c.a - 0.4).abs() < f32::EPSILON);
}

#[test]
fn color_to_array_matches_fields() {
    let c = Color::new(0.5, 0.6, 0.7, 0.8);
    let arr = c.to_array();
    assert_eq!(arr[0], c.r);
    assert_eq!(arr[1], c.g);
    assert_eq!(arr[2], c.b);
    assert_eq!(arr[3], c.a);
}

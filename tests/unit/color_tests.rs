use rustgames::graphics::color::Color;

#[test]
fn hex_6_digits_parses_correctly() {
    let color = Color::from_hex("#FF8800").unwrap();
    assert!((color.r - 1.0).abs() < 0.01);
    assert!((color.g - 0.533).abs() < 0.01);
    assert!(color.b.abs() < 0.01);
    assert_eq!(color.a, 1.0);
}

#[test]
fn hex_8_digits_includes_alpha() {
    let color = Color::from_hex("#FF880080").unwrap();
    assert!((color.a - 0.502).abs() < 0.01);
}

#[test]
fn hex_invalid_returns_none() {
    assert!(Color::from_hex("#ZZZ").is_none());
    assert!(Color::from_hex("#12345").is_none());
}

#[test]
fn named_constants_correct() {
    assert_eq!(Color::WHITE.to_array(), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(Color::BLACK.to_array(), [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(Color::TRANSPARENT.to_array(), [0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn lerp_midpoint() {
    let result = Color::BLACK.lerp(Color::WHITE, 0.5);
    assert!((result.r - 0.5).abs() < 0.01);
    assert!((result.g - 0.5).abs() < 0.01);
    assert!((result.b - 0.5).abs() < 0.01);
}

#[test]
fn lerp_clamps_above_one() {
    let result = Color::BLACK.lerp(Color::WHITE, 2.0);
    assert_eq!(result.r, 1.0);
}

#[test]
fn with_alpha_preserves_rgb() {
    let color = Color::RED.with_alpha(0.5);
    assert_eq!(color.r, 1.0);
    assert_eq!(color.a, 0.5);
}

#[test]
fn to_u32_white() {
    assert_eq!(Color::WHITE.to_u32(), 0xFFFFFFFF);
}

#[test]
fn from_rgba_u8_conversion() {
    let color = Color::from_rgba_u8(255, 128, 0, 255);
    assert!((color.r - 1.0).abs() < 0.01);
    assert!((color.g - 0.502).abs() < 0.01);
    assert!(color.b.abs() < 0.01);
}

#[test]
fn from_tuple_sets_alpha_to_one() {
    let color: Color = (0.5, 0.6, 0.7).into();
    assert!((color.r - 0.5).abs() < f32::EPSILON);
    assert_eq!(color.a, 1.0);
}

#[test]
fn partial_eq_ignores_alpha() {
    let a = Color::new(1.0, 0.0, 0.0, 1.0);
    let b = Color::new(1.0, 0.0, 0.0, 0.5);
    assert_eq!(a, b);
}

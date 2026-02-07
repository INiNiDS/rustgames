use rustgames::text::{
    RichTextParser, TextWrapper, FontWeight, TextStyle, TextAlignment, VerticalAlignment,
};
use rustgames::graphics::color::Color;

#[test]
fn wrap_text_splits_into_lines() {
    let lines = TextWrapper::wrap_text("Hello world this is a test", 50.0, 10.0);
    assert!(!lines.is_empty());
}

#[test]
fn wrap_text_single_word() {
    let lines = TextWrapper::wrap_text("Hello", 100.0, 10.0);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "Hello");
}

#[test]
fn wrap_text_empty_string() {
    let lines = TextWrapper::wrap_text("", 100.0, 10.0);
    assert!(lines.is_empty());
}

#[test]
fn wrap_text_forces_line_break() {
    let lines = TextWrapper::wrap_text("aaa bbb ccc", 35.0, 10.0);
    assert!(lines.len() >= 2);
}

#[test]
fn rich_text_three_segments() {
    let segments = RichTextParser::parse("Hello [b]world[/b]!");
    assert_eq!(segments.len(), 3);
}

#[test]
fn rich_text_bold_weight() {
    let segments = RichTextParser::parse("[b]bold[/b]");
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].attrs.weight, FontWeight::Bold);
}

#[test]
fn rich_text_italic() {
    let segments = RichTextParser::parse("[i]italic[/i]");
    assert_eq!(segments.len(), 1);
    assert!(segments[0].attrs.italic);
}

#[test]
fn rich_text_nested() {
    let segments = RichTextParser::parse("[b]bold [i]bolditalic[/i][/b]");
    assert!(segments.len() >= 2);
}

#[test]
fn rich_text_color() {
    let segments = RichTextParser::parse("[color=#FF0000]red[/color]");
    assert_eq!(segments.len(), 1);
    assert!(segments[0].attrs.color.is_some());
}

#[test]
fn rich_text_plain_text_only() {
    let segments = RichTextParser::parse("just plain text");
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].attrs.weight, FontWeight::Normal);
    assert!(!segments[0].attrs.italic);
}

#[test]
fn text_style_defaults() {
    let style = TextStyle::default();
    assert_eq!(style.size, 16.0);
    assert_eq!(style.alignment, TextAlignment::Left);
    assert_eq!(style.vertical_alignment, VerticalAlignment::Top);
}

#[test]
fn text_style_builder() {
    let style = TextStyle::new(32.0)
        .with_color(Color::RED)
        .with_alignment(TextAlignment::Center);
    assert_eq!(style.size, 32.0);
    assert_eq!(style.alignment, TextAlignment::Center);
}

#[test]
fn measure_text_positive_dimensions() {
    let (w, h) = TextWrapper::measure_text("Hello", 16.0);
    assert!(w > 0.0);
    assert!(h > 0.0);
}

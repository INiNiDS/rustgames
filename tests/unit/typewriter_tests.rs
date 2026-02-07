use rustgames::text::typewriter::{TextSpeed, TypewriterEffect};
use rustgames::text::TypewriterInstance;

#[test]
fn basic_effect_starts_empty() {
    let mut tw = TypewriterEffect::new("Hello", TextSpeed::Fast, 0, 0.0, 0.0);
    assert!(!tw.is_complete());
    assert_eq!(tw.visible_text(), "");
    tw.update(1.0);
    assert!(!tw.visible_text().is_empty());
}

#[test]
fn instant_speed_shows_all() {
    let tw = TypewriterEffect::new("Hello", TextSpeed::Instant, 0, 0.0, 0.0);
    assert!(tw.is_complete());
    assert_eq!(tw.visible_text(), "Hello");
}

#[test]
fn skip_completes_immediately() {
    let mut tw = TypewriterEffect::new("Hello World", TextSpeed::Slow, 0, 0.0, 0.0);
    tw.skip();
    assert!(tw.is_complete());
    assert_eq!(tw.visible_text(), "Hello World");
}

#[test]
fn reset_clears_progress() {
    let mut tw = TypewriterEffect::new("Hello", TextSpeed::Fast, 0, 0.0, 0.0);
    tw.skip();
    tw.reset();
    assert!(!tw.is_complete());
    assert_eq!(tw.visible_text(), "");
}

#[test]
fn instant_progress_is_one() {
    let tw = TypewriterEffect::new("Hello", TextSpeed::Instant, 0, 0.0, 0.0);
    assert_eq!(tw.progress(), 1.0);
}

#[test]
fn instance_add_remove() {
    let mut inst = TypewriterInstance::new();
    let id = inst.add_typewriter_effect("Test", TextSpeed::Fast, 0.0, 0.0);
    assert_eq!(inst.len(), 1);
    inst.remove_typewriter_effect(id);
    assert!(inst.is_empty());
}

#[test]
fn instance_get_preserves_position() {
    let mut inst = TypewriterInstance::new();
    let id = inst.add_typewriter_effect("Hello", TextSpeed::Instant, 10.0, 20.0);
    let effect = inst.get_effect(id).unwrap();
    assert_eq!(effect.full_text(), "Hello");
    assert_eq!(effect.x, 10.0);
    assert_eq!(effect.y, 20.0);
}

#[test]
fn speed_values() {
    assert_eq!(TextSpeed::Slow.chars_per_second(), 20.0);
    assert_eq!(TextSpeed::Medium.chars_per_second(), 40.0);
    assert_eq!(TextSpeed::Fast.chars_per_second(), 80.0);
    assert!(TextSpeed::Instant.chars_per_second().is_infinite());
    assert_eq!(TextSpeed::Custom(100.0).chars_per_second(), 100.0);
}

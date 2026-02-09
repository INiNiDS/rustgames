use rustgames::graphics::color::Color;
use rustgames::graphics::{EmitterConfig, VfxEffect};
use rustgames::graphics::effects::renderer::VfxRenderer;
use glam::Vec2;

#[test]
fn new_starts_empty() {
    let ra = VfxRenderer::new();
    assert_eq!(ra.active_effect_count(), 0);
    assert!(!ra.flash_state().active);
    assert!(!ra.overlay_state().active);
}

#[test]
fn flash_activates_and_decays() {
    let mut ra = VfxRenderer::new();
    ra.add_effect(VfxEffect::Flash { color: Color::RED, duration: 1.0 });
    assert!(ra.flash_state().active);
    assert!((ra.flash_state().alpha() - 1.0).abs() < 0.01);

    ra.update(0.5);
    assert!(ra.flash_state().active);
    assert!((ra.flash_state().alpha() - 0.5).abs() < 0.01);

    ra.update(0.6);
    assert!(!ra.flash_state().active);
}

#[test]
fn color_overlay_persists() {
    let mut ra = VfxRenderer::new();
    ra.add_effect(VfxEffect::Overlay { color: Color::BLUE, alpha: 0.5 });
    assert!(ra.overlay_state().active);
    assert_eq!(ra.overlay_state().alpha, 0.5);

    ra.update(10.0);
    assert!(ra.overlay_state().active);
}

#[test]
fn clear_overlay_deactivates() {
    let mut ra = VfxRenderer::new();
    ra.add_effect(VfxEffect::Overlay { color: Color::GREEN, alpha: 0.8 });
    ra.clear_overlay();
    assert!(!ra.overlay_state().active);
}

#[test]
fn clear_all_resets_everything() {
    let mut ra = VfxRenderer::new();
    ra.add_effect(VfxEffect::Flash { color: Color::WHITE, duration: 2.0 });
    ra.add_effect(VfxEffect::Overlay { color: Color::RED, alpha: 1.0 });
    ra.clear_all();
    assert_eq!(ra.active_effect_count(), 0);
    assert!(!ra.flash_state().active);
    assert!(!ra.overlay_state().active);
}

#[test]
fn particles_generate_instances() {
    let mut ra = VfxRenderer::new();
    ra.add_effect(VfxEffect::Emitter(EmitterConfig::sparkles(Vec2::ZERO)));
    let frame = ra.build_frame();
    assert!(!frame.particle.is_empty());
}

#[test]
fn empty_frame_has_no_effects() {
    let ra = VfxRenderer::new();
    let frame = ra.build_frame();
    assert!(frame.flash_color.is_none());
    assert!(frame.overlay_color.is_none());
    assert!(frame.particle.is_empty());
}

#[test]
fn particles_decay_over_time() {
    let mut ra = VfxRenderer::new();
    ra.add_effect(VfxEffect::Emitter(EmitterConfig::new(Vec2::ZERO)));
    let initial_count = ra.build_frame().particle.len();
    ra.update(2.0);
    let after_count = ra.build_frame().particle.len();
    assert!(after_count <= initial_count);
}

#[test]
fn multiple_effects_at_once() {
    let mut ra = VfxRenderer::new();
    ra.add_effect(VfxEffect::Flash { color: Color::WHITE, duration: 1.0 });
    ra.add_effect(VfxEffect::Emitter(EmitterConfig::new(Vec2::ZERO)));
    assert_eq!(ra.active_effect_count(), 2);
    ra.update(0.1);
    let frame = ra.build_frame();
    assert!(frame.flash_color.is_some());
}

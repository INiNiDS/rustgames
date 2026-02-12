use glam::Vec2;
use rustgames::graphics::color::Color;
use rustgames::graphics::effects::VfxRenderer;
use rustgames::graphics::effects::trauma_shake::TraumaShake;
use rustgames::graphics::{EmitterConfig, Particle, VfxEffect};

#[test]
fn effect_renderer_lifecycle() {
    let mut renderer = VfxRenderer::new();
    renderer.add_effect(VfxEffect::Flash {
        color: Color::WHITE,
        duration: 0.5,
    });
    assert_eq!(renderer.active_effect_count(), 1);
    renderer.update(1.0);
    assert_eq!(renderer.active_effect_count(), 0);
}

#[test]
fn effect_renderer_clear() {
    let mut renderer = VfxRenderer::new();
    renderer.add_effect(VfxEffect::Vignette { intensity: 0.5 });
    renderer.clear_all();
    assert_eq!(renderer.active_effect_count(), 0);
}

#[test]
fn particle_preset_counts() {
    assert_eq!(EmitterConfig::sparkles(Vec2::ZERO).count, 20);
    assert_eq!(EmitterConfig::explosion(Vec2::ZERO).count, 50);
    assert_eq!(EmitterConfig::rain(Vec2::ZERO).count, 200);
}

#[test]
fn particle_update_moves_position() {
    let mut p = Particle::new(Vec2::ZERO, Vec2::new(10.0, 0.0), 1.0, Color::WHITE, 5.0);
    p.update(0.1, Vec2::ZERO);
    assert!(p.position.x > 0.0);
}

#[test]
fn particle_lifetime_expires() {
    let mut p = Particle::new(Vec2::ZERO, Vec2::ZERO, 0.5, Color::WHITE, 5.0);
    assert!(p.is_alive());
    p.update(0.6, Vec2::ZERO);
    assert!(!p.is_alive());
}

#[test]
fn flash_state_duration() {
    let mut renderer = VfxRenderer::new();
    renderer.add_effect(VfxEffect::Flash {
        color: Color::RED,
        duration: 0.3,
    });
    assert!(renderer.flash_state().active);
}

#[test]
fn vignette_does_not_auto_finish() {
    let mut renderer = VfxRenderer::new();
    renderer.add_effect(VfxEffect::Vignette { intensity: 0.5 });
    renderer.update(10.0);
    assert_eq!(renderer.active_effect_count(), 1);
}

#[test]
fn trauma_shake_activation() {
    let mut shake = TraumaShake::new(10.0, 2.0);
    assert!(!shake.is_active());
    shake.add_trauma(0.5);
    assert!(shake.is_active());
    assert_eq!(shake.trauma(), 0.5);
}

#[test]
fn trauma_shake_produces_offset() {
    let mut shake = TraumaShake::new(10.0, 1.0);
    shake.add_trauma(0.8);
    shake.update(0.016);
    let offset = shake.offset();
    let magnitude = (offset.x * offset.x + offset.y * offset.y).sqrt();
    assert!(magnitude > 0.0);
}

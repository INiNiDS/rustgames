use rustgames::graphics::color::Color;
use rustgames::graphics::effects::effects::{
    EffectInstance, EffectManager, Particle, ParticleEffect, VisualEffect,
};
use rustgames::graphics::effects::shake_effect::TraumaShake;
use glam::Vec2;

#[test]
fn effect_manager_lifecycle() {
    let mut mgr = EffectManager::new();
    mgr.add_effect(VisualEffect::Flash { color: Color::WHITE, duration: 0.5 });
    assert_eq!(mgr.count(), 1);
    mgr.update(1.0);
    assert_eq!(mgr.count(), 0);
}

#[test]
fn effect_manager_clear() {
    let mut mgr = EffectManager::new();
    mgr.add_effect(VisualEffect::Vignette { intensity: 0.5 });
    mgr.clear();
    assert_eq!(mgr.count(), 0);
}

#[test]
fn particle_preset_counts() {
    assert_eq!(ParticleEffect::sparkles(Vec2::ZERO).particle_count, 20);
    assert_eq!(ParticleEffect::explosion(Vec2::ZERO).particle_count, 50);
    assert_eq!(ParticleEffect::rain(Vec2::ZERO).particle_count, 200);
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
fn flash_effect_duration() {
    let inst = EffectInstance::new(VisualEffect::Flash { color: Color::RED, duration: 0.3 });
    assert!((inst.duration() - 0.3).abs() < f32::EPSILON);
}

#[test]
fn vignette_is_infinite_and_incomplete() {
    let inst = EffectInstance::new(VisualEffect::Vignette { intensity: 0.5 });
    assert!(inst.duration().is_infinite());
    assert!(!inst.is_complete());
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

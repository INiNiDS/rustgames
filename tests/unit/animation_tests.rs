use rustgames::graphics::effects::animation::animation_instance::ActiveAnimation;
use rustgames::graphics::effects::animation::easing::Easing;
use rustgames::graphics::effects::animation::timeline::TimelineBuilder;
use rustgames::graphics::effects::animation::visual::{
    AnimEffect, CombinedMode, CustomCombinedMode, VisualState,
};
use glam::Vec2;
use rustgames::graphics::{Animation, AnimationGroupID};

#[test]
fn instance_starts_at_zero_progress() {
    let inst = ActiveAnimation::new(0, Animation::FadeIn { duration: 2.0 }, Easing::Linear, 0.0);
    assert_eq!(inst.progress(), 0.0);
}

#[test]
fn instance_update_reaches_finish() {
    let mut inst = ActiveAnimation::new(0, Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
    inst.update(1.0);
    assert!(inst.is_finished());
}

#[test]
fn instance_delay_postpones_progress() {
    let mut inst = ActiveAnimation::new(0, Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.5);
    inst.update(0.3);
    assert_eq!(inst.progress(), 0.0);
    inst.update(0.3);
    assert!(inst.progress() > 0.0);
}

#[test]
fn instance_paused_does_not_advance() {
    let mut inst = ActiveAnimation::new(0, Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
    inst.paused = true;
    inst.update(0.5);
    assert_eq!(inst.progress(), 0.0);
}

#[test]
fn timeline_builder_constructs_steps() {
    let steps = TimelineBuilder::new()
        .single(Animation::FadeIn { duration: 0.5 }, Easing::Linear)
        .gap(0.2)
        .single(Animation::FadeOut { duration: 0.5 }, Easing::Linear)
        .build();
    assert_eq!(steps.len(), 3);
}

#[test]
fn animation_group_id_empty() {
    let group = AnimationGroupID::empty();
    assert!(group.is_empty());
    assert_eq!(group.len(), 0);
}

#[test]
fn visual_state_default_values() {
    let state = VisualState::default();
    assert_eq!(state.opacity, 1.0);
    assert_eq!(state.position, Vec2::ZERO);
    assert_eq!(state.scale, Vec2::ONE);
    assert_eq!(state.rotation, 0.0);
}

#[test]
fn anim_effect_combine_multiplies_opacity() {
    let a = AnimEffect::with_opacity(0.5);
    let b = AnimEffect::with_opacity(0.5);
    let combined = a.combine(b);
    assert!((combined.opacity_mul - 0.25).abs() < 0.01);
}

#[test]
fn anim_effect_apply_default_adds_offset() {
    let effect = AnimEffect::with_offset(Vec2::new(10.0, 20.0));
    let result = effect.apply_to_default(VisualState::default());
    assert_eq!(result.position, Vec2::new(10.0, 20.0));
    assert_eq!(result.opacity, 1.0);
}

#[test]
fn anim_effect_apply_override_mode() {
    let effect = AnimEffect::with_scale(Vec2::new(2.0, 3.0));
    let config = CustomCombinedMode::with_scale(CombinedMode::Override);
    let result = effect.apply_to_config(VisualState::default(), config);
    assert_eq!(result.scale, Vec2::new(2.0, 3.0));
}

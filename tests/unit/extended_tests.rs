use glam::{Vec2, Vec4};
use rustgames::graphics::color::Color;
use rustgames::graphics::effects::animation::animation_instance::ActiveAnimation;
use rustgames::graphics::effects::animation::easing::Easing;
use rustgames::graphics::effects::animation::timeline::TimelineBuilder;
use rustgames::graphics::effects::animation::transition::{Transition, TransitionState};
use rustgames::graphics::effects::animation::visual::{AnimEffect, VisualState};
use rustgames::graphics::render::instance::SpriteInstance;
use rustgames::graphics::{Animation, AnimationGroupID, Direction};
use rustgames::prelude::PunctuationConfig;
use rustgames::text::typewriter::{TextSpeed, TypewriterEffect};
use rustgames::text::TextStyle;
use rustgames::window::{Event, EventQueue, KeyCode};

// ───────────────────────── Color (10 tests) ─────────────────────────

#[test]
fn color_from_hex_empty_string_returns_none() {
    assert!(Color::from_hex("").is_none());
}

#[test]
fn color_from_hex_wrong_length_returns_none() {
    assert!(Color::from_hex("#ABC").is_none());
    assert!(Color::from_hex("#ABCDE").is_none());
    assert!(Color::from_hex("#ABCDEFABC").is_none());
}

#[test]
fn color_from_hex_non_hex_chars_returns_none() {
    assert!(Color::from_hex("#GGHHII").is_none());
    assert!(Color::from_hex("#XXYYZZ").is_none());
}

#[test]
fn color_from_hex_6_digit_red() {
    let c = Color::from_hex("#FF0000").unwrap();
    assert!((c.r - 1.0).abs() < 0.01);
    assert!(c.g.abs() < 0.01);
    assert!(c.b.abs() < 0.01);
    assert_eq!(c.a, 1.0);
}

#[test]
fn color_from_hex_8_digit_with_alpha() {
    let c = Color::from_hex("#00FF0080").unwrap();
    assert!(c.r.abs() < 0.01);
    assert!((c.g - 1.0).abs() < 0.01);
    assert!(c.b.abs() < 0.01);
    assert!((c.a - 128.0 / 255.0).abs() < 0.01);
}

#[test]
fn color_lerp_at_zero_returns_self() {
    let result = Color::RED.lerp(Color::BLUE, 0.0);
    assert!((result.r - 1.0).abs() < f32::EPSILON);
    assert!(result.b.abs() < f32::EPSILON);
}

#[test]
fn color_lerp_at_one_returns_other() {
    let result = Color::RED.lerp(Color::BLUE, 1.0);
    assert!(result.r.abs() < f32::EPSILON);
    assert!((result.b - 1.0).abs() < f32::EPSILON);
}

#[test]
fn color_lerp_above_one_clamps() {
    let result = Color::BLACK.lerp(Color::WHITE, 5.0);
    assert_eq!(result.r, 1.0);
    assert_eq!(result.g, 1.0);
    assert_eq!(result.b, 1.0);
}

#[test]
fn color_lerp_below_zero_clamps() {
    let result = Color::WHITE.lerp(Color::BLACK, -3.0);
    assert_eq!(result.r, 1.0);
    assert_eq!(result.g, 1.0);
    assert_eq!(result.b, 1.0);
}

#[test]
fn color_to_u32_roundtrip_consistency() {
    let c = Color::from_rgba_u8(200, 100, 50, 255);
    let packed = c.to_u32();
    let r = (packed >> 24) & 0xFF;
    let g = (packed >> 16) & 0xFF;
    let b = (packed >> 8) & 0xFF;
    let a = packed & 0xFF;
    assert_eq!(r, 200);
    assert_eq!(g, 100);
    assert_eq!(b, 50);
    assert_eq!(a, 255);
}

#[test]
fn color_partial_eq_symmetry_and_transitivity() {
    let a = Color::new(0.5, 0.6, 0.7, 1.0);
    let b = Color::new(0.5, 0.6, 0.7, 0.3);
    let c = Color::new(0.5, 0.6, 0.7, 0.0);
    // symmetry
    assert_eq!(a, b);
    assert_eq!(b, a);
    // transitivity
    assert_eq!(b, c);
    assert_eq!(a, c);
}

#[test]
fn color_with_alpha_preserves_rgb_components() {
    let original = Color::rgb(0.2, 0.4, 0.6);
    let modified = original.with_alpha(0.1);
    assert_eq!(modified.r, original.r);
    assert_eq!(modified.g, original.g);
    assert_eq!(modified.b, original.b);
    assert_eq!(modified.a, 0.1);
}

// ───────────────────── Transition System (8 tests) ──────────────────

#[test]
fn transition_state_progress_at_zero() {
    let state = TransitionState::new(Transition::Fade(2.0));
    assert_eq!(state.progress(), 0.0);
}

#[test]
fn transition_state_progress_at_fifty_percent() {
    let mut state = TransitionState::new(Transition::Fade(2.0));
    state.update(1.0);
    assert!((state.progress() - 0.5).abs() < f32::EPSILON);
}

#[test]
fn transition_state_progress_at_hundred_percent() {
    let mut state = TransitionState::new(Transition::Fade(1.0));
    state.update(1.0);
    assert!((state.progress() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn transition_state_update_overshooting_clamps() {
    let mut state = TransitionState::new(Transition::Fade(1.0));
    state.update(5.0);
    assert!(state.is_finished());
    assert_eq!(state.elapsed, 1.0);
    assert!((state.progress() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn transition_state_reset() {
    let mut state = TransitionState::new(Transition::Fade(1.0));
    state.update(1.0);
    assert!(state.is_finished());
    state.reset();
    assert!(!state.is_finished());
    assert_eq!(state.elapsed, 0.0);
    assert_eq!(state.progress(), 0.0);
}

#[test]
fn transition_instant_has_zero_duration() {
    let t = Transition::Instant;
    assert_eq!(t.duration(), 0.0);
}

#[test]
fn transition_fade_duration_matches_input() {
    let t = Transition::Fade(3.5);
    assert!((t.duration() - 3.5).abs() < f32::EPSILON);
}

#[test]
fn transition_is_instant_variants() {
    assert!(Transition::Instant.is_instant());
    assert!(!Transition::Fade(1.0).is_instant());
    assert!(!Transition::FadeToBlack(1.0).is_instant());
    assert!(!Transition::Dissolve(1.0).is_instant());
    assert!(
        !Transition::Wipe {
            direction: Direction::Left,
            duration: 1.0,
        }
        .is_instant()
    );
}

#[test]
fn transition_state_multiple_resets() {
    let mut state = TransitionState::new(Transition::Fade(0.5));
    for _ in 0..3 {
        state.update(0.5);
        assert!(state.is_finished());
        state.reset();
        assert!(!state.is_finished());
        assert_eq!(state.progress(), 0.0);
    }
}

// ────────────────────────── Animation (8 tests) ─────────────────────

#[test]
fn active_animation_starts_at_progress_zero() {
    let inst = ActiveAnimation::new(1, Animation::FadeOut { duration: 2.0 }, Easing::Linear, 0.0);
    assert_eq!(inst.progress(), 0.0);
    assert!(!inst.is_finished());
}

#[test]
fn active_animation_finishes_after_duration() {
    let mut inst =
        ActiveAnimation::new(1, Animation::FadeIn { duration: 0.5 }, Easing::Linear, 0.0);
    inst.update(0.5);
    assert!(inst.is_finished());
    assert!((inst.progress() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn easing_linear_at_boundaries() {
    assert_eq!(Easing::Linear.apply(0.0), 0.0);
    assert_eq!(Easing::Linear.apply(1.0), 1.0);
}

#[test]
fn easing_ease_in_out_at_boundaries() {
    assert_eq!(Easing::EaseInOut.apply(0.0), 0.0);
    assert!((Easing::EaseInOut.apply(1.0) - 1.0).abs() < 0.01);
}

#[test]
fn anim_effect_default_is_identity() {
    let e = AnimEffect::default();
    assert_eq!(e.opacity_mul, 1.0);
    assert_eq!(e.offset_add, Vec2::ZERO);
    assert_eq!(e.scale_mul, Vec2::ONE);
    assert_eq!(e.rotation_add, 0.0);
}

#[test]
fn anim_effect_identity_combine() {
    let identity = AnimEffect::default();
    let effect = AnimEffect::with_opacity(0.7);
    let combined = identity.combine(effect);
    assert!((combined.opacity_mul - 0.7).abs() < f32::EPSILON);
    assert_eq!(combined.offset_add, Vec2::ZERO);
}

#[test]
fn visual_state_default_anchor() {
    let state = VisualState::default();
    assert_eq!(state.anchor, Vec2::splat(0.5));
}

#[test]
fn timeline_builder_with_gaps() {
    let steps = TimelineBuilder::new()
        .single(Animation::FadeIn { duration: 0.3 }, Easing::Linear)
        .gap(0.1)
        .single(Animation::FadeOut { duration: 0.3 }, Easing::Linear)
        .gap(0.2)
        .single(
            Animation::Scale {
                from: 1.0,
                to: 2.0,
                duration: 0.5,
            },
            Easing::EaseOut,
        )
        .build();
    assert_eq!(steps.len(), 5);
}

#[test]
fn animation_group_id_empty_len() {
    let group = AnimationGroupID::empty();
    assert!(group.is_empty());
    assert_eq!(group.len(), 0);
}

// ──────────────────────── Typewriter (8 tests) ──────────────────────

fn make_typewriter(text: &str, speed: TextSpeed) -> TypewriterEffect {
    TypewriterEffect::new(
        text,
        speed,
        0,
        0.0,
        0.0,
        TextStyle::default(),
        PunctuationConfig::default(),
    )
}

#[test]
fn typewriter_progress_starts_at_zero() {
    let tw = make_typewriter("Hello", TextSpeed::Slow);
    assert_eq!(tw.progress(), 0.0);
}

#[test]
fn typewriter_skip_completes_immediately() {
    let mut tw = make_typewriter("Hello World", TextSpeed::Slow);
    tw.skip();
    assert!(tw.is_complete());
    assert_eq!(tw.visible_text(), "Hello World");
    assert_eq!(tw.progress(), 1.0);
}

#[test]
fn typewriter_pause_resume() {
    let mut tw = make_typewriter("Hello", TextSpeed::Fast);
    tw.pause();
    assert!(tw.is_paused());
    tw.update(10.0);
    assert_eq!(tw.progress(), 0.0);
    tw.resume();
    assert!(!tw.is_paused());
    tw.update(10.0);
    assert!(tw.progress() > 0.0);
}

#[test]
fn text_speed_values() {
    assert_eq!(TextSpeed::Slow.chars_per_second(), 20.0);
    assert_eq!(TextSpeed::Medium.chars_per_second(), 40.0);
    assert_eq!(TextSpeed::Fast.chars_per_second(), 80.0);
    assert!(TextSpeed::Instant.chars_per_second().is_infinite());
    assert_eq!(TextSpeed::Custom(50.0).chars_per_second(), 50.0);
}

#[test]
fn punctuation_config_default() {
    let config = PunctuationConfig::default();
    assert!(config.sentence_end >= 0.0);
    assert!(config.comma >= 0.0);
    assert!(config.other >= 0.0);
}

#[test]
fn typewriter_empty_text_handling() {
    let tw = make_typewriter("", TextSpeed::Fast);
    assert_eq!(tw.progress(), 1.0);
    assert_eq!(tw.visible_text(), "");
    assert_eq!(tw.full_text(), "");
}

#[test]
fn typewriter_progress_clamped() {
    let mut tw = make_typewriter("Hi", TextSpeed::Fast);
    tw.update(1000.0);
    assert!(tw.progress() <= 1.0);
}

#[test]
fn typewriter_visible_text_substring() {
    let mut tw = make_typewriter("ABCDEF", TextSpeed::Custom(1.0));
    tw.update(3.0);
    let visible = tw.visible_text();
    assert!(tw.full_text().starts_with(visible));
    assert!(visible.len() <= tw.full_text().len());
}

// ──────────────────────── EventQueue (6 tests) ──────────────────────

#[test]
fn event_queue_starts_empty() {
    let queue = EventQueue::new();
    assert!(queue.is_empty());
}

#[test]
fn event_queue_push_and_drain() {
    let mut queue = EventQueue::new();
    queue.push(Event::WindowClosed);
    assert!(!queue.is_empty());
    let events = queue.drain();
    assert_eq!(events.len(), 1);
    assert!(queue.is_empty());
}

#[test]
fn event_queue_key_pressed_tracking() {
    let mut queue = EventQueue::new();
    queue.push(Event::KeyPressed(KeyCode::Space));
    assert!(queue.is_key_pressed(KeyCode::Space));
    queue.push(Event::KeyReleased(KeyCode::Space));
    assert!(!queue.is_key_pressed(KeyCode::Space));
}

#[test]
fn event_queue_was_key_just_pressed() {
    let mut queue = EventQueue::new();
    assert!(!queue.was_key_just_pressed(KeyCode::Enter));
    queue.push(Event::KeyPressed(KeyCode::Enter));
    assert!(queue.was_key_just_pressed(KeyCode::Enter));
}

#[test]
fn event_queue_multiple_events() {
    let mut queue = EventQueue::new();
    queue.push(Event::KeyPressed(KeyCode::KeyA));
    queue.push(Event::KeyPressed(KeyCode::KeyB));
    queue.push(Event::MouseMoved(10.0, 20.0));
    let events = queue.drain();
    assert_eq!(events.len(), 3);
}

#[test]
fn event_queue_clear() {
    let mut queue = EventQueue::new();
    queue.push(Event::WindowFocused(true));
    queue.push(Event::WindowResized(800, 600));
    queue.clear();
    assert!(queue.is_empty());
}

// ─────────────────────── FpsCounter (5 tests) ───────────────────────

#[test]
fn fps_counter_initial_fps_is_zero() {
    let counter = rustgames::core::FpsCounter::new();
    assert_eq!(counter.fps(), 0.0);
}

#[test]
fn fps_counter_update_changes_fps() {
    let mut counter = rustgames::core::FpsCounter::new();
    counter.update(1.0 / 60.0);
    assert!(counter.fps() > 0.0);
}

#[test]
fn fps_counter_frame_time_tracking() {
    let mut counter = rustgames::core::FpsCounter::new();
    counter.update(0.016);
    let ft = counter.frame_time_ms();
    assert!((ft - 16.0).abs() < 1.0);
}

#[test]
fn fps_counter_multiple_frames() {
    let mut counter = rustgames::core::FpsCounter::new();
    for _ in 0..30 {
        counter.update(1.0 / 30.0);
    }
    let fps = counter.fps();
    assert!((fps - 30.0).abs() < 1.0);
}

#[test]
fn fps_counter_min_max() {
    let mut counter = rustgames::core::FpsCounter::new();
    counter.update(1.0 / 60.0);
    counter.update(1.0 / 30.0);
    assert!(counter.max_fps() >= counter.min_fps());
}

// ───────────────────── SpriteInstance (5 tests) ─────────────────────

#[test]
fn sprite_instance_simple_defaults() {
    let inst = SpriteInstance::simple(Vec2::new(10.0, 20.0), Vec2::new(32.0, 32.0), 0.0, 1.0);
    assert_eq!(inst.uv_rect, [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(inst.color, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn sprite_instance_custom_uv_rect() {
    let inst = SpriteInstance::new(
        Vec2::ZERO,
        Vec2::ONE,
        0.0,
        Vec4::new(0.25, 0.25, 0.5, 0.5),
        Vec4::ONE,
    );
    assert_eq!(inst.uv_rect, [0.25, 0.25, 0.5, 0.5]);
}

#[test]
fn sprite_instance_rotation() {
    let inst = SpriteInstance::new(
        Vec2::ZERO,
        Vec2::ONE,
        std::f32::consts::FRAC_PI_4,
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::ONE,
    );
    assert_eq!(inst.uv_rect, [0.0, 0.0, 1.0, 1.0]);
    // model matrix should differ from identity when rotated
    let m = inst.model;
    let off_diag = m[0][1];
    assert!(off_diag.abs() > 0.1);
}

#[test]
fn sprite_instance_size_alignment() {
    let size = std::mem::size_of::<SpriteInstance>();
    assert_eq!(size, 96);
    assert_eq!(size % 16, 0);
}

#[test]
fn sprite_instance_zero_size() {
    let inst = SpriteInstance::simple(Vec2::ZERO, Vec2::ZERO, 0.0, 1.0);
    // scale of zero means model matrix diagonal should be zero
    assert_eq!(inst.model[0][0], 0.0);
    assert_eq!(inst.model[1][1], 0.0);
}

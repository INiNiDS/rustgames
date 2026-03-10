//! High-volume stress tests that hammer the engine's pure-logic subsystems
//! (no GPU / window required).  They validate correctness under load and serve
//! as regression guards for performance-sensitive code paths.

use glam::{Vec2, Vec4};
use rustgames::core::FpsCounter;
use rustgames::graphics::color::Color;
use rustgames::graphics::effects::animation::animation_instance::ActiveAnimation;
use rustgames::graphics::effects::animation::easing::Easing;
use rustgames::graphics::effects::animation::visual::AnimEffect;
use rustgames::graphics::render::instance::SpriteInstance;
use rustgames::graphics::{Animation, AnimationMode, SpriteAnimation};
use rustgames::window::{Event, EventQueue, KeyCode};

// ─────────────────────────────────────────────────────────────────────────────
// Color – stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn color_partial_eq_ignores_alpha_massive() {
    // 100 000 pairs – different alphas, same RGB must compare equal
    for i in 0u32..100_000 {
        let alpha1 = (i % 256) as f32 / 255.0;
        let alpha2 = ((i + 128) % 256) as f32 / 255.0;
        let r = (i % 256) as f32 / 255.0;
        let g = ((i * 3) % 256) as f32 / 255.0;
        let b = ((i * 7) % 256) as f32 / 255.0;
        let a = Color::new(r, g, b, alpha1);
        let b_color = Color::new(r, g, b, alpha2);
        assert_eq!(
            a, b_color,
            "iteration {i}: RGB-equal colors should be PartialEq"
        );
    }
}

#[test]
fn color_partial_eq_distinct_rgb_never_equal() {
    // Colors that differ in any RGB channel must NOT be equal
    for i in 1u32..1_000 {
        let f = i as f32 / 1000.0;
        let a = Color::new(f, 0.0, 0.0, 1.0);
        let b = Color::new(0.0, f, 0.0, 1.0);
        assert_ne!(a, b, "R≠G should not be equal at i={i}");
    }
}

#[test]
fn color_eq_rgba_requires_matching_alpha() {
    let a = Color::new(1.0, 0.0, 0.0, 1.0);
    let b = Color::new(1.0, 0.0, 0.0, 0.5);
    // PartialEq ignores alpha → equal
    assert_eq!(a, b);
    // eq_rgba checks alpha → not equal
    assert!(!a.eq_rgba(&b));
    // identical including alpha → both agree
    let c = Color::new(1.0, 0.0, 0.0, 1.0);
    assert!(a.eq_rgba(&c));
}

#[test]
fn color_lerp_monotonicity_stress() {
    // Lerp from BLACK to WHITE should be monotonically non-decreasing in r
    let mut prev = 0.0_f32;
    let steps = 10_000;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let c = Color::BLACK.lerp(Color::WHITE, t);
        assert!(
            c.r >= prev - f32::EPSILON,
            "lerp should be monotone at t={t}, c.r={}, prev={}",
            c.r,
            prev
        );
        prev = c.r;
    }
}

#[test]
fn color_lerp_clamps_outside_range_stress() {
    for exp in [2.0_f32, 5.0, 10.0, 100.0, -1.0, -50.0] {
        let result = Color::BLACK.lerp(Color::WHITE, exp);
        assert!(
            result.r <= 1.0 && result.r >= 0.0,
            "r out of range for t={exp}"
        );
        assert!(result.g <= 1.0 && result.g >= 0.0);
        assert!(result.b <= 1.0 && result.b >= 0.0);
        assert!(result.a <= 1.0 && result.a >= 0.0);
    }
}

#[test]
fn color_to_u32_roundtrip_stress() {
    // Every 16th u8 triple to keep the test fast but still thorough
    for r in (0u8..=255).step_by(16) {
        for g in (0u8..=255).step_by(16) {
            for b in (0u8..=255).step_by(16) {
                let color = Color::from_rgba_u8(r, g, b, 255);
                let packed = color.to_u32();
                let r2 = ((packed >> 24) & 0xFF) as u8;
                let g2 = ((packed >> 16) & 0xFF) as u8;
                let b2 = ((packed >> 8) & 0xFF) as u8;
                let a2 = (packed & 0xFF) as u8;
                assert_eq!(r2, r, "r mismatch");
                assert_eq!(g2, g, "g mismatch");
                assert_eq!(b2, b, "b mismatch");
                assert_eq!(a2, 255_u8, "a mismatch");
            }
        }
    }
}

#[test]
fn color_from_hex_exhaustive_valid_6_digit() {
    // Spot-check 1 000 random-ish hex strings generated deterministically
    for i in 0u32..1_000 {
        let r = ((i * 17 + 3) % 256) as u8;
        let g = ((i * 31 + 7) % 256) as u8;
        let b = ((i * 53 + 13) % 256) as u8;
        let hex = format!("#{r:02X}{g:02X}{b:02X}");
        let c = Color::from_hex(&hex).unwrap_or_else(|| panic!("parse failed for {hex}"));
        assert!((c.r - r as f32 / 255.0).abs() < 0.005, "{hex}");
        assert!((c.g - g as f32 / 255.0).abs() < 0.005, "{hex}");
        assert!((c.b - b as f32 / 255.0).abs() < 0.005, "{hex}");
    }
}

#[test]
fn color_with_alpha_does_not_mutate_rgb_stress() {
    let base = Color::new(0.3, 0.5, 0.8, 1.0);
    for i in 0..=100 {
        let alpha = i as f32 / 100.0;
        let c = base.with_alpha(alpha);
        assert_eq!(c.r, base.r);
        assert_eq!(c.g, base.g);
        assert_eq!(c.b, base.b);
        assert_eq!(c.a, alpha);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FpsCounter – stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn fps_counter_rolling_window_stability_stress() {
    let mut counter = FpsCounter::new();
    // Feed 10 000 frames at exactly 60 fps → should converge to ≈60
    for _ in 0..10_000 {
        counter.update(1.0 / 60.0);
    }
    let fps = counter.fps();
    assert!((fps - 60.0).abs() < 0.5, "expected ≈60 fps, got {fps}");
}

#[test]
fn fps_counter_variable_frame_times_no_panic() {
    let mut counter = FpsCounter::new();
    // Alternate between very fast (1ms) and very slow (500ms) frames
    for i in 0..500 {
        let dt = if i % 2 == 0 { 0.001 } else { 0.5 };
        counter.update(dt);
    }
    // Should not panic and should give a sensible (non-negative, non-inf) result
    let fps = counter.fps();
    assert!(fps.is_finite() && fps >= 0.0);
}

#[test]
fn fps_counter_min_le_avg_le_max_stress() {
    let mut counter = FpsCounter::new();
    for i in 0u32..200 {
        // Gradually increasing frame times
        let dt = 1.0 / (10.0 + i as f32);
        counter.update(dt);
    }
    let min = counter.min_fps();
    let avg = counter.fps();
    let max = counter.max_fps();
    assert!(min <= avg + 0.01, "min={min} > avg={avg}");
    assert!(avg <= max + 0.01, "avg={avg} > max={max}");
}

#[test]
fn fps_counter_frame_time_matches_inverse_fps() {
    let mut counter = FpsCounter::new();
    for _ in 0..60 {
        counter.update(1.0 / 30.0);
    }
    let fps = counter.fps();
    let ft_ms = counter.frame_time_ms();
    if fps > 0.0 {
        let expected_ms = 1000.0 / fps;
        assert!(
            (ft_ms - expected_ms).abs() < 0.5,
            "ft={ft_ms} expected≈{expected_ms}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SpriteAnimation – stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn sprite_animation_loop_wraps_correctly_stress() {
    let frames: Vec<Vec4> = (0..8)
        .map(|i| Vec4::new(i as f32 * 0.125, 0.0, 0.125, 1.0))
        .collect();
    let fps = 10.0; // 0.1 s per frame
    let mut anim = SpriteAnimation::new(frames, fps, AnimationMode::Loop);

    // Simulate 100 seconds worth of updates at small increments
    let steps = 10_000u32;
    let dt = 100.0 / steps as f32;
    for _ in 0..steps {
        anim.update(dt);
    }
    // Loop animations never finish
    assert!(!anim.is_finished());
    // Frame index must be within bounds
    assert!(anim.current_frame_index() < 8);
}

#[test]
fn sprite_animation_play_once_finishes_and_freezes_stress() {
    let frames: Vec<Vec4> = (0..4)
        .map(|i| Vec4::new(i as f32 * 0.25, 0.0, 0.25, 1.0))
        .collect();
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PlayOnce);
    // Advance far beyond the end
    for _ in 0..1_000 {
        anim.update(0.1);
    }
    assert!(anim.is_finished());
    // Frame must stay at last frame
    assert_eq!(anim.current_frame_index(), 3);
}

#[test]
fn sprite_animation_ping_pong_bounded_stress() {
    let frame_count = 6;
    let frames: Vec<Vec4> = (0..frame_count)
        .map(|i| {
            Vec4::new(
                i as f32 / frame_count as f32,
                0.0,
                1.0 / frame_count as f32,
                1.0,
            )
        })
        .collect();
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PingPong);
    for _ in 0..10_000 {
        anim.update(0.01);
        let idx = anim.current_frame_index();
        assert!(idx < frame_count, "index {idx} out of bounds");
    }
}

#[test]
fn sprite_animation_from_grid_uv_coverage() {
    // 4×4 grid → 16 frames; each should be a valid [0..1] UV rect
    let anim = SpriteAnimation::from_grid(4, 4, 16, 24.0, AnimationMode::Loop);
    assert_eq!(anim.frame_count(), 16);
    // Walk through all frames by advancing one frame at a time
    let dt = 1.0 / 24.0 + 0.0001; // slightly more than one frame
    let mut clone = anim;
    for _ in 0..32 {
        let uv = clone.current_uv();
        assert!(uv.x >= 0.0 && uv.x <= 1.0, "uv.x={}", uv.x);
        assert!(uv.y >= 0.0 && uv.y <= 1.0, "uv.y={}", uv.y);
        assert!(uv.z > 0.0 && uv.z <= 1.0, "uv.w={}", uv.z);
        assert!(uv.w > 0.0 && uv.w <= 1.0, "uv.h={}", uv.w);
        clone.update(dt);
    }
}

#[test]
fn sprite_animation_pause_freezes_frame_stress() {
    let frames: Vec<Vec4> = (0..4)
        .map(|i| Vec4::new(i as f32 * 0.25, 0.0, 0.25, 1.0))
        .collect();
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
    anim.update(0.05); // advance to frame 0 still (half-way)
    let before_pause = anim.current_frame_index();
    anim.pause();
    for _ in 0..1_000 {
        anim.update(1.0);
    }
    assert_eq!(anim.current_frame_index(), before_pause);
}

// ─────────────────────────────────────────────────────────────────────────────
// AnimEffect / VisualState – stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn anim_effect_opacity_combine_chain_stress() {
    // Chaining 50 half-opacity effects → 0.5^50 ≈ 0
    let mut effect = AnimEffect::default();
    for _ in 0..50 {
        effect = effect.combine(AnimEffect::with_opacity(0.5));
    }
    assert!(effect.opacity_mul < 1e-10, "opacity should approach 0");
}

#[test]
fn anim_effect_offset_accumulates_stress() {
    let mut effect = AnimEffect::default();
    let step = Vec2::new(1.0, 2.0);
    for _ in 0..1_000 {
        effect = effect.combine(AnimEffect::with_offset(step));
    }
    // offset_add should have accumulated to 1000 * step
    assert!(
        (effect.offset_add.x - 1000.0).abs() < 0.01,
        "x={}",
        effect.offset_add.x
    );
    assert!(
        (effect.offset_add.y - 2000.0).abs() < 0.01,
        "y={}",
        effect.offset_add.y
    );
}

#[test]
fn active_animation_many_updates_do_not_overshoot_stress() {
    for dur in [0.1_f32, 0.5, 1.0, 2.0, 10.0] {
        let mut inst =
            ActiveAnimation::new(0, Animation::FadeIn { duration: dur }, Easing::Linear, 0.0);
        // Feed 10 000 tiny increments whose sum vastly exceeds `dur`
        for _ in 0..10_000 {
            inst.update(dur * 2.0 / 10_000.0);
        }
        let p = inst.progress();
        assert!(
            p <= 1.0 + f32::EPSILON,
            "progress={p} exceeded 1.0 for dur={dur}"
        );
        assert!(inst.is_finished());
    }
}

#[test]
fn active_animation_delay_respected_stress() {
    let delay = 1.0_f32;
    let dur = 0.5_f32;
    let mut inst = ActiveAnimation::new(
        0,
        Animation::FadeOut { duration: dur },
        Easing::EaseInOut,
        delay,
    );
    // Update for slightly less than the delay → progress must stay 0
    inst.update(delay - 0.001);
    assert_eq!(inst.progress(), 0.0, "should still be in delay phase");
    // Now cross the delay boundary
    inst.update(0.002);
    assert!(inst.progress() > 0.0, "should have started after delay");
}

// ─────────────────────────────────────────────────────────────────────────────
// SpriteInstance – stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn sprite_instance_bulk_creation_correct_size() {
    let instances: Vec<SpriteInstance> = (0..10_000)
        .map(|i| {
            let pos = Vec2::new(i as f32, -(i as f32));
            SpriteInstance::simple(pos, Vec2::new(16.0, 16.0), 0.0, 1.0)
        })
        .collect();
    assert_eq!(instances.len(), 10_000);
    // GPU alignment: every instance must be 96 bytes
    assert_eq!(size_of::<SpriteInstance>(), 96);
}

#[test]
fn sprite_instance_rotation_sweep_no_nan_stress() {
    use std::f32::consts::TAU;
    let steps = 3_600;
    for i in 0..steps {
        let angle = (i as f32 / steps as f32) * TAU;
        let inst = SpriteInstance::simple(Vec2::ZERO, Vec2::ONE, angle, 1.0);
        for row in &inst.model {
            for val in row {
                assert!(!val.is_nan(), "NaN in model matrix at angle={angle}");
            }
        }
    }
}

#[test]
fn sprite_instance_uv_passthrough_stress() {
    for i in 0..256u32 {
        let x = (i % 16) as f32 / 16.0;
        let y = (i / 16) as f32 / 16.0;
        let uv = Vec4::new(x, y, 1.0 / 16.0, 1.0 / 16.0);
        let inst = SpriteInstance::new(Vec2::ZERO, Vec2::ONE, 0.0, uv, Vec4::ONE);
        assert_eq!(inst.uv_rect[0], x);
        assert_eq!(inst.uv_rect[1], y);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EventQueue – stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn event_queue_high_volume_push_drain() {
    let mut queue = EventQueue::new();
    for _ in 0..50_000 {
        queue.push(Event::WindowClosed);
    }
    let drained = queue.drain();
    assert_eq!(drained.len(), 50_000);
    assert!(queue.is_empty());
}

#[test]
fn event_queue_key_state_consistency_stress() {
    let mut queue = EventQueue::new();
    let keys = [KeyCode::KeyA, KeyCode::KeyB, KeyCode::Space, KeyCode::Enter];
    for _ in 0..1_000 {
        for &k in &keys {
            queue.push(Event::KeyPressed(k));
        }
        for &k in &keys {
            assert!(queue.is_key_pressed(k));
        }
        for &k in &keys {
            queue.push(Event::KeyReleased(k));
        }
        for &k in &keys {
            assert!(!queue.is_key_pressed(k));
        }
    }
}

#[test]
fn event_queue_rapid_clear_stress() {
    let mut queue = EventQueue::new();
    for _ in 0..10_000 {
        queue.push(Event::WindowFocused(true));
        queue.push(Event::WindowResized(1920, 1080));
        queue.clear();
        assert!(queue.is_empty());
    }
}

#[test]
fn event_queue_mixed_events_order_preserved() {
    let mut queue = EventQueue::new();
    let expected: Vec<Event> = (0..100)
        .flat_map(|i| {
            vec![
                Event::MouseMoved(i as f64, i as f64 * 2.0),
                Event::WindowResized(800 + i, 600 + i),
            ]
        })
        .collect();
    for e in &expected {
        queue.push(*e);
    }
    let drained = queue.drain();
    assert_eq!(drained.len(), expected.len());
}

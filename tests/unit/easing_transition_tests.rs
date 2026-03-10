//! Deep-dive tests for every [`Easing`] variant and all [`Transition`]
//! types with [`TransitionState`].  Includes boundary checks, monotonicity
//! probes, and stress sweeps.

use rustgames::graphics::effects::animation::easing::Easing;
use rustgames::graphics::effects::animation::transition::{Transition, TransitionState};
use rustgames::graphics::Direction;

// ─────────────────────────────────────────────────────────────────────────────
// Easing – boundary invariants (every variant must map 0→0 and 1→1)
// ─────────────────────────────────────────────────────────────────────────────

const ALL_EASINGS: [Easing; 6] = [
    Easing::Linear,
    Easing::EaseIn,
    Easing::EaseOut,
    Easing::EaseInOut,
    Easing::Bounce,
    Easing::Elastic,
];

#[test]
fn every_easing_maps_zero_to_zero() {
    for e in ALL_EASINGS {
        let v = e.apply(0.0);
        assert!(v.abs() < 1e-5, "{e:?}.apply(0) = {v}, expected 0");
    }
}

#[test]
fn every_easing_maps_one_to_one() {
    for e in ALL_EASINGS {
        let v = e.apply(1.0);
        assert!((v - 1.0).abs() < 1e-4, "{e:?}.apply(1) = {v}, expected 1");
    }
}

#[test]
fn every_easing_clamps_negative_input() {
    for e in ALL_EASINGS {
        let v = e.apply(-5.0);
        assert!(
            v.abs() < 1e-5,
            "{e:?}.apply(-5) = {v}, expected 0 (clamped)"
        );
    }
}

#[test]
fn every_easing_clamps_above_one_input() {
    for e in ALL_EASINGS {
        let v = e.apply(10.0);
        assert!(
            (v - 1.0).abs() < 1e-4,
            "{e:?}.apply(10) = {v}, expected 1 (clamped)"
        );
    }
}

#[test]
fn every_easing_output_in_range_stress() {
    let steps = 1_000;
    for e in ALL_EASINGS {
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let v = e.apply(t);
            // Elastic can overshoot slightly; allow a generous margin
            assert!(
                (-0.5..=1.5).contains(&v),
                "{e:?}.apply({t}) = {v}, out of generous [-0.5, 1.5] range"
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Easing – shape properties
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn linear_is_monotone_increasing() {
    let mut prev = 0.0_f32;
    for i in 1..=100 {
        let t = i as f32 / 100.0;
        let v = Easing::Linear.apply(t);
        assert!(v >= prev - f32::EPSILON, "Linear not monotone at t={t}");
        prev = v;
    }
}

#[test]
fn ease_in_accelerates_midpoint_below_linear() {
    // EaseIn is slower than linear at t=0.5
    assert!(Easing::EaseIn.apply(0.5) < 0.5);
}

#[test]
fn ease_out_decelerates_midpoint_above_linear() {
    assert!(Easing::EaseOut.apply(0.5) > 0.5);
}

#[test]
fn ease_in_out_symmetric_around_half() {
    let a = Easing::EaseInOut.apply(0.25);
    let b = Easing::EaseInOut.apply(0.75);
    // Mirror symmetry: f(0.25) + f(0.75) ≈ 1
    assert!(
        (a + b - 1.0).abs() < 0.01,
        "EaseInOut symmetry: {a} + {b} ≠ 1"
    );
}

#[test]
fn ease_in_and_ease_out_are_complementary() {
    // EaseOut(t) ≈ 1 - EaseIn(1-t)
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let out = Easing::EaseOut.apply(t);
        let ein_mirror = 1.0 - Easing::EaseIn.apply(1.0 - t);
        assert!(
            (out - ein_mirror).abs() < 0.01,
            "complementarity failed at t={t}: out={out} mirror={ein_mirror}"
        );
    }
}

#[test]
fn bounce_is_non_negative_on_0_to_1() {
    for i in 0..=100 {
        let t = i as f32 / 100.0;
        let v = Easing::Bounce.apply(t);
        assert!(v >= -0.001, "Bounce({t}) = {v} < 0");
    }
}

#[test]
fn elastic_at_half_is_not_linear() {
    let linear_half = 0.5_f32;
    let elastic_half = Easing::Elastic.apply(0.5);
    assert!(
        (elastic_half - linear_half).abs() > 0.05,
        "Elastic at 0.5 should differ significantly from linear"
    );
}

#[test]
fn ease_in_out_midpoint_is_exactly_half() {
    // At t=0.5, EaseInOut should be 0.5 (inflection point)
    let v = Easing::EaseInOut.apply(0.5);
    assert!((v - 0.5).abs() < 0.01, "EaseInOut(0.5) = {v}");
}

// ─────────────────────────────────────────────────────────────────────────────
// Easing – high-volume stress
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_easings_10k_samples_no_nan_no_inf() {
    let steps = 10_000;
    for e in ALL_EASINGS {
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let v = e.apply(t);
            assert!(v.is_finite(), "{e:?}.apply({t}) = {v} is not finite");
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Transition – variant properties
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn transition_instant_duration_is_zero() {
    assert_eq!(Transition::Instant.duration(), 0.0);
}

#[test]
fn transition_fade_duration() {
    assert!((Transition::Fade(2.5).duration() - 2.5).abs() < f32::EPSILON);
}

#[test]
fn transition_fade_to_black_duration() {
    assert!((Transition::FadeToBlack(1.5).duration() - 1.5).abs() < f32::EPSILON);
}

#[test]
fn transition_dissolve_duration() {
    assert!((Transition::Dissolve(0.75).duration() - 0.75).abs() < f32::EPSILON);
}

#[test]
fn transition_wipe_duration() {
    let t = Transition::Wipe {
        direction: Direction::Right,
        duration: 1.2,
    };
    assert!((t.duration() - 1.2).abs() < f32::EPSILON);
}

#[test]
fn only_instant_is_instant() {
    assert!(Transition::Instant.is_instant());
    assert!(!Transition::Fade(1.0).is_instant());
    assert!(!Transition::FadeToBlack(1.0).is_instant());
    assert!(!Transition::Dissolve(1.0).is_instant());
    assert!(
        !Transition::Wipe {
            direction: Direction::Top,
            duration: 1.0,
        }
        .is_instant()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// TransitionState – lifecycle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn transition_state_starts_at_zero_progress() {
    let state = TransitionState::new(Transition::Fade(1.0));
    assert_eq!(state.progress(), 0.0);
    assert!(!state.is_finished());
    assert_eq!(state.elapsed, 0.0);
}

#[test]
fn transition_state_instant_is_always_finished() {
    let state = TransitionState::new(Transition::Instant);
    // Instant has zero duration → progress should be 1.0 immediately
    assert_eq!(state.progress(), 1.0);
}

#[test]
fn transition_state_update_advances_progress() {
    let mut state = TransitionState::new(Transition::Fade(2.0));
    state.update(1.0);
    assert!((state.progress() - 0.5).abs() < f32::EPSILON);
}

#[test]
fn transition_state_finishes_when_elapsed_reaches_duration() {
    let mut state = TransitionState::new(Transition::Fade(1.0));
    state.update(1.0);
    assert!(state.is_finished());
    assert!((state.progress() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn transition_state_overshoot_clamps_at_duration() {
    let mut state = TransitionState::new(Transition::Fade(0.5));
    state.update(99.0);
    assert!(state.is_finished());
    assert_eq!(state.elapsed, 0.5, "elapsed must not exceed duration");
    assert_eq!(state.progress(), 1.0);
}

#[test]
fn transition_state_reset_restores_initial_state() {
    let mut state = TransitionState::new(Transition::Fade(1.0));
    state.update(1.0);
    assert!(state.is_finished());
    state.reset();
    assert!(!state.is_finished());
    assert_eq!(state.elapsed, 0.0);
    assert_eq!(state.progress(), 0.0);
}

#[test]
fn transition_state_reset_then_finish_again() {
    let mut state = TransitionState::new(Transition::Dissolve(0.3));
    for _ in 0..3 {
        state.update(0.3);
        assert!(state.is_finished());
        state.reset();
        assert!(!state.is_finished());
    }
}

#[test]
fn transition_state_progress_monotone_through_update() {
    let mut state = TransitionState::new(Transition::FadeToBlack(1.0));
    let mut prev = 0.0_f32;
    for _ in 0..20 {
        state.update(0.05);
        let p = state.progress();
        assert!(p >= prev - f32::EPSILON, "progress not monotone: {p} < {prev}");
        prev = p;
    }
}

#[test]
fn transition_state_wipe_all_directions() {
    for dir in [Direction::Left, Direction::Right, Direction::Top, Direction::Bottom] {
        let mut state = TransitionState::new(Transition::Wipe {
            direction: dir,
            duration: 1.0,
        });
        state.update(0.5);
        assert!((state.progress() - 0.5).abs() < f32::EPSILON, "dir={dir:?}");
        state.update(0.5);
        assert!(state.is_finished(), "dir={dir:?}");
    }
}

#[test]
fn transition_state_many_small_updates_equals_one_big() {
    // 110 steps × 0.01 = 1.10 s, well past the 1.0 s duration
    let mut state_small = TransitionState::new(Transition::Fade(1.0));
    for _ in 0..110 {
        state_small.update(0.01);
    }

    let mut state_big = TransitionState::new(Transition::Fade(1.0));
    state_big.update(1.1);

    assert!((state_small.progress() - state_big.progress()).abs() < 0.001);
    assert!(state_small.is_finished(), "small-step state should be finished");
    assert!(state_big.is_finished(), "big-step state should be finished");
}


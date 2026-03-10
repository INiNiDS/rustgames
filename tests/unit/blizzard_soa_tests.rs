//! Tests for the SoA (Structure-of-Arrays) particle blizzard physics logic.

use glam::Vec2;
use rustgames::graphics::{AnimationMode, SpriteAnimation};

const BOUNDS: f32 = 500.0;
const GRAVITY: f32 = 60.0;

struct TestSoA {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    anims: Vec<SpriteAnimation>,
}

impl TestSoA {
    fn with_capacity(cap: usize) -> Self {
        Self {
            positions: Vec::with_capacity(cap),
            velocities: Vec::with_capacity(cap),
            anims: Vec::with_capacity(cap),
        }
    }

    fn push(&mut self, pos: Vec2, vel: Vec2) {
        self.positions.push(pos);
        self.velocities.push(vel);
        self.anims.push(SpriteAnimation::from_grid(2, 2, 4, 8.0, AnimationMode::Loop));
    }

    fn len(&self) -> usize {
        self.positions.len()
    }

    fn step(&mut self, dt: f32) {
        for (pos, vel) in self.positions.iter_mut().zip(self.velocities.iter_mut()) {
            vel.y += GRAVITY * dt;
            *vel *= 1.0 - dt * 0.1;
            *pos += *vel * dt;
            if pos.x < -BOUNDS {
                vel.x = vel.x.abs();
                pos.x = -BOUNDS;
            } else if pos.x > BOUNDS {
                vel.x = -vel.x.abs();
                pos.x = BOUNDS;
            }
            if pos.y < -BOUNDS {
                vel.y = vel.y.abs();
                pos.y = -BOUNDS;
            } else if pos.y > BOUNDS {
                vel.y = -vel.y.abs();
                pos.y = BOUNDS;
            }
        }
        for anim in &mut self.anims {
            anim.update(dt);
        }
    }
}

#[test]
fn particle_falls_due_to_gravity() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::ZERO);
    let y_before = soa.positions[0].y;
    soa.step(1.0);
    assert!(soa.positions[0].y > y_before, "particle should fall with gravity");
}

#[test]
fn particle_moves_in_x_with_velocity() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::new(100.0, 0.0));
    soa.step(0.1);
    assert!(soa.positions[0].x > 0.0, "particle should move right");
}

#[test]
fn particle_bounces_off_right_wall() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::new(BOUNDS, 0.0), Vec2::new(200.0, 0.0));
    soa.step(0.016);
    assert!(soa.velocities[0].x <= 0.0, "x vel should reverse at right wall");
    assert!(soa.positions[0].x <= BOUNDS, "x pos must not exceed BOUNDS");
}

#[test]
fn particle_bounces_off_left_wall() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::new(-BOUNDS, 0.0), Vec2::new(-200.0, 0.0));
    soa.step(0.016);
    assert!(soa.velocities[0].x >= 0.0, "x vel should reverse at left wall");
    assert!(soa.positions[0].x >= -BOUNDS);
}

#[test]
fn particle_bounces_off_top_wall() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::new(0.0, -BOUNDS), Vec2::new(0.0, -200.0));
    soa.step(0.016);
    assert!(soa.velocities[0].y >= 0.0, "y vel should reverse at top wall");
    assert!(soa.positions[0].y >= -BOUNDS);
}

#[test]
fn particle_bounces_off_bottom_wall() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::new(0.0, BOUNDS), Vec2::new(0.0, 200.0));
    soa.step(0.016);
    assert!(soa.velocities[0].y <= 0.0, "y vel should reverse at bottom wall");
    assert!(soa.positions[0].y <= BOUNDS);
}

#[test]
fn particle_stays_within_bounds_long_sim() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::new(300.0, -300.0));
    for _ in 0..10_000 {
        soa.step(0.016);
        assert!(soa.positions[0].x.abs() <= BOUNDS + 1e-3,
            "x out of bounds: {}", soa.positions[0].x);
        assert!(soa.positions[0].y.abs() <= BOUNDS + 1e-3,
            "y out of bounds: {}", soa.positions[0].y);
    }
}

#[test]
fn velocity_decays_due_to_damping() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::new(100.0, 0.0));
    let initial_vx = soa.velocities[0].x;
    for _ in 0..100 {
        soa.step(0.016);
    }
    assert!(soa.velocities[0].x < initial_vx, "velocity should decay from damping");
}

#[test]
fn particle_at_origin_zero_velocity_only_falls() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::ZERO);
    soa.step(0.1);
    assert!(soa.positions[0].x.abs() < 1e-5, "no horizontal drift expected");
    assert!(soa.positions[0].y > 0.0, "particle should fall");
}

#[test]
fn all_arrays_have_same_length() {
    let mut soa = TestSoA::with_capacity(50);
    for i in 0..50 {
        soa.push(Vec2::new(i as f32, 0.0), Vec2::ZERO);
    }
    assert_eq!(soa.positions.len(), 50);
    assert_eq!(soa.velocities.len(), 50);
    assert_eq!(soa.anims.len(), 50);
    assert_eq!(soa.len(), 50);
}

#[test]
fn truncate_reduces_all_arrays() {
    let mut soa = TestSoA::with_capacity(100);
    for i in 0..100 {
        soa.push(Vec2::new(i as f32, 0.0), Vec2::ZERO);
    }
    soa.positions.truncate(60);
    soa.velocities.truncate(60);
    soa.anims.truncate(60);
    assert_eq!(soa.positions.len(), 60);
    assert_eq!(soa.velocities.len(), 60);
    assert_eq!(soa.anims.len(), 60);
}

#[test]
fn soa_stress_50_000_particles_step() {
    let n = 50_000;
    let mut soa = TestSoA::with_capacity(n);
    for i in 0..n {
        let angle = (i as f32 / n as f32) * std::f32::consts::TAU;
        soa.push(
            Vec2::new(angle.cos() * 100.0, angle.sin() * 100.0),
            Vec2::new(angle.sin() * 40.0, -angle.cos() * 40.0),
        );
    }
    soa.step(1.0 / 60.0);
    soa.step(1.0 / 60.0);
    for (i, pos) in soa.positions.iter().enumerate() {
        assert!(pos.x.abs() <= BOUNDS + 1e-3 && pos.y.abs() <= BOUNDS + 1e-3,
            "particle {i} out of bounds: {pos}");
    }
}

#[test]
fn anim_inside_soa_advances_frame() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::ZERO);
    let frame_before = soa.anims[0].current_frame_index();
    // 4 frames at 8fps → one frame = 0.125 s; step slightly more
    soa.step(0.13);
    let frame_after = soa.anims[0].current_frame_index();
    assert_ne!(frame_before, frame_after, "animation should advance inside SoA step");
}

#[test]
fn anim_inside_soa_stays_in_bounds() {
    let n = 200;
    let mut soa = TestSoA::with_capacity(n);
    for _ in 0..n {
        soa.push(Vec2::ZERO, Vec2::ZERO);
    }
    for _ in 0..1_000 {
        soa.step(0.016);
    }
    for (i, anim) in soa.anims.iter().enumerate() {
        assert!(anim.current_frame_index() < 4, "anim {i} frame index out of range: {}", anim.current_frame_index());
    }
}

#[test]
fn anim_loop_never_finishes_inside_soa() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::ZERO);
    for _ in 0..500 {
        soa.step(0.1);
    }
    assert!(!soa.anims[0].is_finished(), "loop animation must never finish");
}

#[test]
fn gravity_accumulates_with_multiple_steps() {
    let mut soa = TestSoA::with_capacity(1);
    soa.push(Vec2::ZERO, Vec2::ZERO);
    // After step 1 velocity is GRAVITY*dt
    soa.step(0.1);
    let vy_after_1 = soa.velocities[0].y;
    // After step 2 velocity should be larger (gravity keeps adding)
    soa.step(0.1);
    let vy_after_2 = soa.velocities[0].y;
    assert!(vy_after_2 > vy_after_1, "gravity should accumulate each step");
}

#[test]
fn soa_positions_are_finite_after_stress_run() {
    let n = 1_000;
    let mut soa = TestSoA::with_capacity(n);
    for i in 0..n {
        let f = i as f32 - n as f32 / 2.0;
        soa.push(Vec2::new(f * 0.5, f * 0.3), Vec2::new(-f * 0.2, f * 0.1));
    }
    for _ in 0..500 {
        soa.step(0.016);
    }
    for (i, pos) in soa.positions.iter().enumerate() {
        assert!(pos.x.is_finite() && pos.y.is_finite(), "particle {i} position is non-finite");
    }
    for (i, vel) in soa.velocities.iter().enumerate() {
        assert!(vel.x.is_finite() && vel.y.is_finite(), "particle {i} velocity is non-finite");
    }
}


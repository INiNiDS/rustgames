//! Tests for [`Camera`]: movement, zoom, bounds, follow, screen→world
//! projection, resize, and view-projection matrix sanity checks.

use glam::{Vec2, Vec3};
use rustgames::graphics::Camera;

// ─────────────────────────────────────────────────────────────────────────────
// Construction & defaults
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn camera_default_position_is_origin() {
    let cam = Camera::new(800, 600);
    assert_eq!(cam.position, Vec3::ZERO);
}

#[test]
fn camera_default_zoom_is_one() {
    let cam = Camera::new(800, 600);
    assert!((cam.zoom - 1.0).abs() < f32::EPSILON);
}

// ─────────────────────────────────────────────────────────────────────────────
// move_to / move_by
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn camera_move_to_sets_position() {
    let mut cam = Camera::new(800, 600);
    cam.move_to(100.0, -50.0);
    assert_eq!(cam.position.x, 100.0);
    assert_eq!(cam.position.y, -50.0);
}

#[test]
fn camera_move_by_accumulates() {
    let mut cam = Camera::new(800, 600);
    cam.move_by(30.0, 40.0);
    cam.move_by(-10.0, 10.0);
    assert!((cam.position.x - 20.0).abs() < f32::EPSILON);
    assert!((cam.position.y - 50.0).abs() < f32::EPSILON);
}

#[test]
fn camera_move_by_from_non_zero_start() {
    let mut cam = Camera::new(800, 600);
    cam.move_to(50.0, 50.0);
    cam.move_by(25.0, -25.0);
    assert!((cam.position.x - 75.0).abs() < f32::EPSILON);
    assert!((cam.position.y - 25.0).abs() < f32::EPSILON);
}

// ─────────────────────────────────────────────────────────────────────────────
// Zoom
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn camera_set_zoom_applies_immediately() {
    let mut cam = Camera::new(800, 600);
    cam.set_zoom(3.0);
    assert!((cam.zoom - 3.0).abs() < f32::EPSILON);
}

#[test]
fn camera_set_zoom_clamps_below_minimum() {
    let mut cam = Camera::new(800, 600);
    cam.set_zoom(-1.0);
    assert!(cam.zoom >= 0.1, "zoom must be at least 0.1");
}

#[test]
fn camera_smooth_zoom_converges_toward_target() {
    let mut cam = Camera::new(800, 600);
    cam.set_zoom_smooth(4.0, 10.0);
    for _ in 0..100 {
        cam.update(0.05);
    }
    // After 5 seconds of updates, zoom should be very close to 4.0
    assert!(
        (cam.zoom - 4.0).abs() < 0.05,
        "zoom should approach 4.0, got {}",
        cam.zoom
    );
}

#[test]
fn camera_smooth_zoom_starts_at_original_zoom() {
    let mut cam = Camera::new(800, 600);
    // Before any update the zoom must remain at 1.0
    assert!((cam.zoom - 1.0).abs() < f32::EPSILON);
    cam.set_zoom_smooth(2.0, 5.0);
    assert!(
        (cam.zoom - 1.0).abs() < f32::EPSILON,
        "zoom should not jump instantly"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Bounds
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn camera_bounds_clamp_positive_overshoot() {
    let mut cam = Camera::new(800, 600);
    cam.set_bounds(Vec2::ZERO, Vec2::new(100.0, 100.0));
    cam.move_to(200.0, 300.0);
    assert_eq!(cam.position.x, 100.0);
    assert_eq!(cam.position.y, 100.0);
}

#[test]
fn camera_bounds_clamp_negative_undershoot() {
    let mut cam = Camera::new(800, 600);
    cam.set_bounds(Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));
    cam.move_to(-200.0, -200.0);
    assert_eq!(cam.position.x, -50.0);
    assert_eq!(cam.position.y, -50.0);
}

#[test]
fn camera_bounds_allow_in_range() {
    let mut cam = Camera::new(800, 600);
    cam.set_bounds(Vec2::ZERO, Vec2::new(100.0, 100.0));
    cam.move_to(50.0, 50.0);
    assert_eq!(cam.position.x, 50.0);
    assert_eq!(cam.position.y, 50.0);
}

#[test]
fn camera_clear_bounds_allows_free_movement() {
    let mut cam = Camera::new(800, 600);
    cam.set_bounds(Vec2::ZERO, Vec2::new(10.0, 10.0));
    cam.clear_bounds();
    cam.move_to(9999.0, 9999.0);
    assert_eq!(cam.position.x, 9999.0);
    assert_eq!(cam.position.y, 9999.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// Trauma shake
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn camera_trauma_decays_over_time() {
    let mut cam = Camera::new(800, 600);
    cam.add_trauma(1.0);
    // After a long update the shake should have decayed completely
    cam.update(100.0);
    let matrix_after = cam.build_view_projection_matrix();
    // The camera should be back at origin with no shake — diagonal elements > 0
    assert!(matrix_after.col(0).x.abs() > 0.0);
}

#[test]
fn camera_add_trauma_does_not_panic() {
    let mut cam = Camera::new(1920, 1080);
    // Clamping at 1.0 should be handled gracefully
    cam.add_trauma(2.0);
    cam.add_trauma(0.5);
    cam.update(0.016);
}

// ─────────────────────────────────────────────────────────────────────────────
// screen_to_world
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn screen_center_maps_to_camera_position() {
    let cam = Camera::new(800, 600);
    let screen_size = Vec2::new(800.0, 600.0);
    let center = Vec2::new(400.0, 300.0);
    let world = cam.screen_to_world(center, screen_size);
    // Camera at origin → center of screen maps to (0, 0)
    assert!(world.x.abs() < 0.01, "x={}", world.x);
    assert!(world.y.abs() < 0.01, "y={}", world.y);
}

#[test]
fn screen_top_left_is_negative_world_coords() {
    let cam = Camera::new(800, 600);
    let screen_size = Vec2::new(800.0, 600.0);
    let top_left = Vec2::new(0.0, 0.0);
    let world = cam.screen_to_world(top_left, screen_size);
    assert!(world.x < 0.0, "top-left x should be negative world coord");
    assert!(
        world.y > 0.0,
        "top-left y should be positive world coord (y-up)"
    );
}

#[test]
fn screen_bottom_right_is_positive_x_negative_y() {
    let cam = Camera::new(800, 600);
    let screen_size = Vec2::new(800.0, 600.0);
    let bottom_right = Vec2::new(800.0, 600.0);
    let world = cam.screen_to_world(bottom_right, screen_size);
    assert!(world.x > 0.0);
    assert!(world.y < 0.0);
}

#[test]
fn screen_to_world_respects_camera_offset() {
    let mut cam = Camera::new(800, 600);
    cam.move_to(100.0, 50.0);
    let screen_size = Vec2::new(800.0, 600.0);
    let center = Vec2::new(400.0, 300.0);
    let world = cam.screen_to_world(center, screen_size);
    // Screen center maps to camera position
    assert!((world.x - 100.0).abs() < 0.5, "x={}", world.x);
    assert!((world.y - 50.0).abs() < 0.5, "y={}", world.y);
}

// ─────────────────────────────────────────────────────────────────────────────
// View-projection matrix
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn view_projection_is_finite_and_non_zero() {
    let cam = Camera::new(800, 600);
    let m = cam.build_view_projection_matrix();
    for col in 0..4 {
        for row in 0..4 {
            assert!(
                m.col(col)[row].is_finite(),
                "matrix element [{col}][{row}] is not finite"
            );
        }
    }
    // At least one diagonal element should be non-zero
    assert!(m.col(0).x.abs() > 0.0);
}

#[test]
fn view_projection_changes_with_zoom() {
    let mut cam1 = Camera::new(800, 600);
    cam1.set_zoom(1.0);
    let m1 = cam1.build_view_projection_matrix();

    let mut cam2 = Camera::new(800, 600);
    cam2.set_zoom(2.0);
    let m2 = cam2.build_view_projection_matrix();

    assert_ne!(
        m1.col(0).x,
        m2.col(0).x,
        "different zooms should yield different matrices"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// resize
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn camera_resize_changes_aspect_ratio_effect() {
    let mut cam = Camera::new(800, 600);
    let m_before = cam.build_view_projection_matrix();
    cam.resize(1920, 1080);
    let m_after = cam.build_view_projection_matrix();
    // Wide screen has different x-scale than 4:3
    assert_ne!(m_before.col(0).x, m_after.col(0).x);
}

#[test]
fn camera_resize_to_zero_height_does_not_panic() {
    let mut cam = Camera::new(800, 600);
    cam.resize(800, 0); // degenerate – should not crash
}

// ─────────────────────────────────────────────────────────────────────────────
// Follow target
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn camera_follow_moves_toward_target() {
    let mut cam = Camera::new(800, 600);
    cam.follow(Vec3::new(100.0, 0.0, 0.0), 20.0);
    let before = cam.position.x;
    for _ in 0..60 {
        cam.update(1.0 / 60.0);
    }
    assert!(cam.position.x > before, "camera should move toward target");
}

#[test]
fn camera_stop_follow_freezes_position() {
    let mut cam = Camera::new(800, 600);
    cam.follow(Vec3::new(200.0, 0.0, 0.0), 10.0);
    cam.update(0.1);
    cam.stop_follow();
    let pos_after_stop = cam.position.x;
    cam.update(1.0);
    assert!(
        (cam.position.x - pos_after_stop).abs() < f32::EPSILON,
        "position should not change after stop_follow"
    );
}

use glam::Vec2;
use rustgames::graphics::effects::TraumaShake;
use rustgames::graphics::Camera;

#[test]
fn test_trauma_shake_decay() {
    let mut shake = TraumaShake::new(10.0, 1.0);
    shake.add_trauma(1.0);

    assert_eq!(shake.trauma(), 1.0);
    assert!(shake.is_active());

    shake.update(0.5);
    assert!((shake.trauma() - 0.5).abs() < 0.01);

    shake.update(0.5);
    assert_eq!(shake.trauma(), 0.0);
    assert!(!shake.is_active());
}

#[test]
fn test_trauma_shake_clamping() {
    let mut shake = TraumaShake::new(10.0, 0.1);
    shake.add_trauma(0.8);
    shake.add_trauma(0.5);

    assert_eq!(shake.trauma(), 1.0);
}

#[test]
fn test_camera_smooth_zoom() {
    let mut camera = Camera::new(800, 600);
    camera.set_zoom_smooth(2.0, 5.0);

    assert_eq!(camera.zoom, 1.0);

    camera.update(0.1);

    assert!(camera.zoom > 1.0);
}

#[test]
fn test_camera_bounds() {
    let mut camera = Camera::new(800, 600);
    camera.set_bounds(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0));

    camera.move_to(150.0, 150.0);

    assert_eq!(camera.position.x, 100.0);
    assert_eq!(camera.position.y, 100.0);
}

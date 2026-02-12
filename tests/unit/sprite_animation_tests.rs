use glam::Vec4;
use rustgames::graphics::{AnimationMode, SpriteAnimation};

#[test]
fn test_animation_loop() {
    let frames = vec![
        Vec4::new(0.0, 0.0, 0.5, 0.5),
        Vec4::new(0.5, 0.0, 0.5, 0.5),
        Vec4::new(0.0, 0.5, 0.5, 0.5),
    ];
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
    assert_eq!(anim.current_frame_index(), 0);
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 1);
    anim.update(0.2);
    assert_eq!(anim.current_frame_index(), 0);
    assert!(!anim.is_finished());
}

#[test]
fn test_animation_play_once() {
    let frames = vec![Vec4::new(0.0, 0.0, 0.5, 0.5), Vec4::new(0.5, 0.0, 0.5, 0.5)];
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PlayOnce);
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 1);
    assert!(!anim.is_finished());
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 1);
    assert!(anim.is_finished());
}

#[test]
fn test_animation_ping_pong() {
    let frames = vec![
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
    ];
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PingPong);
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 1);
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 2);
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 1);
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 0);
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 1);
}

#[test]
fn test_animation_from_grid() {
    let anim = SpriteAnimation::from_grid(4, 4, 8, 10.0, AnimationMode::Loop);
    assert_eq!(anim.frame_count(), 8);
    let first_uv = anim.current_uv();
    assert_eq!(first_uv.x, 0.0);
    assert_eq!(first_uv.y, 0.0);
    assert_eq!(first_uv.z, 0.25);
    assert_eq!(first_uv.w, 0.25);
}

#[test]
fn test_animation_pause_resume() {
    let frames = vec![Vec4::new(0.0, 0.0, 1.0, 1.0), Vec4::new(0.0, 0.0, 1.0, 1.0)];
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
    anim.pause();
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 0);
    anim.resume();
    anim.update(0.1);
    assert_eq!(anim.current_frame_index(), 1);
}

#[test]
fn test_animation_reset() {
    let frames = vec![Vec4::new(0.0, 0.0, 1.0, 1.0), Vec4::new(0.0, 0.0, 1.0, 1.0)];
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PlayOnce);
    anim.update(0.2);
    assert!(anim.is_finished());
    anim.reset();
    assert_eq!(anim.current_frame_index(), 0);
    assert!(!anim.is_finished());
}

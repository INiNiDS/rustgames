use rustgames::graphics::render::instance::SpriteInstance;
use rustgames::graphics::effects::animation::sprite_animation::{AnimationMode, SpriteAnimation};
use glam::{Vec2, Vec4};

#[test]
fn simple_defaults() {
    let inst = SpriteInstance::simple(Vec2::ZERO, Vec2::new(64.0, 64.0), 0.0, 1.0);
    assert_eq!(inst.uv_rect, [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(inst.color[3], 1.0);
}

#[test]
fn custom_color() {
    let inst = SpriteInstance::new(
        Vec2::ZERO,
        Vec2::ONE,
        0.0,
        Vec4::new(0.0, 0.0, 1.0, 1.0),
        Vec4::new(1.0, 0.0, 0.0, 0.5),
    );
    assert_eq!(inst.color, [1.0, 0.0, 0.0, 0.5]);
}

#[test]
fn set_frame_clamps() {
    let frames = vec![
        Vec4::new(0.0, 0.0, 0.5, 0.5),
        Vec4::new(0.5, 0.0, 0.5, 0.5),
        Vec4::new(0.0, 0.5, 0.5, 0.5),
    ];
    let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
    anim.set_frame(2);
    assert_eq!(anim.current_frame_index(), 2);
    anim.set_frame(100);
    assert_eq!(anim.current_frame_index(), 2);
}

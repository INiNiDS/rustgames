#[cfg(test)]
mod tests {
    use glam::{Vec2, Vec4};
    use rustgames::graphics::SpriteInstance;

    #[test]
    fn test_instance_size_alignment() {
        let size = size_of::<SpriteInstance>();
        assert_eq!(size, 96, "SpriteInstance size must be 96 bytes");
        assert_eq!(size % 16, 0, "SpriteInstance must be 16-byte aligned");
    }

    #[test]
    fn test_instance_creation() {
        let instance =
            SpriteInstance::simple(Vec2::new(100.0, 200.0), Vec2::new(64.0, 64.0), 0.0, 1.0);
        assert_eq!(instance.uv_rect, [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(instance.color, [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_instance_with_rotation() {
        let instance = SpriteInstance::new(
            Vec2::ZERO,
            Vec2::ONE,
            std::f32::consts::PI / 2.0,
            Vec4::new(0.0, 0.0, 0.5, 0.5),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(instance.uv_rect, [0.0, 0.0, 0.5, 0.5]);
        assert_eq!(instance.color, [1.0, 0.0, 0.0, 1.0]);
    }
}

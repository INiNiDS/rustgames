use crate::graphics::Color;
use glam::{Mat4, Vec2, Vec4};
use wgpu::{BufferAddress, VertexBufferLayout, VertexFormat};

/// GPU-side representation of a sprite instance for instanced rendering.
///
/// 96 bytes: 4×4 model matrix, UV rectangle, and color tint.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteInstance {
    pub model: [[f32; 4]; 4],
    pub uv_rect: [f32; 4],
    pub color: [f32; 4],
}

impl SpriteInstance {
    #[must_use]
    pub fn new(position: Vec2, size: Vec2, rotation: f32, uv_rect: Vec4, color: Vec4) -> Self {
        let translation = Mat4::from_translation(position.extend(0.0));
        let rotation_mat = Mat4::from_rotation_z(rotation);
        let scale = Mat4::from_scale(size.extend(1.0));

        let model = translation * rotation_mat * scale;

        Self {
            model: model.to_cols_array_2d(),
            uv_rect: uv_rect.to_array(),
            color: color.to_array(),
        }
    }

    #[must_use]
    pub fn simple(position: Vec2, size: Vec2, rotation: f32, opacity: f32) -> Self {
        let mut color = Color::WHITE;
        color.a = opacity;
        Self::new(
            position,
            size,
            rotation,
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::from(color.to_array()),
        )
    }

    const INSTANCE_ATTRIBUTES: [wgpu::VertexAttribute; 6] = [
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 2,
            format: VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: size_of::<[f32; 4]>() as BufferAddress,
            shader_location: 3,
            format: VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: size_of::<[f32; 8]>() as BufferAddress,
            shader_location: 4,
            format: VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: size_of::<[f32; 12]>() as BufferAddress,
            shader_location: 5,
            format: VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: size_of::<[f32; 16]>() as BufferAddress,
            shader_location: 6,
            format: VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: size_of::<[f32; 20]>() as BufferAddress,
            shader_location: 7,
            format: VertexFormat::Float32x4,
        },
    ];

    #[must_use]
    pub const fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::INSTANCE_ATTRIBUTES,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_size_alignment() {
        let size = std::mem::size_of::<SpriteInstance>();
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

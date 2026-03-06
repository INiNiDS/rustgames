use crate::graphics::Color;
use crate::utils;
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
    /// Creates a [`SpriteInstance`] from explicit transformation components.
    ///
    /// * `position` — world-space centre of the sprite.
    /// * `size` — pixel dimensions of the sprite quad.
    /// * `rotation` — counter-clockwise rotation in radians.
    /// * `uv_rect` — UV sub-rectangle `(u, v, w, h)` into the texture atlas.
    /// * `color` — RGBA tint applied during shading.
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

    /// Convenience constructor: full UV rect, white tint with `opacity` alpha.
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

    /// Returns the wgpu [`VertexBufferLayout`] that describes the per-instance
    /// data layout to the GPU pipeline.
    #[must_use]
    pub const fn desc() -> VertexBufferLayout<'static> {
        utils::render_utils::desc(&Self::INSTANCE_ATTRIBUTES, size_of::<Self>() as BufferAddress, wgpu::VertexStepMode::Instance)
    }
}

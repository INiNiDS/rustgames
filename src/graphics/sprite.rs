use crate::graphics::color::Color;
use crate::utils;
use glam::Vec2;
use std::sync::Arc;
use wgpu::{BufferAddress, VertexBufferLayout};

/// A textured 2D sprite with position, size, rotation, color tint, anchor
/// point, and per-axis flip support.
pub struct Sprite {
    pub texture: Arc<wgpu::Texture>,
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    pub color: Color,
    pub anchor: Vec2,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Sprite {
    /// Creates a new [`Sprite`] whose size matches the texture dimensions.
    /// Defaults: position `(0,0)`, no rotation, scale `(1,1)`, white tint,
    /// centred anchor, no flip.
    #[must_use]
    pub fn new(texture: Arc<wgpu::Texture>) -> Self {
        let size = texture.size();

        let size = glam::uvec2(size.width, size.height).as_vec2();

        Self {
            texture,
            size,
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            color: Color::WHITE,
            anchor: Vec2::new(0.5, 0.5),
            flip_x: false,
            flip_y: false,
        }
    }

    /// Builder: sets the world-space position of the sprite.
    #[must_use]
    pub const fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = Vec2::new(x, y);
        self
    }

    /// Mutates the world-space position of the sprite.
    pub const fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);
    }

    /// Builder: sets the rotation angle in radians (counter-clockwise).
    #[must_use]
    pub const fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = angle;
        self
    }

    /// Mutates the rotation angle in radians.
    pub const fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }

    /// Builder: sets independent `x` / `y` scale factors.
    #[must_use]
    pub const fn with_scale(mut self, x: f32, y: f32) -> Self {
        self.scale = Vec2::new(x, y);
        self
    }

    /// Mutates the `x` / `y` scale factors.
    pub const fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = Vec2::new(x, y);
    }

    /// Builder: applies the same `scale` on both axes.
    #[must_use]
    pub const fn with_uniform_scale(mut self, scale: f32) -> Self {
        self.scale = Vec2::splat(scale);
        self
    }

    /// Builder: sets the RGBA color tint applied to the texture.
    #[must_use]
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Mutates the RGBA color tint.
    pub const fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Builder: sets the normalized anchor point used for rotation and
    /// positioning. `(0.5, 0.5)` is the center; `(0.0, 0.0)` is top-left.
    #[must_use]
    pub const fn with_anchor(mut self, x: f32, y: f32) -> Self {
        self.anchor = Vec2::new(x, y);
        self
    }

    /// Builder: flips the sprite horizontally and/or vertically.
    #[must_use]
    pub const fn with_flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }
}

/// A single vertex of a textured quad: 3D position and UV texture coordinates.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    /// Returns the wgpu [`VertexBufferLayout`] for a [`Vertex`].
    #[must_use]
    pub const fn desc() -> VertexBufferLayout<'static> {
        utils::render_utils::desc(
            &Self::ATTRIBS,
            size_of::<Self>() as BufferAddress,
            wgpu::VertexStepMode::Vertex,
        )
    }
}

pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
    },
];

pub const QUAD_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

use crate::graphics::color::Color;
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
    #[must_use]
    pub fn new(texture: Arc<wgpu::Texture>) -> Self {
        let w = texture.size().width as f32;
        let h = texture.size().height as f32;
        Self {
            texture,
            position: Vec2::ZERO,
            size: Vec2::new(w, h),
            rotation: 0.0,
            scale: Vec2::ONE,
            color: Color::WHITE,
            anchor: Vec2::new(0.5, 0.5),
            flip_x: false,
            flip_y: false,
        }
    }

    #[must_use]
    pub const fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = Vec2::new(x, y);
        self
    }

    pub const fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);
    }

    #[must_use]
    pub const fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = angle;
        self
    }

    pub const fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }

    #[must_use]
    pub const fn with_scale(mut self, x: f32, y: f32) -> Self {
        self.scale = Vec2::new(x, y);
        self
    }

    pub const fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = Vec2::new(x, y);
    }

    #[must_use]
    pub const fn with_uniform_scale(mut self, scale: f32) -> Self {
        self.scale = Vec2::splat(scale);
        self
    }

    #[must_use]
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub const fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[must_use]
    pub const fn with_anchor(mut self, x: f32, y: f32) -> Self {
        self.anchor = Vec2::new(x, y);
        self
    }

    #[must_use]
    pub const fn with_flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    #[must_use]
    pub const fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
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

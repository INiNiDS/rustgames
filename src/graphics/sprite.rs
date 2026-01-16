
use std::sync::Arc;
use glam::Vec2;
use wgpu::{BufferAddress, VertexBufferLayout, VertexFormat};
use crate::graphics::color::Color;

pub struct Sprite {
    pub texture: Arc<wgpu::Texture>,
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Color,
    pub anchor: Vec2
}

impl Sprite {
    pub fn new(texture: Arc<wgpu::Texture>) -> Self {
        let w = texture.size().width as f32;
        let h = texture.size().height as f32;
        Self {
            texture,
            position: Vec2::ZERO,
            size: Vec2::new(w, h),
            rotation: 0.0,
            color: Color::WHITE,
            anchor: Vec2::new(0.5, 0.5),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}


impl Vertex {
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.0, 0.0] },
];

pub const QUAD_INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];
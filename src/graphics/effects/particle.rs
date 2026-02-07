use glam::Vec2;
use crate::graphics::Color;

/// A single particle with position, velocity, remaining lifetime, colour, and
/// size. Updated each frame by its owning `EffectInstance`.
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub color: Color,
    pub size: f32,
}

impl Particle {
    #[inline]
    #[must_use]
    pub const fn new(position: Vec2, velocity: Vec2, lifetime: f32, color: Color, size: f32) -> Self {
        Self {
            position,
            velocity,
            lifetime,
            color,
            size,
        }
    }

    #[inline]
    pub fn update(&mut self, delta_time: f32, gravity: Vec2) {
        self.velocity += gravity * delta_time;
        self.position += self.velocity * delta_time;
        self.lifetime -= delta_time;
    }

    #[inline]
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    #[inline]
    #[must_use]
    pub fn alpha(&self, total_lifetime: f32) -> f32 {
        (self.lifetime / total_lifetime).clamp(0.0, 1.0) * self.color.a
    }
}
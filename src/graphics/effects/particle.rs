use crate::graphics::Color;
use glam::Vec2;

/// A single particle with position, velocity, remaining lifetime, color, and
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
    /// Creates a new [`Particle`] with explicit position, velocity, lifetime,
    /// color, and pixel size.
    #[inline]
    #[must_use]
    pub const fn new(
        position: Vec2,
        velocity: Vec2,
        lifetime: f32,
        color: Color,
        size: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            lifetime,
            color,
            size,
        }
    }

    /// Advances the particle by `delta_time` seconds: applies gravity,
    /// integrates velocity, and decrements remaining lifetime.
    #[inline]
    pub fn update(&mut self, delta_time: f32, gravity: Vec2) {
        self.velocity += gravity * delta_time;
        self.position += self.velocity * delta_time;
        self.lifetime -= delta_time;
    }

    /// Returns `true` while the particle still has remaining lifetime.
    #[inline]
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    /// Returns the current alpha (0.0–1.0) for this particle, linearly
    /// interpolated from the original color alpha based on remaining lifetime.
    #[inline]
    #[must_use]
    pub fn alpha(&self, total_lifetime: f32) -> f32 {
        (self.lifetime / total_lifetime).clamp(0.0, 1.0) * self.color.a
    }
}

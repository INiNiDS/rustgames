use glam::Vec2;
use crate::graphics::{Color, SpriteInstance};

/// A visual effect that can be submitted to the `EffectManager` or
/// `RendererAlpha`.
#[derive(Debug, Clone)]
pub enum VfxEffect {
    Flash { color: Color, duration: f32 },
    Emitter(EmitterConfig),
    Vignette { intensity: f32 },
    Overlay { color: Color, alpha: f32 },
}

/// Configuration for a particle effect. Use the named constructors such as
/// `sparkles`, `explosion`, `snow`, `rain`, or `smoke` for common presets.
#[derive(Debug, Clone)]
pub struct EmitterConfig {
    pub position: Vec2,
    pub count: u32,
    pub lifetime: f32,
    pub velocity_min: Vec2,
    pub velocity_max: Vec2,
    pub color: Color,
    pub size: f32,
    pub gravity: Vec2,
}

impl EmitterConfig {
    #[must_use]
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            count: 10,
            lifetime: 1.0,
            velocity_min: Vec2::splat(-50.0),
            velocity_max: Vec2::splat(50.0),
            color: Color::WHITE,
            size: 5.0,
            gravity: Vec2::new(0.0, 98.0),
        }
    }

    #[must_use]
    pub fn sparkles(position: Vec2) -> Self {
        Self {
            position,
            count: 20,
            lifetime: 0.5,
            velocity_min: Vec2::splat(-100.0),
            velocity_max: Vec2::splat(100.0),
            color: Color::YELLOW,
            size: 3.0,
            gravity: Vec2::ZERO,
        }
    }

    #[must_use]
    pub fn explosion(position: Vec2) -> Self {
        Self {
            position,
            count: 50,
            lifetime: 1.0,
            velocity_min: Vec2::splat(-200.0),
            velocity_max: Vec2::splat(200.0),
            color: Color::from((1.0, 0.5, 0.0)),
            size: 8.0,
            gravity: Vec2::new(0.0, 50.0),
        }
    }

    #[must_use]
    pub fn snow(position: Vec2, _width: f32) -> Self {
        Self {
            position,
            count: 100,
            lifetime: 5.0,
            velocity_min: Vec2::new(-10.0, 30.0),
            velocity_max: Vec2::new(10.0, 60.0),
            color: Color::WHITE,
            size: 4.0,
            gravity: Vec2::new(0.0, 20.0),
        }
    }

    #[must_use]
    pub fn rain(position: Vec2) -> Self {
        Self {
            position,
            count: 200,
            lifetime: 2.0,
            velocity_min: Vec2::new(-5.0, 300.0),
            velocity_max: Vec2::new(5.0, 400.0),
            color: Color::from((0.5, 0.5, 1.0, 0.6)),
            size: 2.0,
            gravity: Vec2::new(0.0, 200.0),
        }
    }

    #[must_use]
    pub fn smoke(position: Vec2) -> Self {
        Self {
            position,
            count: 30,
            lifetime: 2.0,
            velocity_min: Vec2::new(-20.0, -100.0),
            velocity_max: Vec2::new(20.0, -50.0),
            color: Color::from((0.5, 0.5, 0.5, 0.5)),
            size: 10.0,
            gravity: Vec2::new(0.0, -10.0),
        }
    }
}


/// Tracks the current state of a screen-wide colour flash.
#[derive(Debug, Clone)]
pub struct Flash {
    pub active: bool,
    pub(crate) color: Color,
    pub(crate) remaining: f32,
    pub(crate) duration: f32,
}

impl Default for Flash {
    fn default() -> Self {
        Self {
            active: false,
            color: Color::WHITE,
            remaining: 0.0,
            duration: 0.0,
        }
    }
}

impl Flash {
    /// Returns the current alpha for the flash, fading linearly to zero.
    #[must_use]
    pub fn alpha(&self) -> f32 {
        if self.duration <= 0.0 {
            return 0.0;
        }
        (self.remaining / self.duration).clamp(0.0, 1.0)
    }
}

/// Tracks a persistent colour overlay applied over the scene.
#[derive(Debug, Clone)]
pub struct Overlay {
    pub active: bool,
    pub(crate) color: Color,
    pub alpha: f32,
}

impl Default for Overlay {
    fn default() -> Self {
        Self {
            active: false,
            color: Color::TRANSPARENT,
            alpha: 0.0,
        }
    }
}

/// A snapshot of the effects state at a point in time, suitable for external
/// rendering integration.
#[derive(Debug, Clone)]
pub struct VfxFrame {
    pub flash_color: Option<Color>,
    pub overlay_color: Option<Color>,
    pub particle: Vec<SpriteInstance>,
}

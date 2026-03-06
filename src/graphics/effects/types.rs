use crate::graphics::{Color, SpriteInstance};
use glam::Vec2;

/// A visual effect that can be submitted to the `EffectManager` or
/// `RendererAlpha`.
#[derive(Debug, Clone)]
pub enum VfxEffect {
    /// A full-screen color flash that fades out over `duration` seconds.
    Flash { color: Color, duration: f32 },
    /// A particle burst defined by an [`EmitterConfig`].
    Emitter(EmitterConfig),
    /// A darkened vignette around the screen edges at the given `intensity`.
    Vignette { intensity: f32 },
    /// A persistent semi-transparent color layer drawn over the scene.
    Overlay { color: Color, alpha: f32 },
}

/// Configuration for a particle effect. Use the named constructors such as
/// [`sparkles`][EmitterConfig::sparkles], [`explosion`][EmitterConfig::explosion],
/// [`snow`][EmitterConfig::snow], [`rain`][EmitterConfig::rain], or
/// [`smoke`][EmitterConfig::smoke] for common presets.
#[derive(Debug, Clone)]
pub struct EmitterConfig {
    /// World-space origin of the emitter.
    pub position: Vec2,
    /// Number of particles spawned in a single burst.
    pub count: u32,
    /// Time in seconds before each particle is considered dead.
    pub lifetime: f32,
    /// Minimum random velocity applied to each particle (px/s).
    pub velocity_min: Vec2,
    /// Maximum random velocity applied to each particle (px/s).
    pub velocity_max: Vec2,
    /// Starting color of every particle.
    pub color: Color,
    /// Radius of each particle in pixels.
    pub size: f32,
    /// Gravity acceleration added to particle velocity each second (px/s²).
    pub gravity: Vec2,
}

impl EmitterConfig {
    /// Creates a generic emitter at `position` with sensible defaults:
    /// 10 particles, 1 s lifetime, ±50 px/s velocity, white, 5 px, gravity 98 px/s².
    #[must_use]
    pub const fn new(position: Vec2) -> Self {
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

    /// Preset: small yellow sparks radiating outward with no gravity.
    #[must_use]
    pub const fn sparkles(position: Vec2) -> Self {
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

    /// Preset: fiery orange burst with heavy outward velocity.
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

    /// Preset: slow white snowflakes drifting downward.
    #[must_use]
    pub const fn snow(position: Vec2, _width: f32) -> Self {
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

    /// Preset: semi-transparent blue streaks falling fast.
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

    /// Preset: gray semi-transparent puffs rising upward.
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

/// Tracks the current state of a screen-wide color flash.
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

/// Tracks a persistent color overlay applied over the scene.
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

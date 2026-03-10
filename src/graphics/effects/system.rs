pub use crate::graphics::Particle;
pub use crate::graphics::effects::{EmitterConfig, VfxEffect};
use glam::Vec2;
use rand::RngExt;

/// An individual effect that is currently running, together with its
/// elapsed time and any live particles it owns.
#[derive(Debug)]
pub struct ActiveEffect {
    /// The effect configuration driving this instance.
    pub config: VfxEffect,
    /// Time in seconds since this effect was spawned.
    pub elapsed: f32,
    /// Live particles owned by this effect (only populated for `Emitter`).
    pub particles: Vec<Particle>,
}

impl ActiveEffect {
    /// Creates a new [`ActiveEffect`] from `effect`.
    /// If `effect` is an `Emitter`, the initial particle burst is spawned
    /// immediately.
    #[must_use]
    pub fn new(effect: VfxEffect) -> Self {
        let emitter_cfg = if let VfxEffect::Emitter(ref cfg) = effect {
            Some(cfg.clone())
        } else {
            None
        };

        let mut instance = Self {
            config: effect,
            elapsed: 0.0,
            particles: Vec::new(),
        };

        if let Some(cfg) = emitter_cfg {
            instance.spawn_burst(&cfg);
        }

        instance
    }

    /// Returns the total duration of this effect in seconds.
    /// Returns `f32::INFINITY` for persistent effects (vignette, overlay).
    #[must_use]
    pub const fn duration(&self) -> f32 {
        match &self.config {
            VfxEffect::Flash { duration, .. } => *duration,
            VfxEffect::Emitter(cfg) => cfg.lifetime,
            VfxEffect::Vignette { .. } | VfxEffect::Overlay { .. } => f32::INFINITY,
        }
    }

    /// Advances the effect by `delta_time` seconds, updating particles if
    /// this is an `Emitter`.
    pub fn update(&mut self, delta_time: f32) {
        self.elapsed += delta_time;

        if let VfxEffect::Emitter(ref cfg) = self.config {
            for p in &mut self.particles {
                p.update(delta_time, cfg.gravity);
            }

            self.particles.retain(Particle::is_alive);
        }
    }

    /// Returns `true` when the effect has exceeded its duration and all
    /// particles have expired.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        let duration = self.duration();
        if duration.is_infinite() {
            false
        } else {
            self.elapsed >= duration && self.particles.is_empty()
        }
    }

    fn spawn_burst(&mut self, cfg: &EmitterConfig) {
        let mut rng = rand::rng();
        self.particles.reserve(cfg.count as usize);

        for _ in 0..cfg.count {
            let velocity = Vec2::new(
                rng.random_range(cfg.velocity_min.x..cfg.velocity_max.x),
                rng.random_range(cfg.velocity_min.y..cfg.velocity_max.y),
            );

            let particle = Particle::new(cfg.position, velocity, cfg.lifetime, cfg.color, cfg.size);

            self.particles.push(particle);
        }
    }
}

// so its need to be all public, only VfxSystem is need to be pub(crate)

/// Manages active `EffectInstance` values, advancing and pruning them each
/// frame.
pub struct VfxSystem {
    effects: Vec<ActiveEffect>,
}

impl VfxSystem {
    /// Creates a new, empty [`VfxSystem`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    /// Adds a new effect to the system. It begins updating immediately.
    pub fn push(&mut self, effect: VfxEffect) {
        self.effects.push(ActiveEffect::new(effect));
    }

    /// Advances all active effects by `delta_time` seconds and removes any
    /// that have finished.
    pub fn update(&mut self, delta_time: f32) {
        for effect in &mut self.effects {
            effect.update(delta_time);
        }

        self.effects.retain(|e| !e.is_finished());
    }

    /// Removes all active effects.
    pub fn clear(&mut self) {
        self.effects.clear();
    }

    /// Returns a slice of all currently active effects.
    #[must_use]
    pub fn active_effects(&self) -> &[ActiveEffect] {
        &self.effects
    }

    /// Returns the number of currently active effects.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.effects.len()
    }
}

impl Default for VfxSystem {
    fn default() -> Self {
        Self::new()
    }
}

pub use crate::graphics::effects::{EmitterConfig, VfxEffect};
pub use crate::graphics::Particle;
use glam::Vec2;
use rand::Rng;

#[derive(Debug)]
pub struct ActiveEffect {
    pub config: VfxEffect,
    pub elapsed: f32,
    pub particles: Vec<Particle>,
}

impl ActiveEffect {
    #[must_use] 
    pub fn new(effect: VfxEffect) -> Self {
        let mut instance = Self {
            config: effect.clone(),
            elapsed: 0.0,
            particles: Vec::new(),
        };
        
        if let VfxEffect::Emitter(ref cfg) = effect {
            instance.spawn_burst(cfg);
        }
        
        instance
    }
    
    #[must_use] 
    pub const fn duration(&self) -> f32 {
        match &self.config {
            VfxEffect::Flash { duration, .. } => *duration,
            VfxEffect::Emitter(cfg) => cfg.lifetime,
            VfxEffect::Vignette { .. } | VfxEffect::Overlay { .. }  => f32::INFINITY,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.elapsed += delta_time;
        
        if let VfxEffect::Emitter(ref cfg) = self.config {
            for p in &mut self.particles {
                p.update(delta_time, cfg.gravity);
            }
            
            self.particles.retain(Particle::is_alive);
        }
    }
    
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

            let particle = Particle::new(
                cfg.position,
                velocity,
                cfg.lifetime,
                cfg.color,
                cfg.size,
            );

            self.particles.push(particle);
        }
    }
}

/// Manages active `EffectInstance` values, advancing and pruning them each
/// frame.
pub struct VfxSystem {
    effects: Vec<ActiveEffect>,
}

impl VfxSystem {
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
    
    pub fn push(&mut self, effect: VfxEffect) {
        self.effects.push(ActiveEffect::new(effect));
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for effect in &mut self.effects {
            effect.update(delta_time);
        }
        
        self.effects.retain(|e| !e.is_finished());
    }
    
    pub fn clear(&mut self) {
        self.effects.clear();
    }
    
    #[must_use] 
    pub fn active_effects(&self) -> &[ActiveEffect] {
        &self.effects
    }
    
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


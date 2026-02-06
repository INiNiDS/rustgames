use glam::Vec2;
use rand::Rng;
use crate::graphics::color::Color;

/// A visual effect that can be submitted to the `EffectManager` or
/// `RendererAlpha`.
#[derive(Debug, Clone)]
pub enum VisualEffect {
    Flash { color: Color, duration: f32 },
    ScreenShake { intensity: f32, duration: f32 },
    Particles(ParticleEffect),
    Vignette { intensity: f32 },
    ColorOverlay { color: Color, alpha: f32 },
}

/// Configuration for a particle effect. Use the named constructors such as
/// `sparkles`, `explosion`, `snow`, `rain`, or `smoke` for common presets.
#[derive(Debug, Clone)]
pub struct ParticleEffect {
    pub position: Vec2,
    pub particle_count: u32,
    pub lifetime: f32,
    pub velocity_min: Vec2,
    pub velocity_max: Vec2,
    pub color: Color,
    pub size: f32,
    pub gravity: Vec2,
}

impl ParticleEffect {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            particle_count: 10,
            lifetime: 1.0,
            velocity_min: Vec2::new(-50.0, -50.0),
            velocity_max: Vec2::new(50.0, 50.0),
            color: Color::WHITE,
            size: 5.0,
            gravity: Vec2::new(0.0, 98.0),
        }
    }
    
    pub fn sparkles(position: Vec2) -> Self {
        Self {
            position,
            particle_count: 20,
            lifetime: 0.5,
            velocity_min: Vec2::new(-100.0, -100.0),
            velocity_max: Vec2::new(100.0, 100.0),
            color: Color::YELLOW,
            size: 3.0,
            gravity: Vec2::ZERO,
        }
    }
    
    pub fn explosion(position: Vec2) -> Self {
        Self {
            position,
            particle_count: 50,
            lifetime: 1.0,
            velocity_min: Vec2::new(-200.0, -200.0),
            velocity_max: Vec2::new(200.0, 200.0),
            color: Color::from((1.0, 0.5, 0.0)),
            size: 8.0,
            gravity: Vec2::new(0.0, 50.0),
        }
    }
    
    pub fn snow(position: Vec2, _width: f32) -> Self {
        Self {
            position,
            particle_count: 100,
            lifetime: 5.0,
            velocity_min: Vec2::new(-10.0, 30.0),
            velocity_max: Vec2::new(10.0, 60.0),
            color: Color::WHITE,
            size: 4.0,
            gravity: Vec2::new(0.0, 20.0),
        }
    }
    
    pub fn rain(position: Vec2) -> Self {
        Self {
            position,
            particle_count: 200,
            lifetime: 2.0,
            velocity_min: Vec2::new(-5.0, 300.0),
            velocity_max: Vec2::new(5.0, 400.0),
            color: Color::from((0.5, 0.5, 1.0, 0.6)),
            size: 2.0,
            gravity: Vec2::new(0.0, 200.0),
        }
    }
    
    pub fn smoke(position: Vec2) -> Self {
        Self {
            position,
            particle_count: 30,
            lifetime: 2.0,
            velocity_min: Vec2::new(-20.0, -100.0),
            velocity_max: Vec2::new(20.0, -50.0),
            color: Color::from((0.5, 0.5, 0.5, 0.5)),
            size: 10.0,
            gravity: Vec2::new(0.0, -10.0),
        }
    }
}

/// A single particle with position, velocity, remaining lifetime, colour, and
/// size. Updated each frame by its owning `EffectInstance`.
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime_remaining: f32,
    pub color: Color,
    pub size: f32,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, lifetime: f32, color: Color, size: f32) -> Self {
        Self {
            position,
            velocity,
            lifetime_remaining: lifetime,
            color,
            size,
        }
    }
    
    pub fn update(&mut self, delta_time: f32, gravity: Vec2) {
        self.velocity += gravity * delta_time;
        self.position += self.velocity * delta_time;
        self.lifetime_remaining -= delta_time;
    }
    
    pub fn is_alive(&self) -> bool {
        self.lifetime_remaining > 0.0
    }
    
    pub fn alpha(&self, total_lifetime: f32) -> f32 {
        (self.lifetime_remaining / total_lifetime).clamp(0.0, 1.0) * self.color.a
    }
}

#[derive(Debug)]
pub struct EffectInstance {
    pub effect: VisualEffect,
    pub elapsed: f32,
    pub particles: Vec<Particle>,
}

impl EffectInstance {
    pub fn new(effect: VisualEffect) -> Self {
        let mut instance = Self {
            effect: effect.clone(),
            elapsed: 0.0,
            particles: Vec::new(),
        };
        
        if let VisualEffect::Particles(ref particle_effect) = effect {
            instance.spawn_particles(particle_effect);
        }
        
        instance
    }
    
    fn spawn_particles(&mut self, effect: &ParticleEffect) {
        let mut rng = rand::rng();
        
        for _ in 0..effect.particle_count {
            let velocity = Vec2::new(
                rng.random_range(effect.velocity_min.x..effect.velocity_max.x),
                rng.random_range(effect.velocity_min.y..effect.velocity_max.y),
            );
            
            let particle = Particle::new(
                effect.position,
                velocity,
                effect.lifetime,
                effect.color,
                effect.size,
            );
            
            self.particles.push(particle);
        }
    }
    
    pub fn duration(&self) -> f32 {
        match &self.effect {
            VisualEffect::Flash { duration, .. } => *duration,
            VisualEffect::ScreenShake { duration, .. } => *duration,
            VisualEffect::Particles(effect) => effect.lifetime,
            VisualEffect::Vignette { .. } => f32::INFINITY,
            VisualEffect::ColorOverlay { .. } => f32::INFINITY,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.elapsed += delta_time;
        
        if let VisualEffect::Particles(ref effect) = self.effect {
            for particle in &mut self.particles {
                particle.update(delta_time, effect.gravity);
            }
            
            self.particles.retain(|p| p.is_alive());
        }
    }
    
    pub fn is_complete(&self) -> bool {
        let duration = self.duration();
        if duration.is_infinite() {
            false
        } else {
            self.elapsed >= duration && self.particles.is_empty()
        }
    }
}

/// Manages active `EffectInstance` values, advancing and pruning them each
/// frame.
pub struct EffectManager {
    effects: Vec<EffectInstance>,
}

impl EffectManager {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
    
    pub fn add_effect(&mut self, effect: VisualEffect) {
        self.effects.push(EffectInstance::new(effect));
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for effect in &mut self.effects {
            effect.update(delta_time);
        }
        
        self.effects.retain(|e| !e.is_complete());
    }
    
    pub fn clear(&mut self) {
        self.effects.clear();
    }
    
    pub fn effects(&self) -> &[EffectInstance] {
        &self.effects
    }
    
    pub fn count(&self) -> usize {
        self.effects.len()
    }
}

impl Default for EffectManager {
    fn default() -> Self {
        Self::new()
    }
}


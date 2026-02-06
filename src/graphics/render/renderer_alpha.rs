/// An alternative renderer that integrates the effects system on top of the
/// standard rendering pipeline. It delegates sprite and text drawing to the
/// existing `Renderer` while applying active visual effects such as screen
/// flashes, color overlays, and particle emission to each frame.
///
/// # Design
///
/// `RendererAlpha` wraps `RenderSettings` without modifying the main
/// `Renderer`. All effect-related state lives here, keeping the original
/// rendering path untouched.
use crate::graphics::effects::effects::{EffectManager, VisualEffect};
use crate::graphics::color::Color;
use crate::graphics::render::instance::SpriteInstance;
use glam::{Vec2, Vec4};

/// Manages the lifecycle and rendering of visual effects layered on top of the
/// standard renderer output.
pub struct RendererAlpha {
    effect_manager: EffectManager,
    flash_state: FlashState,
    overlay_state: OverlayState,
    screen_shake_offset: Vec2,
}

/// Tracks the current state of a screen-wide colour flash.
#[derive(Debug, Clone)]
pub struct FlashState {
    pub active: bool,
    pub color: Color,
    pub remaining: f32,
    pub duration: f32,
}

impl Default for FlashState {
    fn default() -> Self {
        Self {
            active: false,
            color: Color::WHITE,
            remaining: 0.0,
            duration: 0.0,
        }
    }
}

impl FlashState {
    /// Returns the current alpha for the flash, fading linearly to zero.
    pub fn alpha(&self) -> f32 {
        if self.duration <= 0.0 {
            return 0.0;
        }
        (self.remaining / self.duration).clamp(0.0, 1.0)
    }
}

/// Tracks a persistent colour overlay applied over the scene.
#[derive(Debug, Clone)]
pub struct OverlayState {
    pub active: bool,
    pub color: Color,
    pub alpha: f32,
}

impl Default for OverlayState {
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
pub struct EffectFrame {
    pub flash_color: Option<Color>,
    pub overlay_color: Option<Color>,
    pub screen_shake_offset: Vec2,
    pub particle_instances: Vec<SpriteInstance>,
}

impl RendererAlpha {
    /// Creates a new `RendererAlpha` with no active effects.
    pub fn new() -> Self {
        Self {
            effect_manager: EffectManager::new(),
            flash_state: FlashState::default(),
            overlay_state: OverlayState::default(),
            screen_shake_offset: Vec2::ZERO,
        }
    }

    /// Submits a visual effect to be managed and rendered.
    pub fn add_effect(&mut self, effect: VisualEffect) {
        self.apply_effect_state(&effect);
        self.effect_manager.add_effect(effect);
    }

    /// Advances all active effects by `delta_time` seconds.
    pub fn update(&mut self, delta_time: f32) {
        self.effect_manager.update(delta_time);
        self.update_flash(delta_time);
        self.screen_shake_offset = self.compute_shake_offset();
    }

    /// Removes a persistent overlay effect.
    pub fn clear_overlay(&mut self) {
        self.overlay_state = OverlayState::default();
    }

    /// Removes all active effects and resets internal state.
    pub fn clear_all(&mut self) {
        self.effect_manager.clear();
        self.flash_state = FlashState::default();
        self.overlay_state = OverlayState::default();
        self.screen_shake_offset = Vec2::ZERO;
    }

    /// Returns the number of active effects.
    pub fn active_effect_count(&self) -> usize {
        self.effect_manager.count()
    }

    /// Returns the current screen-shake offset to be applied to the camera or
    /// view transform.
    pub fn screen_shake_offset(&self) -> Vec2 {
        self.screen_shake_offset
    }

    /// Returns a reference to the current flash state.
    pub fn flash_state(&self) -> &FlashState {
        &self.flash_state
    }

    /// Returns a reference to the current overlay state.
    pub fn overlay_state(&self) -> &OverlayState {
        &self.overlay_state
    }

    /// Produces an `EffectFrame` snapshot containing all data needed to render
    /// the current effects. Particle effects are converted to `SpriteInstance`
    /// values suitable for instanced drawing.
    pub fn build_effect_frame(&self) -> EffectFrame {
        let flash_color = if self.flash_state.active {
            Some(self.flash_state.color.with_alpha(self.flash_state.alpha()))
        } else {
            None
        };

        let overlay_color = if self.overlay_state.active {
            Some(self.overlay_state.color.with_alpha(self.overlay_state.alpha))
        } else {
            None
        };

        EffectFrame {
            flash_color,
            overlay_color,
            screen_shake_offset: self.screen_shake_offset,
            particle_instances: self.collect_particle_instances(),
        }
    }
    
    /// Returns a reference to the underlying `EffectManager`.
    pub fn effect_manager(&self) -> &EffectManager {
        &self.effect_manager
    }

    /// Returns a mutable reference to the underlying `EffectManager`.
    pub fn effect_manager_mut(&mut self) -> &mut EffectManager {
        &mut self.effect_manager
    }

    fn apply_effect_state(&mut self, effect: &VisualEffect) {
        match effect {
            VisualEffect::Flash { color, duration } => {
                self.flash_state = FlashState {
                    active: true,
                    color: *color,
                    remaining: *duration,
                    duration: *duration,
                };
            }
            VisualEffect::ColorOverlay { color, alpha } => {
                self.overlay_state = OverlayState {
                    active: true,
                    color: *color,
                    alpha: *alpha,
                };
            }
            _ => {}
        }
    }

    fn update_flash(&mut self, delta_time: f32) {
        if !self.flash_state.active {
            return;
        }
        self.flash_state.remaining -= delta_time;
        if self.flash_state.remaining <= 0.0 {
            self.flash_state.active = false;
            self.flash_state.remaining = 0.0;
        }
    }

    fn compute_shake_offset(&self) -> Vec2 {
        let mut offset = Vec2::ZERO;
        for inst in self.effect_manager.effects() {
            if let VisualEffect::ScreenShake { intensity, .. } = &inst.effect {
                let progress = 1.0 - (inst.elapsed / inst.duration()).clamp(0.0, 1.0);
                let angle = inst.elapsed * 37.0;
                offset += Vec2::new(
                    angle.sin() * intensity * progress,
                    (angle * 1.3).cos() * intensity * progress,
                );
            }
        }
        offset
    }


    fn collect_particle_instances(&self) -> Vec<SpriteInstance> {
        let mut instances = Vec::new();
        for inst in self.effect_manager.effects() {
            if let VisualEffect::Particles(ref pe) = inst.effect {
                instances.reserve(inst.particles.len());
                for particle in &inst.particles {
                    let alpha = particle.alpha(pe.lifetime);
                    let color_with_alpha = particle.color.with_alpha(alpha);
                    instances.push(SpriteInstance::new(
                        particle.position,
                        Vec2::splat(particle.size),
                        0.0,
                        Vec4::new(0.0, 0.0, 1.0, 1.0),
                        Vec4::from(color_with_alpha.to_array()),
                    ));
                }
            }
        }
        instances
    }
}

impl Default for RendererAlpha {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::effects::effects::ParticleEffect;

    #[test]
    fn test_renderer_alpha_new_is_empty() {
        let ra = RendererAlpha::new();
        assert_eq!(ra.active_effect_count(), 0);
        assert_eq!(ra.screen_shake_offset(), Vec2::ZERO);
        assert!(!ra.flash_state().active);
        assert!(!ra.overlay_state().active);
    }

    #[test]
    fn test_flash_effect_activates_and_decays() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::Flash {
            color: Color::RED,
            duration: 1.0,
        });
        assert!(ra.flash_state().active);
        assert!((ra.flash_state().alpha() - 1.0).abs() < 0.01);

        ra.update(0.5);
        assert!(ra.flash_state().active);
        assert!((ra.flash_state().alpha() - 0.5).abs() < 0.01);

        ra.update(0.6);
        assert!(!ra.flash_state().active);
    }

    #[test]
    fn test_color_overlay_persists() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::ColorOverlay {
            color: Color::BLUE,
            alpha: 0.5,
        });
        assert!(ra.overlay_state().active);
        assert_eq!(ra.overlay_state().alpha, 0.5);

        ra.update(10.0);
        assert!(ra.overlay_state().active);
    }

    #[test]
    fn test_clear_overlay() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::ColorOverlay {
            color: Color::GREEN,
            alpha: 0.8,
        });
        ra.clear_overlay();
        assert!(!ra.overlay_state().active);
    }

    #[test]
    fn test_clear_all_resets_state() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::Flash {
            color: Color::WHITE,
            duration: 2.0,
        });
        ra.add_effect(VisualEffect::ColorOverlay {
            color: Color::RED,
            alpha: 1.0,
        });
        ra.add_effect(VisualEffect::ScreenShake {
            intensity: 10.0,
            duration: 1.0,
        });
        ra.clear_all();
        assert_eq!(ra.active_effect_count(), 0);
        assert!(!ra.flash_state().active);
        assert!(!ra.overlay_state().active);
        assert_eq!(ra.screen_shake_offset(), Vec2::ZERO);
    }

    #[test]
    fn test_screen_shake_produces_offset() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::ScreenShake {
            intensity: 20.0,
            duration: 2.0,
        });
        ra.update(0.1);
        assert_ne!(ra.screen_shake_offset(), Vec2::ZERO);
    }

    #[test]
    fn test_particle_effect_generates_instances() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::Particles(ParticleEffect::sparkles(Vec2::ZERO)));
        let frame = ra.build_effect_frame();
        assert!(!frame.particle_instances.is_empty());
    }

    #[test]
    fn test_effect_frame_no_effects() {
        let ra = RendererAlpha::new();
        let frame = ra.build_effect_frame();
        assert!(frame.flash_color.is_none());
        assert!(frame.overlay_color.is_none());
        assert!(frame.particle_instances.is_empty());
        assert_eq!(frame.screen_shake_offset, Vec2::ZERO);
    }

    #[test]
    fn test_particles_decay_over_time() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::Particles(ParticleEffect::new(Vec2::ZERO)));
        let initial_count = ra.build_effect_frame().particle_instances.len();
        ra.update(2.0);
        let after_count = ra.build_effect_frame().particle_instances.len();
        assert!(after_count <= initial_count);
    }

    #[test]
    fn test_flash_alpha_at_boundaries() {
        let state = FlashState {
            active: true,
            color: Color::WHITE,
            remaining: 0.0,
            duration: 1.0,
        };
        assert_eq!(state.alpha(), 0.0);

        let state2 = FlashState {
            active: true,
            color: Color::WHITE,
            remaining: 1.0,
            duration: 1.0,
        };
        assert_eq!(state2.alpha(), 1.0);
    }

    #[test]
    fn test_flash_alpha_zero_duration() {
        let state = FlashState {
            active: true,
            color: Color::WHITE,
            remaining: 0.5,
            duration: 0.0,
        };
        assert_eq!(state.alpha(), 0.0);
    }
}

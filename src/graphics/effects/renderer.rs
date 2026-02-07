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
use crate::graphics::effects::system::{VfxSystem, VfxEffect};
use crate::graphics::color::Color;
use crate::graphics::render::instance::SpriteInstance;
use glam::{Vec2, Vec4};
use crate::graphics::effects::{ActiveEffect, Flash, Overlay, VfxFrame};
use crate::graphics::EmitterConfig;
use crate::prelude::Particle;

/// Manages the lifecycle and rendering of visual effects layered on top of the
/// standard renderer output.
pub struct VfxRenderer {
    system: VfxSystem,
    flash: Flash,
    overlay: Overlay,
}

impl VfxRenderer {
    /// Creates a new `RendererAlpha` with no active effects.
    #[must_use] 
    pub fn new() -> Self {
        Self {
            system: VfxSystem::new(),
            flash: Flash::default(),
            overlay: Overlay::default(),
        }
    }

    /// Submits a visual effect to be managed and rendered.
    pub fn add_effect(&mut self, effect: VfxEffect) {
        self.apply_global_state(&effect);
        self.system.push(effect);
    }

    /// Advances all active effects by `delta_time` seconds.
    pub fn update(&mut self, delta_time: f32) {
        self.system.update(delta_time);
        self.update_flash(delta_time);
    }

    /// Removes a persistent overlay effect.
    pub fn clear_overlay(&mut self) {
        self.overlay = Overlay::default();
    }

    /// Removes all active effects and resets internal state.
    pub fn clear_all(&mut self) {
        self.system.clear();
        self.flash = Flash::default();
        self.overlay = Overlay::default();
    }

    /// Produces an `EffectFrame` snapshot containing all data needed to render
    /// the current effects. Particle effects are converted to `SpriteInstance`
    /// values suitable for instanced drawing.
    #[must_use]
    pub fn build_frame(&self) -> VfxFrame {
        VfxFrame {
            flash_color: self.get_flash_color(),
            overlay_color: self.get_overlay_color(),
            particle: self.batch_particle(),
        }
    }

    /// Returns the number of active effects.
    #[must_use] 
    pub const fn active_effect_count(&self) -> usize {
        self.system.count()
    }

    /// Returns a reference to the current flash state.
    #[must_use] 
    pub const fn flash_state(&self) -> &Flash {
        &self.flash
    }

    /// Returns a reference to the current overlay state.
    #[must_use] 
    pub const fn overlay_state(&self) -> &Overlay {
        &self.overlay
    }
    
    /// Returns a reference to the underlying `EffectManager`.
    #[must_use] 
    pub const fn effect_manager(&self) -> &VfxSystem {
        &self.system
    }

    /// Returns a mutable reference to the underlying `EffectManager`.
    pub const fn effect_manager_mut(&mut self) -> &mut VfxSystem {
        &mut self.system
    }

    // -- Helpers --

    const fn apply_global_state(&mut self, effect: &VfxEffect) {
        match effect {
            VfxEffect::Flash { color, duration } => {
                self.flash = Flash {
                    active: true,
                    color: *color,
                    remaining: *duration,
                    duration: *duration,
                };
            }
            VfxEffect::Overlay { color, alpha } => {
                self.overlay = Overlay {
                    active: true,
                    color: *color,
                    alpha: *alpha,
                };
            }
            _ => {}
        }
    }

    fn update_flash(&mut self, delta_time: f32) {
        if !self.flash.active {
            return;
        }
        self.flash.remaining -= delta_time;
        if self.flash.remaining <= 0.0 {
            self.flash.active = false;
            self.flash.remaining = 0.0;
        }
    }

    fn batch_particle(&self) -> Vec<SpriteInstance> {
        let mut batch = Vec::with_capacity(self.system.count() * 10);

        for inst in self.system.active_effects() {
            batch.extend(self.batch_effect(inst));
        }
        batch
    }

    fn get_flash_color(&self) -> Option<Color> {
        if self.flash.active && self.flash.duration > 0.0 {
            let alpha = (self.flash.remaining / self.flash.duration).clamp(0.0, 1.0);
            Some(self.flash.color.with_alpha(alpha))
        } else {
            None
        }
    }

    const fn get_overlay_color(&self) -> Option<Color> {
        if self.overlay.active {
            Some(self.overlay.color.with_alpha(self.overlay.alpha))
        } else {
            None
        }
    }

    fn batch_effect(&self, inst: &ActiveEffect) -> Vec<SpriteInstance> {
        let mut batch = Vec::new();
        if let VfxEffect::Emitter(ref cfg) = inst.config {
            for p in &inst.particles {
                batch.push(self.batch_emitter(p, cfg));
            }
        }
        batch
    }

    fn batch_emitter(&self, p: &Particle, cfg: &EmitterConfig) -> SpriteInstance {
        let alpha = p.alpha(cfg.lifetime);
        let color_with_alpha = p.color.with_alpha(alpha);
        SpriteInstance::new(
            p.position,
            Vec2::splat(p.size),
            0.0,
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::from(color_with_alpha.to_array()),
        )
    }
}

impl Default for VfxRenderer {
    fn default() -> Self {
        Self::new()
    }
}
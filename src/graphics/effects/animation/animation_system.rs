use crate::graphics::effects::CustomCombinedMode;
use crate::prelude::{ActiveAnimation, AnimEffect, Animation, Easing, VisualState};
use glam::Vec2;

/// Manages animation instances: starting, stopping, pausing, seeking, and
/// evaluating combined effects on a `VisualState`.
pub struct AnimationSystem {
    pub(crate) animations: Vec<ActiveAnimation>,
    pub(crate) next_id: usize,
}

impl AnimationSystem {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            animations: Vec::new(),
            next_id: 0,
        }
    }

    pub fn start(&mut self, animation: Animation, easing: Easing, delay: f32) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.animations
            .push(ActiveAnimation::new(id, animation, easing, delay));
        id
    }

    pub fn stop(&mut self, id: usize) {
        self.animations.retain(|a| a.id != id);
    }

    pub fn update(&mut self, delta_time: f32) {
        for anim in &mut self.animations {
            anim.update(delta_time);
        }
        self.animations.retain(|anim| !anim.is_finished());
    }

    #[must_use]
    pub const fn is_playing(&self) -> bool {
        !self.animations.is_empty()
    }

    pub fn pause(&mut self, id: usize) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.paused = true;
            true
        } else {
            false
        }
    }

    pub fn resume(&mut self, id: usize) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.paused = false;
            true
        } else {
            false
        }
    }

    pub fn set_playback(&mut self, id: usize, speed: f32) -> bool {
        if speed == 0.0 {
            self.pause(id);
        }
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.playback = speed;
            true
        } else {
            false
        }
    }

    pub fn seek_time(&mut self, id: usize, time: f32) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.elapsed = time.clamp(0.0, anim.duration());
            true
        } else {
            false
        }
    }

    pub fn seek_progress(&mut self, id: usize, progress: f32) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.elapsed = progress * anim.duration();
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_playing_id(&self, id: usize) -> bool {
        self.animations.iter().any(|a| a.id == id)
    }

    #[must_use]
    pub const fn count(&self) -> usize {
        self.animations.len()
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }

    #[must_use]
    pub fn evaluate(
        &self,
        base: VisualState,
        size: Vec2,
        mode: Option<CustomCombinedMode>,
    ) -> VisualState {
        let mut combined = AnimEffect::default();
        for anim in &self.animations {
            combined = combined.combine(anim.effect(size));
        }
        combined.apply_to(base, mode)
    }

    pub(crate) fn spawn_instance(
        &mut self,
        anim: Animation,
        easing: Easing,
        delay: f32,
    ) -> (usize, f32) {
        let id = self.next_id;
        self.next_id += 1;
        let inst = ActiveAnimation::new(id, anim, easing, delay);
        let dur = inst.duration();
        self.animations.push(inst);
        (id, dur)
    }
}

impl Default for AnimationSystem {
    fn default() -> Self {
        Self::new()
    }
}

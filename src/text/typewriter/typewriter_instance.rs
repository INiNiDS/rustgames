use crate::prelude::{TextStyle, TypewriterEffect};
use crate::text::{PunctuationConfig, TextSpeed};
use std::slice::{Iter, IterMut};

/// Manages a collection of `TypewriterEffect` instances by ID.
pub struct TypewriterInstance {
    typewriter_effects: Vec<TypewriterEffect>,
    next_id: usize,
}

impl Default for TypewriterInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl TypewriterInstance {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            typewriter_effects: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_typewriter_effect(
        &mut self,
        text: impl Into<String>,
        speed: TextSpeed,
        x: f32,
        y: f32,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> usize {
        let effect =
            TypewriterEffect::new(text, speed, self.next_id, x, y, style, punctuation_config);
        self.typewriter_effects.push(effect);
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn update(&mut self, delta_time: f32) {
        for effect in &mut self.typewriter_effects {
            effect.update(delta_time);
        }
    }

    pub fn remove_typewriter_effect(&mut self, id: usize) {
        self.typewriter_effects.retain(|effect| effect.id != id);
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.typewriter_effects.is_empty()
    }

    pub fn get_typewriter_effects(&'_ self) -> Iter<'_, TypewriterEffect> {
        self.typewriter_effects.iter()
    }

    pub fn get_typewriter_effects_mut(&mut self) -> IterMut<'_, TypewriterEffect> {
        self.typewriter_effects.iter_mut()
    }

    #[must_use]
    pub fn get_effect(&self, id: usize) -> Option<&TypewriterEffect> {
        self.typewriter_effects.iter().find(|e| e.id == id)
    }

    pub fn get_effect_mut(&mut self, id: usize) -> Option<&mut TypewriterEffect> {
        self.typewriter_effects.iter_mut().find(|e| e.id == id)
    }

    pub fn skip_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.skip();
        }
    }

    pub fn pause_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.pause();
        }
    }

    pub fn resume_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.resume();
        }
    }

    pub fn reset_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.reset();
        }
    }

    pub fn set_effect_speed(&mut self, id: usize, speed: TextSpeed) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.set_speed(speed);
        }
    }

    #[must_use]
    pub fn get_text(&self, id: usize) -> Option<&str> {
        self.get_effect(id).map(TypewriterEffect::full_text)
    }

    #[must_use]
    pub fn get_visible_text(&self, id: usize) -> Option<&str> {
        self.get_effect(id).map(TypewriterEffect::visible_text)
    }

    #[must_use]
    pub fn get_position(&self, id: usize) -> Option<(f32, f32)> {
        self.get_effect(id).map(|e| (e.x, e.y))
    }

    #[must_use]
    pub fn is_paused(&self, id: usize) -> bool {
        self.get_effect(id).is_some_and(TypewriterEffect::is_paused)
    }

    #[must_use]
    pub fn is_complete(&self, id: usize) -> bool {
        self.get_effect(id)
            .is_some_and(TypewriterEffect::is_complete)
    }

    #[must_use]
    pub fn get_progress(&self, id: usize) -> f32 {
        self.get_effect(id).map_or(0.0, TypewriterEffect::progress)
    }

    pub fn clear(&mut self) {
        self.typewriter_effects.clear();
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.typewriter_effects.len()
    }

    #[must_use]
    pub fn set_text(
        &mut self,
        id: usize,
        text: impl Into<String>,
        speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> bool {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.set_text(text, speed, style, punctuation_config);
            true
        } else {
            false
        }
    }

    pub fn set_progress(&mut self, id: usize, progress: f32) -> bool {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.set_progress(progress);
            true
        } else {
            false
        }
    }
}

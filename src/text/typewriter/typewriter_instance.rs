use crate::prelude::{TextStyle, TypewriterEffect};
use crate::text::{PunctuationConfig, TextSpeed};
use std::slice::{Iter, IterMut};

/// Builder parameters for creating a [`TypewriterEffect`] via the translation system.
pub struct TypewriterBuilder {
    /// Fallback text used when no translation is found.
    pub text: String,
    /// Optional translation key ID; `None` means raw text (no lookup).
    pub text_id: Option<u32>,
    /// Reveal speed for the typewriter animation.
    pub speed: TextSpeed,
    /// Horizontal screen position in pixels.
    pub x: f32,
    /// Vertical screen position in pixels.
    pub y: f32,
    /// Text style applied to the rendered characters.
    pub style: TextStyle,
    /// Punctuation-based pause configuration.
    pub punctuation_config: PunctuationConfig,
}

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
    /// Creates a new, empty [`TypewriterInstance`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            typewriter_effects: Vec::new(),
            next_id: 0,
        }
    }

    /// Creates a raw-text typewriter effect at `(x, y)` and returns its ID.
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
            TypewriterEffect::new(text, speed,  x, y, style, punctuation_config);
        self.typewriter_effects.push(effect);
        self.next_id += 1;
        self.next_id - 1
    }

    /// Add a typewriter effect that will be resolved via translation system at render time.
    /// `text` is used as fallback when no translation is found.
    pub fn add_typewriter_effect_with_id(
        &mut self,
        typewriter_builder: TypewriterBuilder,
    ) -> usize {
        let effect = TypewriterEffect::new_with_id(
            typewriter_builder,
            self.next_id,
        );
        self.typewriter_effects.push(effect);
        self.next_id += 1;
        self.next_id - 1
    }

    /// Advances all effects by `delta_time` seconds.
    pub fn update(&mut self, delta_time: f32) {
        for effect in &mut self.typewriter_effects {
            effect.update(delta_time);
        }
    }

    /// Removes the effect with the given `id`.
    pub fn remove_typewriter_effect(&mut self, id: usize) {
        self.typewriter_effects.retain(|effect| effect.id != id);
    }

    /// Returns `true` when no effects are active.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.typewriter_effects.is_empty()
    }

    /// Returns an iterator over all active effects.
    pub fn get_typewriter_effects(&'_ self) -> Iter<'_, TypewriterEffect> {
        self.typewriter_effects.iter()
    }

    /// Returns a mutable iterator over all active effects.
    pub fn get_typewriter_effects_mut(&mut self) -> IterMut<'_, TypewriterEffect> {
        self.typewriter_effects.iter_mut()
    }

    /// Returns a reference to the effect with `id`, or `None`.
    #[must_use]
    pub fn get_effect(&self, id: usize) -> Option<&TypewriterEffect> {
        self.typewriter_effects.iter().find(|e| e.id == id)
    }

    /// Returns a mutable reference to the effect with `id`, or `None`.
    pub fn get_effect_mut(&mut self, id: usize) -> Option<&mut TypewriterEffect> {
        self.typewriter_effects.iter_mut().find(|e| e.id == id)
    }

    /// Instantly reveals all remaining characters in the effect with `id`.
    pub fn skip_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.skip();
        }
    }

    /// Pauses the effect with `id` at its current character position.
    pub fn pause_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.pause();
        }
    }

    /// Resumes the paused effect with `id`.
    pub fn resume_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.resume();
        }
    }

    /// Resets the effect with `id` back to the beginning.
    pub fn reset_effect(&mut self, id: usize) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.reset();
        }
    }

    /// Changes the reveal speed of the effect with `id`.
    pub fn set_effect_speed(&mut self, id: usize, speed: TextSpeed) {
        if let Some(effect) = self.get_effect_mut(id) {
            effect.set_speed(speed);
        }
    }

    /// Returns the full (unrevealed) text of the effect with `id`, or `None`.
    #[must_use]
    pub fn get_text(&self, id: usize) -> Option<&str> {
        self.get_effect(id).map(TypewriterEffect::full_text)
    }

    /// Returns the currently visible portion of the text for `id`, or `None`.
    #[must_use]
    pub fn get_visible_text(&self, id: usize) -> Option<&str> {
        self.get_effect(id).map(TypewriterEffect::visible_text)
    }

    /// Returns the `(x, y)` screen position of the effect with `id`, or `None`.
    #[must_use]
    pub fn get_position(&self, id: usize) -> Option<(f32, f32)> {
        self.get_effect(id).map(|e| (e.x, e.y))
    }

    /// Returns `true` if the effect with `id` is currently paused.
    #[must_use]
    pub fn is_paused(&self, id: usize) -> bool {
        self.get_effect(id).is_some_and(TypewriterEffect::is_paused)
    }

    /// Returns `true` when the effect with `id` has finished revealing.
    #[must_use]
    pub fn is_complete(&self, id: usize) -> bool {
        self.get_effect(id)
            .is_some_and(TypewriterEffect::is_complete)
    }

    /// Returns reveal progress (0.0–1.0) for `id`. Returns `0.0` if not found.
    #[must_use]
    pub fn get_progress(&self, id: usize) -> f32 {
        self.get_effect(id).map_or(0.0, TypewriterEffect::progress)
    }

    /// Removes all active effects.
    pub fn clear(&mut self) {
        self.typewriter_effects.clear();
    }

    /// Returns the number of active effects.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.typewriter_effects.len()
    }

    /// Replaces the text and style of the effect with `id`.
    /// Returns `true` if the effect was found.
    #[must_use]
    pub fn set_text(
        &mut self,
        id: usize,
        text: impl Into<String>,
        speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> bool {
        self.get_effect_mut(id).is_some_and(|effect| {
            effect.set_text(text, speed, style, punctuation_config);
            true
        })
    }

    /// Replaces text, translation key, and style of the effect with `id`.
    /// Returns `true` if the effect was found.
    #[must_use]
    pub fn set_text_with_id(
        &mut self,
        id: usize,
        text: impl Into<String>,
        text_id: u32,
        speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> bool {
        self.get_effect_mut(id).is_some_and(|effect| {
            effect.set_text_with_id(text, text_id, speed, style, punctuation_config);
            true
        })
    }

    /// Sets the reveal progress of effect `id`. Returns `true` if found.
    pub fn set_progress(&mut self, id: usize, progress: f32) -> bool {
        self.get_effect_mut(id).is_some_and(|effect| {
            effect.set_progress(progress);
            true
        })
    }
}

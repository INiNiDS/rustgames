use super::TextData;
use crate::prelude::{TextSpeed, TextStyle, TypewriterEffect};
use crate::text::typewriter::TypewriterBuilder;
use crate::text::{PunctuationConfig, TextSystem};
use std::slice::Iter;

impl TextSystem {
    /// Creates a new [`TypewriterEffect`] at `(x, y)` and returns its ID.
    pub fn add_text(
        &mut self,
        text: impl Into<String>,
        speed: TextSpeed,
        x: f32,
        y: f32,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> usize {
        self.typewriter_instance
            .add_typewriter_effect(text, speed, x, y, style, punctuation_config)
    }

    /// Add a typewriter effect resolved at render time via translation system.
    /// `text` is the fallback shown when no translation is found.
    /// `text_id` is generated via `Dictionary::generate_id_from_name(key)`.
    pub fn add_text_by_id(&mut self, text_data: TextData) -> usize {
        self.typewriter_instance
            .add_typewriter_effect_with_id(TypewriterBuilder {
                text: text_data.text,
                text_id: Some(text_data.text_id),
                speed: text_data.speed,
                x: text_data.x,
                y: text_data.y,
                style: text_data.style,
                punctuation_config: text_data.punctuation_config,
            })
    }

    /// Removes the typewriter effect with the given `id`.
    pub fn remove_text(&mut self, id: usize) {
        self.typewriter_instance.remove_typewriter_effect(id);
    }

    /// Advances all active typewriter effects by `delta_time` seconds.
    pub fn update(&mut self, delta_time: f32) {
        self.typewriter_instance.update(delta_time);
    }

    /// Instantly reveals all remaining characters in the effect with `id`.
    pub fn skip(&mut self, id: usize) {
        self.typewriter_instance.skip_effect(id);
    }

    /// Pauses the typewriter effect with `id` at its current character.
    pub fn pause(&mut self, id: usize) {
        self.typewriter_instance.pause_effect(id);
    }

    /// Resumes the paused typewriter effect with `id`.
    pub fn resume(&mut self, id: usize) {
        self.typewriter_instance.resume_effect(id);
    }

    /// Changes the playback speed of the typewriter effect with `id`.
    pub fn set_speed(&mut self, id: usize, speed: TextSpeed) {
        self.typewriter_instance.set_effect_speed(id, speed);
    }

    /// Replaces the text content and style of the effect with `id`.
    /// Returns `true` if the effect was found and updated.
    #[must_use]
    pub fn set_text(
        &mut self,
        id: usize,
        text: impl Into<String>,
        speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> bool {
        self.typewriter_instance
            .set_text(id, text, speed, style, punctuation_config)
    }

    /// Replaces text content, translation key, and style of the effect.
    /// Returns `true` if the effect was found and updated.
    #[must_use]
    pub fn set_text_by_id(
        &mut self,
        id: usize,
        text: impl Into<String>,
        text_id: u32,
        speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> bool {
        self.typewriter_instance.set_text_with_id(
            id,
            text,
            text_id,
            speed,
            style,
            punctuation_config,
        )
    }

    /// Sets the reveal progress of the effect with `id` to `progress`
    /// (0.0 = hidden, 1.0 = fully revealed).
    /// Returns `true` if the effect was found.
    #[must_use]
    pub fn set_progress(&mut self, id: usize, progress: f32) -> bool {
        self.typewriter_instance.set_progress(id, progress)
    }

    /// Returns the currently visible portion of the text in effect `id`,
    /// or `None` if the effect does not exist.
    #[must_use]
    pub fn get_visible_text(&self, id: usize) -> Option<&str> {
        self.typewriter_instance
            .get_effect(id)
            .map(TypewriterEffect::visible_text)
    }

    /// Returns `true` when the effect with `id` has revealed all its characters.
    #[must_use]
    pub fn is_complete(&self, id: usize) -> bool {
        self.typewriter_instance
            .get_effect(id)
            .is_some_and(TypewriterEffect::is_complete)
    }

    /// Returns reveal progress (0.0–1.0) for the effect with `id`.
    /// Returns `0.0` if the effect does not exist.
    #[must_use]
    pub fn get_progress(&self, id: usize) -> f32 {
        self.typewriter_instance
            .get_effect(id)
            .map_or(0.0, TypewriterEffect::progress)
    }

    /// Returns an iterator over all active [`TypewriterEffect`] instances.
    pub fn effects(&self) -> Iter<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects()
    }

    /// Returns a mutable iterator over all active [`TypewriterEffect`] instances.
    pub fn effects_mut(&mut self) -> std::slice::IterMut<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects_mut()
    }

    /// Returns a reference to the effect with `id`, or `None`.
    #[must_use]
    pub fn effect(&self, id: usize) -> Option<&TypewriterEffect> {
        self.typewriter_instance.get_effect(id)
    }

    /// Returns a mutable reference to the effect with `id`, or `None`.
    pub fn effect_mut(&mut self, id: usize) -> Option<&mut TypewriterEffect> {
        self.typewriter_instance.get_effect_mut(id)
    }

    /// Returns `true` when there are no active typewriter effects.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.typewriter_instance.is_empty()
    }
}

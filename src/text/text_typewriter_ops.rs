use super::TextData;
use crate::prelude::{TextSpeed, TextStyle, TypewriterEffect};
use crate::text::{PunctuationConfig, TextSystem};
use std::slice::Iter;

impl TextSystem {
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
        self.typewriter_instance.add_typewriter_effect_with_id(
            text_data.text,
            text_data.text_id,
            text_data.speed,
            text_data.x,
            text_data.y,
            text_data.style,
            text_data.punctuation_config,
        )
    }

    pub fn remove_text(&mut self, id: usize) {
        self.typewriter_instance.remove_typewriter_effect(id);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.typewriter_instance.update(delta_time);
    }

    pub fn skip(&mut self, id: usize) {
        self.typewriter_instance.skip_effect(id);
    }

    pub fn pause(&mut self, id: usize) {
        self.typewriter_instance.pause_effect(id);
    }

    pub fn resume(&mut self, id: usize) {
        self.typewriter_instance.resume_effect(id);
    }

    pub fn set_speed(&mut self, id: usize, speed: TextSpeed) {
        self.typewriter_instance.set_effect_speed(id, speed);
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
        self.typewriter_instance
            .set_text(id, text, speed, style, punctuation_config)
    }

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

    #[must_use]
    pub fn set_progress(&mut self, id: usize, progress: f32) -> bool {
        self.typewriter_instance.set_progress(id, progress)
    }

    #[must_use]
    pub fn get_visible_text(&self, id: usize) -> Option<&str> {
        self.typewriter_instance
            .get_effect(id)
            .map(TypewriterEffect::visible_text)
    }

    #[must_use]
    pub fn is_complete(&self, id: usize) -> bool {
        self.typewriter_instance
            .get_effect(id)
            .is_some_and(TypewriterEffect::is_complete)
    }

    #[must_use]
    pub fn get_progress(&self, id: usize) -> f64 {
        self.typewriter_instance
            .get_effect(id)
            .map_or(0.0, TypewriterEffect::progress)
    }

    pub fn effects(&self) -> Iter<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects()
    }

    pub fn effects_mut(&mut self) -> std::slice::IterMut<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects_mut()
    }

    #[must_use]
    pub fn effect(&self, id: usize) -> Option<&TypewriterEffect> {
        self.typewriter_instance.get_effect(id)
    }

    pub fn effect_mut(&mut self, id: usize) -> Option<&mut TypewriterEffect> {
        self.typewriter_instance.get_effect_mut(id)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.typewriter_instance.is_empty()
    }
}

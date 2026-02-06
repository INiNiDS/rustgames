use std::slice::Iter;
use crate::text::{TextSpeed, TypewriterEffect};
use crate::text::typewriter::TypewriterInstance;

/// Controls typewriter text-reveal effects with per-effect speed, pausing, and
/// completion tracking.
pub struct TypewriterController {
    typewriter_instance: TypewriterInstance
}


impl TypewriterController {
    pub fn new(typewriter_instance: TypewriterInstance) -> TypewriterController {
        Self { typewriter_instance  }
    }

    pub fn add_effect(&mut self, text: impl Into<String>, speed: TextSpeed, x: f32, y: f32) -> usize {
        self.typewriter_instance.add_typewriter_effect(text, speed, x, y)
    }

    pub fn remove_effect(&mut self, id: usize) {
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


    pub fn get_visible_text(&self, id: usize) -> Option<&str> {
        self.typewriter_instance.get_effect(id).map(|e| e.visible_text())
    }

    pub fn is_complete(&self, id: usize) -> bool {
        self.typewriter_instance.get_effect(id).is_some_and(|e| e.is_complete())
    }

    pub fn get_progress(&self, id: usize) -> f32 {
        self.typewriter_instance.get_effect(id).map_or(0.0, |e| e.progress())
    }

    pub fn effects(&self) -> Iter<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects()
    }

    pub fn effects_mut(&mut self) -> std::slice::IterMut<'_, TypewriterEffect> {
        self.typewriter_instance.get_typewriter_effects_mut()
    }

    pub fn effect(&self, id: usize) -> Option<&TypewriterEffect> {
        self.typewriter_instance.get_effect(id)
    }

    pub fn effect_mut(&mut self, id: usize) -> Option<&mut TypewriterEffect> {
        self.typewriter_instance.get_effect_mut(id)
    }

    pub fn is_empty(&self) -> bool {
        self.typewriter_instance.is_empty()
    }
}

use std::slice::{Iter, IterMut};

/// The speed at which a `TypewriterEffect` reveals characters.
#[derive(Debug, Clone, Copy)]
pub enum TextSpeed {
    Slow,       
    Medium,    
    Fast,       
    Instant,   
    Custom(f32), 
}

impl TextSpeed {
    #[must_use] 
    pub const fn chars_per_second(&self) -> f32 {
        match self {
            Self::Slow => 20.0,
            Self::Medium => 40.0,
            Self::Fast => 80.0,
            Self::Instant => f32::INFINITY,
            Self::Custom(speed) => *speed,
        }
    }
}

/// A character-by-character text reveal with configurable speed and automatic
/// punctuation pauses.
#[derive(Debug)]
pub struct TypewriterEffect {
    text: String,
    visible_chars: usize,
    chars_per_second: f32,
    elapsed: f32,
    paused: bool,
    complete: bool,
    pub(crate) id: usize,
    pub x: f32,
    pub y: f32,
    pause_timer: f32
}

impl TypewriterEffect {
    pub fn new(text: impl Into<String>, speed: TextSpeed, id: usize, x: f32, y: f32) -> Self {
        let text = text.into();
        let chars_per_second = speed.chars_per_second();
        let complete = chars_per_second.is_infinite();
        let visible_chars = if complete { text.chars().count() } else { 0 };
        
        Self {
            text,
            visible_chars,
            chars_per_second,
            elapsed: 0.0,
            paused: false,
            complete,
            id,
            x,
            y,
            pause_timer: 0.0
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.complete { return; }

        if self.pause_timer > 0.0 {
            self.pause_timer -= delta_time;
            return;
        }

        let seconds_per_char = 1.0 / self.chars_per_second;
        self.elapsed += delta_time;

        while self.elapsed >= seconds_per_char {
            self.elapsed -= seconds_per_char;
            self.visible_chars += 1;

            if let Some(c) = self.text.chars().nth(self.visible_chars - 1) {
                match c {
                    '.' | '!' | '?' => {
                        self.pause_timer = 0.5;
                        return;
                    },
                    ',' => {
                        self.pause_timer = 0.2;
                        return;
                    },
                    _ => {}
                }
            }

            if self.visible_chars >= self.text.chars().count() {
                self.complete = true;
                return;
            }
        }
    }
    
    #[must_use] 
    pub fn visible_text(&self) -> &str {
        if self.complete {
            &self.text
        } else {
            let byte_index = self.text.char_indices()
                .nth(self.visible_chars)
                .map_or(self.text.len(), |(i, _)| i);
            &self.text[..byte_index]
        }
    }
    
    #[must_use] 
    pub const fn is_complete(&self) -> bool {
        self.complete
    }
    
    pub fn skip(&mut self) {
        self.visible_chars = self.text.chars().count();
        self.complete = true;
        self.paused = false;
    }
    
    pub const fn reset(&mut self) {
        self.visible_chars = 0;
        self.elapsed = 0.0;
        self.complete = false;
        self.paused = false;
    }
    
    pub const fn pause(&mut self) {
        self.paused = true;
    }
    
    pub const fn resume(&mut self) {
        self.paused = false;
    }
    
    #[must_use] 
    pub const fn is_paused(&self) -> bool {
        self.paused
    }
    
    pub fn set_speed(&mut self, speed: TextSpeed) {
        self.chars_per_second = speed.chars_per_second();
        if self.chars_per_second.is_infinite() {
            self.skip();
        }
    }
    
    #[must_use] 
    pub fn full_text(&self) -> &str {
        &self.text
    }
    
    #[must_use] 
    pub fn progress(&self) -> f32 {
        let total = self.text.chars().count();
        if total == 0 {
            1.0
        } else {
            (self.visible_chars as f32) / (total as f32)
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>, new_speed: TextSpeed) {
        self.text = text.into();
        self.set_speed(new_speed);
    }
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
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            typewriter_effects: Vec::new(),
            next_id: 0
        }
    }


    pub fn add_typewriter_effect(&mut self, text: impl Into<String>, speed: TextSpeed, x: f32, y: f32) -> usize {
        let effect = TypewriterEffect::new(text, speed, self.next_id, x, y);
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
        self.get_effect(id).is_some_and(TypewriterEffect::is_complete)
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
}


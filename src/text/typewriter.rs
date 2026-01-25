use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub enum TextSpeed {
    Slow,       
    Medium,    
    Fast,       
    Instant,   
    Custom(f32), 
}

impl TextSpeed {
    pub fn chars_per_second(&self) -> f32 {
        match self {
            TextSpeed::Slow => 20.0,
            TextSpeed::Medium => 40.0,
            TextSpeed::Fast => 80.0,
            TextSpeed::Instant => f32::INFINITY,
            TextSpeed::Custom(speed) => *speed,
        }
    }
}

pub struct TypewriterEffect {
    text: String,
    visible_chars: usize,
    chars_per_second: f32,
    elapsed: f32,
    paused: bool,
    complete: bool,
    id: usize
}

impl TypewriterEffect {
    pub fn new(text: impl Into<String>, speed: TextSpeed, id: usize) -> Self {
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
            id
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if self.complete || self.paused {
            return;
        }
        
        self.elapsed += delta_time;
        
        let total_chars = self.text.chars().count();
        let target_chars = (self.elapsed * self.chars_per_second) as usize;
        
        self.visible_chars = target_chars.min(total_chars);
        
        if self.visible_chars >= total_chars {
            self.complete = true;
        }
        
        if self.visible_chars < total_chars {
            let current_char = self.text.chars().nth(self.visible_chars);
            if let Some(c) = current_char {
                if matches!(c, '.' | '!' | '?' | ',') {
                    self.paused = true;
                }
            }
        }
        
        if self.paused {
            std::thread::sleep(std::time::Duration::from_secs_f32(0.2));
            self.paused = false;
        }
    }
    
    pub fn visible_text(&self) -> &str {
        if self.complete {
            &self.text
        } else {
            let byte_index = self.text.char_indices()
                .nth(self.visible_chars)
                .map(|(i, _)| i)
                .unwrap_or(self.text.len());
            &self.text[..byte_index]
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.complete
    }
    
    pub fn skip(&mut self) {
        self.visible_chars = self.text.chars().count();
        self.complete = true;
        self.paused = false;
    }
    
    pub fn reset(&mut self) {
        self.visible_chars = 0;
        self.elapsed = 0.0;
        self.complete = false;
        self.paused = false;
    }
    
    pub fn pause(&mut self) {
        self.paused = true;
    }
    
    pub fn resume(&mut self) {
        self.paused = false;
    }
    
    pub fn is_paused(&self) -> bool {
        self.paused
    }
    
    pub fn set_speed(&mut self, speed: TextSpeed) {
        self.chars_per_second = speed.chars_per_second();
        if self.chars_per_second.is_infinite() {
            self.skip();
        }
    }
    
    pub fn full_text(&self) -> &str {
        &self.text
    }
    
    pub fn progress(&self) -> f32 {
        let total = self.text.chars().count();
        if total == 0 {
            1.0
        } else {
            (self.visible_chars as f32) / (total as f32)
        }
    }
}
pub struct TypewriterInstance {
    typewriter_effects: Vec<TypewriterEffect>,
    next_id: usize,
}

impl TypewriterInstance {
    pub fn new() -> Self {
        Self {
            typewriter_effects: Vec::new(),
            next_id: 0
        }
    }
    
    
    pub fn add_typewriter_effect(&mut self, text: impl Into<String>, speed: TextSpeed) -> usize {
        let effect = TypewriterEffect::new(text, speed, self.next_id);
        self.typewriter_effects.push(effect);
        self.next_id += 1;
        self.next_id - 1
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for effect in self.typewriter_effects.iter_mut() {
            effect.update(delta_time);
        }
    }
    
    pub fn remove_typewriter_effect(&mut self, id: usize) {
        self.typewriter_effects.remove(id);
    }
    
    pub fn is_empty(&self) -> bool {
        self.typewriter_effects.is_empty()
    }
    
    pub fn get_typewriter_effects(&self) -> Iter<TypewriterEffect> {
        self.typewriter_effects.iter().clone()
    }
}
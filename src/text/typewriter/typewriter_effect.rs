use crate::text::PunctuationConfig;
pub use crate::text::{TextSpeed, TextStyle};

/// A character-by-character text reveal with configurable speed and automatic
/// punctuation pauses.
#[derive(Debug)]
pub struct TypewriterEffect {
    pub(crate) id: usize,
    pub x: f32,
    pub y: f32,
    chars: Vec<char>,
    full_text: String,
    visible_chars: usize,
    chars_per_second: f32,
    elapsed: f32,
    paused: bool,
    complete: bool,
    style: TextStyle,
    pause_timer: f32,
    punctuation_config: PunctuationConfig,
}

impl TypewriterEffect {
    pub fn new(
        text: impl Into<String>,
        speed: TextSpeed,
        id: usize,
        x: f32,
        y: f32,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> Self {
        let full_text = text.into();
        let chars: Vec<char> = full_text.chars().collect();
        let chars_per_second = speed.chars_per_second();
        let complete = chars_per_second.is_infinite();
        let visible_chars = if complete {
            full_text.chars().count()
        } else {
            0
        };

        Self {
            chars,
            full_text,
            visible_chars,
            chars_per_second,
            elapsed: 0.0,
            paused: false,
            complete,
            id,
            x,
            y,
            style,
            pause_timer: 0.0,
            punctuation_config,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.complete {
            return;
        }

        if self.pause_timer > 0.0 {
            self.pause_timer -= delta_time;
            if self.pause_timer > 0.0 {
                return;
            }
        }

        if self.chars_per_second <= f32::EPSILON {
            return;
        }

        let seconds_per_char = 1.0 / self.chars_per_second;
        self.elapsed += delta_time;

        #[allow(clippy::while_float)]
        while self.elapsed >= seconds_per_char {
            if !self.while_need_to_update(seconds_per_char, self.chars.len()) {
                break;
            }
        }
    }

    #[must_use]
    pub fn visible_text(&self) -> &str {
        if self.complete {
            &self.full_text
        } else {
            let byte_index = self
                .full_text
                .char_indices()
                .nth(self.visible_chars)
                .map_or(self.full_text.len(), |(i, _)| i);
            &self.full_text[..byte_index]
        }
    }

    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn skip(&mut self) {
        self.visible_chars = self.chars.len();
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
        &self.full_text
    }

    #[must_use]
    pub fn progress(&self) -> f32 {
        let total = self.chars.len();
        if total == 0 {
            1.0
        } else {
            (self.visible_chars as f32) / (total as f32)
        }
    }

    pub fn set_text(
        &mut self,
        text: impl Into<String>,
        new_speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) {
        self.full_text = text.into();
        self.chars = self.full_text.chars().collect();
        self.punctuation_config = punctuation_config;

        self.visible_chars = 0;
        self.elapsed = 0.0;
        self.complete = false;
        self.paused = false;
        self.pause_timer = 0.0;

        self.set_speed(new_speed);
        self.style = style;
    }

    pub(crate) fn get_style(&self) -> TextStyle {
        self.style.clone()
    }

    pub fn set_progress(&mut self, progress: f32) -> bool {
        if self.chars_per_second <= f32::EPSILON {
            return false;
        }
        let progress = progress.clamp(0.0, 1.0);
        let total_chars = self.chars.len();

        if total_chars == 0 {
            self.complete = true;
            return false;
        }

        self.visible_chars = (total_chars as f32 * progress).round() as usize;

        if self.visible_chars >= total_chars {
            self.visible_chars = total_chars;
            self.complete = true;
        } else {
            self.complete = false;
        }

        self.elapsed = 0.0;
        self.pause_timer = 0.0;
        true
    }

    pub fn set_punctuation_config(&mut self, punctuation_config: PunctuationConfig) {
        self.punctuation_config = punctuation_config
    }

    fn while_need_to_update(&mut self, seconds_per_char: f32, text_len: usize) -> bool {
        self.elapsed -= seconds_per_char;

        if self.visible_chars >= text_len {
            self.complete = true;
            return false;
        }

        let c = self.chars[self.visible_chars];
        self.visible_chars += 1;

        let pause_duration = match c {
            '.' | '!' | '?' => Some(self.punctuation_config.sentence_end),
            ',' => Some(self.punctuation_config.comma),
            _ => Some(self.punctuation_config.other),
        };

        if let Some(duration) = pause_duration {
            self.pause_timer = duration;
            false
        } else {
            true
        }
    }
}

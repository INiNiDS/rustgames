use crate::text::PunctuationConfig;
pub use crate::text::{TextSpeed, TextStyle};

use super::TypewriterEffect;

impl TypewriterEffect {
    pub fn set_text(
        &mut self,
        text: impl Into<String>,
        new_speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) {
        self.text_id = 0;
        self.full_text = text.into();
        self.punctuation_config = punctuation_config;

        let (chars, visible_indices) = Self::parse_tags(&self.full_text);
        self.chars = chars;
        self.visible_indices = visible_indices;

        self.visible_chars = 0;
        self.elapsed = 0.0;
        self.complete = false;
        self.paused = false;
        self.pause_timer = 0.0;

        self.set_speed(new_speed);
        self.style = style;
    }

    pub fn set_text_with_id(
        &mut self,
        text: impl Into<String>,
        text_id: u32,
        new_speed: TextSpeed,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) {
        self.text_id = text_id;
        self.full_text = text.into();
        self.punctuation_config = punctuation_config;

        let (chars, visible_indices) = Self::parse_tags(&self.full_text);
        self.chars = chars;
        self.visible_indices = visible_indices;

        self.visible_chars = 0;
        self.elapsed = 0.0;
        self.complete = false;
        self.paused = false;
        self.pause_timer = 0.0;

        self.set_speed(new_speed);
        self.style = style;
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
        #[allow(clippy::cast_sign_loss)]
        {
            self.visible_chars = (total_chars as f32 * progress).round() as usize;
        }
        self.complete = self.visible_chars >= total_chars;
        if self.complete {
            self.visible_chars = total_chars;
        }
        self.elapsed = 0.0;
        self.pause_timer = 0.0;
        true
    }

    pub const fn set_punctuation_config(&mut self, punctuation_config: PunctuationConfig) {
        self.punctuation_config = punctuation_config;
    }

    pub(super) fn while_need_to_update(&mut self, seconds_per_char: f32, text_len: usize) -> bool {
        self.elapsed -= seconds_per_char;

        if self.visible_chars >= text_len {
            self.complete = true;
            return false;
        }

        let c = self.chars[self.visible_chars];
        self.visible_chars += 1;

        let pause = self.get_pause_for_char(c);
        self.pause_timer = pause;

        self.pause_timer <= 0.0
    }

    const fn get_pause_for_char(&self, c: char) -> f32 {
        match c {
            '.' | '!' | '?' => self.punctuation_config.sentence_end,
            ',' => self.punctuation_config.comma,
            _ => self.punctuation_config.other,
        }
    }
}

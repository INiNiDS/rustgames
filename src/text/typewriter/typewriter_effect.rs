use crate::text::PunctuationConfig;
pub use crate::text::{TextSpeed, TextStyle};

/// A character-by-character text reveal with configurable speed and automatic
/// punctuation pauses.
#[derive(Debug)]
pub struct TypewriterEffect {
    pub(crate) id: usize,
    pub x: f32,
    pub y: f32,
    pub(super) chars: Vec<char>,
    pub(super) full_text: String,
    pub(super) visible_chars: usize,
    pub(super) chars_per_second: f32,
    pub(super) elapsed: f32,
    pub(super) paused: bool,
    pub(super) complete: bool,
    pub(super) style: TextStyle,
    pub(super) pause_timer: f32,
    pub(super) punctuation_config: PunctuationConfig,
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
        if self.complete || self.paused {
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

        self.advance_chars(delta_time);
    }

    fn advance_chars(&mut self, delta_time: f32) {
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

    pub const fn skip(&mut self) {
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

    #[allow(clippy::missing_const_for_fn)]
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

    pub(crate) fn get_style(&self) -> TextStyle {
        self.style.clone()
    }
}

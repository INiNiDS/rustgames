use crate::text::PunctuationConfig;
pub use crate::text::{TextSpeed, TextStyle};
use crate::text::typewriter::TypewriterBuilder;

/// A character-by-character text reveal with configurable speed and automatic
/// punctuation pauses.
///
/// `text_id = 0` means the text is used as-is (no translation lookup).
/// Any non-zero `text_id` will be resolved through the translation system at render time.
#[derive(Debug)]
pub struct TypewriterEffect {
    pub(crate) id: usize,
    pub x: f32,
    pub y: f32,
    pub(super) chars: Vec<char>,
    pub(crate) text_id: u32,
    pub(super) full_text: String,
    pub(super) visible_indices: Vec<usize>,
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
    /// Create a new effect with a raw text string (`text_id = 0`, no translation).
    pub fn new(
        text: impl Into<String>,
        speed: TextSpeed,
        x: f32,
        y: f32,
        style: TextStyle,
        punctuation_config: PunctuationConfig,
    ) -> Self {
        let builder = TypewriterBuilder {
            text: text.into(),
            text_id: None,
            speed,
            x,
            y,
            style,
            punctuation_config,
        };
        Self::new_with_id(builder, 0)
    }

    /// Create a new effect with a translation key.
    /// `full_text` is used as fallback when no translation is found.
    ///
    /// # Panics
    /// Panics if `text_id` is `None` in the builder, since a translation key is required.
    #[must_use]
    pub fn new_with_id(
        tb: TypewriterBuilder,
        id: usize,
    ) -> Self {
        let (chars, visible_indices) = Self::parse_tags(&tb.text);
        let chars_per_second = tb.speed.chars_per_second();
        let complete = chars_per_second.is_infinite();
        let visible_chars = if complete { chars.len() } else { 0 };
        Self {
            id,
            chars,
            text_id: tb.text_id.unwrap_or(0),
            full_text: tb.text,
            visible_indices,
            visible_chars,
            chars_per_second,
            elapsed: 0.0,
            paused: false,
            complete,
            x: tb.x,
            y: tb.y,
            style: tb.style,
            pause_timer: 0.0,
            punctuation_config: tb.punctuation_config,
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

    #[must_use]
    pub fn visible_text(&self) -> &str {
        if self.complete {
            &self.full_text
        } else if self.visible_chars == 0 {
            ""
        } else {
            let end_byte_index = self.visible_indices[self.visible_chars - 1];
            &self.full_text[..end_byte_index]
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

    pub const fn set_speed(&mut self, speed: TextSpeed) {
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
    #[allow(clippy::cast_precision_loss)]
    pub const fn progress(&self) -> f32 {
        let total = self.chars.len();
        if total == 0 {
            1.0
        } else {
            (self.visible_chars as f32) / (total as f32)
        }
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

    pub(crate) fn get_style(&self) -> TextStyle {
        self.style.clone()
    }
    pub(crate) fn parse_tags(text: &str) -> (Vec<char>, Vec<usize>) {
        let mut chars = Vec::new();
        let mut indices = Vec::new();
        let mut iter = text.char_indices().peekable();

        while let Some((idx, c)) = iter.next() {
            if c == '[' {
                let tag_content = Self::collect_tag_content(&iter);
                if Self::is_valid_tag(&tag_content) {
                    Self::skip_tag_chars(&mut iter, tag_content.chars().count());
                    continue;
                }
            }
            chars.push(c);
            indices.push(idx + c.len_utf8());
        }
        (chars, indices)
    }

    /// Lookahead-collects the text inside `[...]` without advancing `iter`.
    fn collect_tag_content<I>(iter: &std::iter::Peekable<I>) -> String
    where
        I: Iterator<Item=(usize, char)> + Clone,
    {
        let mut tag_content = String::new();
        for (_, tc) in iter.clone() {
            if tc == ']' {
                break;
            }
            tag_content.push(tc);
        }
        tag_content
    }

    /// Advances `iter` past `tag_len` characters plus the closing `]`.
    fn skip_tag_chars<I>(iter: &mut std::iter::Peekable<I>, tag_len: usize)
    where
        I: Iterator<Item=(usize, char)>,
    {
        for _ in 0..=tag_len {
            iter.next();
        }
    }

    fn is_valid_tag(tag: &str) -> bool {
        match tag {
            "b" | "m" | "sb" | "i" | "color" => true,
            t if t.starts_with("color=") => true,
            t if t.starts_with('/') => true,
            _ => false,
        }
    }
}

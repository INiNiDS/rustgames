/// Configures extra pause durations (in seconds) inserted after certain
/// punctuation marks during typewriter playback.
#[derive(Debug, Clone, Copy)]
pub struct PunctuationConfig {
    /// Pause after sentence-ending punctuation (`.`, `!`, `?`).
    pub sentence_end: f32,
    /// Pause after a comma (`,`).
    pub comma: f32,
    /// Pause after any other recognized punctuation.
    pub other: f32,
}

impl PunctuationConfig {
    /// Standard pauses: 0.5 s sentence end, 0.2 s comma.
    pub const DEFAULT: Self = Self {
        sentence_end: 0.5,
        comma: 0.2,
        other: 0.0,
    };

    /// No pauses — text reveals at a constant rate.
    pub const INSTANT: Self = Self {
        sentence_end: 0.0,
        comma: 0.0,
        other: 0.0,
    };

    /// Shorter pauses suitable for fast-paced dialogue.
    pub const FAST: Self = Self {
        sentence_end: 0.2,
        comma: 0.1,
        other: 0.05,
    };

    /// Longer pauses for a deliberate, measured reading pace.
    pub const SLOW: Self = Self {
        sentence_end: 1.0,
        comma: 0.5,
        other: 0.2,
    };

    /// Exaggerated pauses for theatrical or suspenseful moments.
    pub const DRAMATIC: Self = Self {
        sentence_end: 1.5,
        comma: 0.4,
        other: 0.1,
    };

    /// Moderate pauses mimicking casual chat-window text.
    pub const CHAT: Self = Self {
        sentence_end: 0.3,
        comma: 0.15,
        other: 0.0,
    };

    /// Gentle pauses for a relaxed narrative pace.
    pub const RELAXED: Self = Self {
        sentence_end: 0.8,
        comma: 0.3,
        other: 0.1,
    };
}

impl Default for PunctuationConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

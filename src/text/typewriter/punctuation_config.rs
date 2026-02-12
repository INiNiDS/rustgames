#[derive(Debug, Clone, Copy)]
pub struct PunctuationConfig {
    pub sentence_end: f32,
    pub comma: f32,
    pub other: f32,
}

impl PunctuationConfig {
    pub const DEFAULT: Self = Self {
        sentence_end: 0.5,
        comma: 0.2,
        other: 0.0,
    };

    pub const INSTANT: Self = Self {
        sentence_end: 0.0,
        comma: 0.0,
        other: 0.0,
    };

    pub const FAST: Self = Self {
        sentence_end: 0.2,
        comma: 0.1,
        other: 0.05,
    };

    pub const SLOW: Self = Self {
        sentence_end: 1.0,
        comma: 0.5,
        other: 0.2,
    };

    pub const DRAMATIC: Self = Self {
        sentence_end: 1.5,
        comma: 0.4,
        other: 0.1,
    };

    pub const CHAT: Self = Self {
        sentence_end: 0.3,
        comma: 0.15,
        other: 0.0,
    };

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

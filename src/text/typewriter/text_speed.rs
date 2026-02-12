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

use crate::graphics::Direction;

/// A scene transition style.
#[derive(Debug, Clone)]
pub enum Transition {
    Instant,
    Fade(f32),
    FadeToBlack(f32),
    Dissolve(f32),
    Wipe { direction: Direction, duration: f32 },
}
use crate::graphics::Direction;

/// A scene transition style with configurable duration.
///
/// # Examples
/// ```
/// use rustgames::prelude::*;
///
/// let fade = Transition::Fade(1.0);
/// assert!((fade.duration() - 1.0).abs() < f32::EPSILON);
///
/// let instant = Transition::Instant;
/// assert!((instant.duration()).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone)]
pub enum Transition {
    /// No transition — scene changes immediately.
    Instant,
    /// Cross-fade between old and new scenes over `duration` seconds.
    Fade(f32),
    /// Fade to black, then reveal the new scene.
    FadeToBlack(f32),
    /// Pixel-dissolve effect over `duration` seconds.
    Dissolve(f32),
    /// Directional wipe from one edge.
    Wipe { direction: Direction, duration: f32 },
}

impl Transition {
    /// Returns the total duration of this transition in seconds.
    #[must_use]
    pub const fn duration(&self) -> f32 {
        match self {
            Self::Instant => 0.0,
            Self::Fade(d) | Self::FadeToBlack(d) | Self::Dissolve(d) => *d,
            Self::Wipe { duration, .. } => *duration,
        }
    }

    /// Returns `true` when the transition completes in zero time.
    #[must_use]
    pub const fn is_instant(&self) -> bool {
        matches!(self, Self::Instant)
    }
}

/// Tracks the progress of an active [`Transition`].
pub struct TransitionState {
    pub transition: Transition,
    pub elapsed: f32,
    pub finished: bool,
}

impl TransitionState {
    /// Creates a new state for the given transition.
    #[must_use]
    pub const fn new(transition: Transition) -> Self {
        Self {
            transition,
            elapsed: 0.0,
            finished: false,
        }
    }

    /// Advances the transition by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        let d = self.transition.duration();
        if self.elapsed >= d {
            self.finished = true;
            self.elapsed = d;
        }
    }

    /// Returns normalised progress in `0.0..=1.0`.
    #[must_use]
    pub fn progress(&self) -> f32 {
        let d = self.transition.duration();
        if d == 0.0 {
            1.0
        } else {
            (self.elapsed / d).clamp(0.0, 1.0)
        }
    }

    /// Resets the transition to the beginning.
    pub const fn reset(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
    }

    /// Returns `true` when the transition has finished.
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.finished
    }
}

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

impl Transition {
    pub fn duration(&self) -> f32 {
        match self {
            Transition::Instant => 0.0,
            Transition::Fade(d) | Transition::FadeToBlack(d) | Transition::Dissolve(d) => *d,
            Transition::Wipe { duration, .. } => *duration,
        }
    }
}

pub struct TransitionState {
    pub transition: Transition,
    pub elapsed: f32,
    pub finished: bool,
}

impl TransitionState {
    pub fn new(transition: Transition) -> Self {
        Self {
            transition,
            elapsed: 0.0,
            finished: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        if self.elapsed >= self.transition.duration() {
            self.finished = true;
            self.elapsed = self.transition.duration();
        }
    }

    pub fn get_progress(&self) -> f32 {
        let d = self.transition.duration();
        if d == 0.0 {
            1.0
        } else {
            (self.elapsed / d).clamp(0.0, 1.0)
        }
    }
}

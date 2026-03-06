use crate::graphics::effects::animation::{AnimationGroupID, AnimationSystem};
use crate::graphics::effects::{Animation, Easing};

/// Fluent API for constructing a sequence of `TimelineStep` values.
#[derive(Debug, Default, Clone)]
pub struct TimelineBuilder {
    steps: Vec<TimelineStep>,
}

impl TimelineBuilder {
    /// Creates an empty [`TimelineBuilder`].
    #[must_use]
    pub const fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Appends a single animation step.
    #[must_use]
    pub fn single(mut self, animation: Animation, easing: Easing) -> Self {
        self.steps.push(TimelineStep::Single(animation, easing));
        self
    }

    /// Appends a group of animations that all play simultaneously.
    #[must_use]
    pub fn parallel(mut self, step: Vec<(Animation, Easing)>) -> Self {
        self.steps.push(TimelineStep::Parallel(step));
        self
    }

    /// Appends an idle gap of `seconds` before the next step.
    #[must_use]
    pub fn gap(mut self, seconds: f32) -> Self {
        self.steps.push(TimelineStep::Gap(seconds.max(0.0)));
        self
    }

    /// Consumes the builder and returns the collected steps.
    #[must_use]
    pub fn build(self) -> Vec<TimelineStep> {
        self.steps
    }

    /// Starts the timeline on `controller` immediately and returns the
    /// resulting [`AnimationGroupID`].
    pub fn start(self, controller: &mut AnimationSystem) -> AnimationGroupID {
        controller.start_timeline(self.steps)
    }
}

/// A single step in a timeline: a single animation, a parallel group, or a
/// time gap.
#[derive(Debug, Clone)]
pub enum TimelineStep {
    /// A single animation with its easing function.
    Single(Animation, Easing),
    /// Several animations played simultaneously, each with its own easing.
    Parallel(Vec<(Animation, Easing)>),
    /// A pause of the given number of seconds before the next step.
    Gap(f32),
}

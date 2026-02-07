use crate::graphics::effects::{Animation, Easing};
use crate::graphics::effects::animation::{AnimationGroupID, AnimationSystem};

/// Fluent API for constructing a sequence of `TimelineStep` values.
#[derive(Debug, Default, Clone)]
pub struct TimelineBuilder {
    steps: Vec<TimelineStep>,
}

impl TimelineBuilder {
    #[must_use] 
    pub const fn new() -> Self {
        Self { steps: Vec::new() }
    }

    #[must_use] 
    pub fn single(mut self, animation: Animation, easing: Easing) -> Self {
        self.steps.push(TimelineStep::Single(animation, easing));
        self
    }

    #[must_use] 
    pub fn parallel(mut self, step: Vec<(Animation, Easing)>) -> Self {
        self.steps.push(TimelineStep::Parallel(step));
        self
    }

    #[must_use] 
    pub fn gap(mut self, seconds: f32) -> Self {
        self.steps.push(TimelineStep::Gap(seconds.max(0.0)));
        self
    }

    #[must_use] 
    pub fn build(self) -> Vec<TimelineStep> {
        self.steps
    }

    pub fn start(self, controller: &mut AnimationSystem) -> AnimationGroupID {
        controller.start_timeline(self.steps)
    }
}

/// A single step in a timeline: a single animation, a parallel group, or a
/// time gap.
#[derive(Debug, Clone)]
pub enum TimelineStep {
    Single(Animation, Easing),
    Parallel(Vec<(Animation, Easing)>),
    Gap(f32)
}

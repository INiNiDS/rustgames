use crate::graphics::{Animation, AnimationController, Easing};
use crate::graphics::animation::AnimationGroupID;

#[derive(Debug, Default, Clone)]
pub struct TimelineBuilder {
    steps: Vec<TimelineStep>,
}

impl TimelineBuilder {
    pub fn new() -> TimelineBuilder {
        Self { steps: Vec::new() }
    }

    pub fn single(mut self, animation: Animation, easing: Easing) -> TimelineBuilder {
        self.steps.push(TimelineStep::Single(animation, easing));
        self
    }

    pub fn parallel(mut self, step: Vec<(Animation, Easing)>) -> TimelineBuilder {
        self.steps.push(TimelineStep::Parallel(step));
        self
    }

    pub fn gap(mut self, seconds: f32) -> Self {
        self.steps.push(TimelineStep::Gap(seconds.max(0.0)));
        self
    }

    pub fn build(self) -> Vec<TimelineStep> {
        self.steps
    }

    pub fn start(self, controller: &mut AnimationController) -> AnimationGroupID {
        controller.start_timeline(self.steps)
    }
}

#[derive(Debug, Clone)]
pub enum TimelineStep {
    Single(Animation, Easing),
    Parallel(Vec<(Animation, Easing)>),
    Gap(f32)
}

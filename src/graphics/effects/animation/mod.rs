pub mod animation_instance;
pub mod direction;
pub mod visual;
pub mod easing;
pub mod timeline;
pub mod transition;
pub(crate) mod animation_system;
pub mod sprite_animation;

pub(crate) use animation_system::AnimationSystem;
pub use animation_instance::ActiveAnimation;
pub use direction::Direction;
pub use easing::Easing;
pub use visual::{AnimEffect, CustomCombinedMode, VisualState};
pub use timeline::{TimelineBuilder, TimelineStep};
pub use transition::Transition;
pub use sprite_animation::*;

/// A named animation that can be played through `AnimationController`.
#[derive(Debug, Clone)]
pub enum Animation {
    FadeIn { duration: f32 },
    FadeOut { duration: f32 },
    SlideIn { from: Direction, distance: f32, duration: f32 },
    SlideOut { to: Direction, distance: f32, duration: f32 },
    Scale { from: f32, to: f32, duration: f32 },
    Rotate { from: f32, to: f32, duration: f32 },
    Shake { intensity: f32, duration: f32 },
}

/// A group of animation IDs returned from sequence/parallel/timeline starts.
#[derive(Debug, Clone)]
pub struct AnimationGroupID {
    ids: Vec<usize>,
}

impl AnimationGroupID {
    #[must_use] 
    pub const fn new(ids: Vec<usize>) -> Self {
        Self { ids }
    }

    #[must_use] 
    pub const fn empty() -> Self {
        Self { ids: vec![] }
    }

    #[must_use] 
    pub const fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    #[must_use] 
    pub const fn len(&self) -> usize {
        self.ids.len()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &usize> {
        self.ids.iter()
    }

    pub(crate) fn get_id(&self, index: usize) -> Option<&usize> {
        self.ids.get(index)
    }

    pub(crate) fn remove(&mut self, index: usize) {
        self.ids.remove(index);
    }
}


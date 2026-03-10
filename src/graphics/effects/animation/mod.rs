mod anim_effect_ops;
pub(crate) mod animation_group_ops;
pub mod animation_instance;
pub mod animation_mode;
pub mod animation_system;
pub(crate) mod animation_system_ext;
pub mod direction;
pub mod easing;
pub mod sprite_animation;
pub mod timeline;
pub mod transition;
pub mod visual;

pub use animation_instance::ActiveAnimation;
pub use animation_mode::AnimationMode;
pub(crate) use animation_system::AnimationSystem;
pub use direction::Direction;
pub use easing::Easing;
pub use sprite_animation::SpriteAnimation;
pub use timeline::{TimelineBuilder, TimelineStep};
pub use transition::Transition;
pub use visual::{AnimEffect, CustomCombinedMode, VisualState};

/// A named animation that can be played through `AnimationController`.
#[derive(Debug, Clone)]
pub enum Animation {
    /// Fade opacity from 0 to 1 over `duration` seconds.
    FadeIn { duration: f32 },
    /// Fade opacity from 1 to 0 over `duration` seconds.
    FadeOut { duration: f32 },
    /// Slide the element into view from `from` by `distance` pixels over
    /// `duration` seconds.
    SlideIn {
        from: Direction,
        distance: f32,
        duration: f32,
    },
    /// Slide the element out of view toward `to` by `distance` pixels over
    /// `duration` seconds.
    SlideOut {
        to: Direction,
        distance: f32,
        duration: f32,
    },
    /// Animate scale from `from` to `to` over `duration` seconds.
    Scale { from: f32, to: f32, duration: f32 },
    /// Animate rotation from `from` to `to` radians over `duration` seconds.
    Rotate { from: f32, to: f32, duration: f32 },
    /// Apply a random positional shake of `intensity` pixels for `duration`
    /// seconds.
    Shake { intensity: f32, duration: f32 },
}

/// A group of animation IDs returned from sequence/parallel/timeline starts.
#[derive(Debug, Clone)]
pub struct AnimationGroupID {
    ids: Vec<usize>,
}

impl AnimationGroupID {
    /// Creates a new group wrapping `ids`.
    #[must_use]
    pub const fn new(ids: Vec<usize>) -> Self {
        Self { ids }
    }

    /// Creates an empty group (no animations).
    #[must_use]
    pub const fn empty() -> Self {
        Self { ids: vec![] }
    }

    /// Returns `true` when the group contains no animation IDs.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Returns the number of animation IDs in the group.
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

use crate::graphics::effects::animation::direction::Direction;

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
    pub fn new(ids: Vec<usize>) -> AnimationGroupID {
        AnimationGroupID { ids }
    }

    pub fn empty() -> AnimationGroupID {
        AnimationGroupID { ids: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn len(&self) -> usize {
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

/// A scene transition style.
#[derive(Debug, Clone)]
pub enum Transition {
    Instant,
    Fade(f32),
    FadeToBlack(f32),
    Dissolve(f32),
    Wipe { direction: Direction, duration: f32 },
}
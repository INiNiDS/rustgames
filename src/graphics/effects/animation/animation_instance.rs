use crate::prelude::{AnimEffect, Animation, Easing};
use glam::Vec2;

const SHAKE_FREQUENCY_X: f32 = 40.0;
const SHAKE_FREQUENCY_Y: f32 = 50.0;

/// A running animation with elapsed time, easing, delay, and playback speed.
#[derive(Debug)]
pub struct ActiveAnimation {
    pub id: usize,
    pub animation: Animation,
    pub elapsed: f32,
    pub easing: Easing,
    pub paused: bool,
    pub playback: f32,
    delay: f32,
}

impl ActiveAnimation {
    /// Creates a new [`ActiveAnimation`] with the given id, animation,
    /// easing, and initial `delay` before playback starts.
    #[must_use]
    pub const fn new(id: usize, animation: Animation, easing: Easing, delay: f32) -> Self {
        Self {
            id,
            animation,
            elapsed: 0.0,
            easing,
            paused: false,
            playback: 1.0,
            delay,
        }
    }

    /// Returns the total duration of the wrapped animation in seconds.
    #[must_use]
    pub const fn duration(&self) -> f32 {
        match &self.animation {
            Animation::FadeIn { duration }
            | Animation::FadeOut { duration }
            | Animation::SlideIn { duration, .. }
            | Animation::SlideOut { duration, .. }
            | Animation::Scale { duration, .. }
            | Animation::Rotate { duration, .. }
            | Animation::Shake { duration, .. } => *duration,
        }
    }

    /// Returns linear progress in `0.0..=1.0`, ignoring easing.
    #[must_use]
    pub fn progress(&self) -> f32 {
        let duration = self.duration();

        if duration <= 0.0 {
            1.0
        } else {
            (self.elapsed / duration).clamp(0.0, 1.0)
        }
    }

    /// Advances the animation by `delta_time` seconds. Respects `paused` flag,
    /// initial delay, and `playback` speed/direction.
    pub fn update(&mut self, delta_time: f32) {
        if self.paused {
            return;
        }

        let mut dt = delta_time;
        if self.playback > 0.0 && self.delay > 0.0 {
            let eat = dt.min(self.delay);

            self.delay -= eat;
            dt -= eat;

            if dt <= 0.0 {
                return;
            }
        }

        self.elapsed += dt * self.playback;
        self.elapsed = self.elapsed.clamp(0.0, self.duration());
    }

    /// Returns `true` when the animation has reached its natural end point
    /// (or start point if playing in reverse).
    #[must_use]
    pub fn is_finished(&self) -> bool {
        if self.playback >= 0.0 {
            self.elapsed >= self.duration()
        } else {
            self.elapsed <= 0.0
        }
    }

    /// Returns progress with the configured easing function applied.
    #[must_use]
    pub fn eased_progress(&self) -> f32 {
        self.easing.apply(self.progress())
    }

    /// Computes the [`AnimEffect`] delta for `size`-scaled rendering at the
    /// current playback position.
    #[must_use]
    pub fn effect(&self, size: Vec2) -> AnimEffect {
        let t = self.eased_progress();

        match &self.animation {
            Animation::FadeIn { .. } => AnimEffect::with_opacity(t),
            Animation::FadeOut { .. } => AnimEffect::with_opacity(1.0 - t),
            Animation::SlideIn { .. }
            | Animation::SlideOut { .. }
            | Animation::Scale { .. }
            | Animation::Rotate { .. } => self.transform_effect(t, size),
            Animation::Shake { intensity, .. } => self.calculate_shake(*intensity, t),
        }
    }

    fn transform_effect(&self, t: f32, size: Vec2) -> AnimEffect {
        match &self.animation {
            Animation::SlideIn { from, distance, .. } => {
                let max_offset = from.to_vector() * *distance * size;
                AnimEffect::with_offset(max_offset * (1.0 - t))
            }
            Animation::SlideOut { to, distance, .. } => {
                let max_offset = to.to_vector() * *distance * size;
                AnimEffect::with_offset(max_offset * t)
            }
            Animation::Scale { from, to, .. } => {
                let scale = Self::lerp(*from, *to, t);
                AnimEffect::with_scale(Vec2::splat(scale))
            }
            Animation::Rotate { from, to, .. } => {
                AnimEffect::with_rotation(Self::lerp(*from, *to, t))
            }
            _ => AnimEffect::default(),
        }
    }

    /// Pauses the animation at its current position.
    pub const fn stop(&mut self) {
        self.paused = true;
    }

    /// Replaces the initial delay with `delay` seconds and resets elapsed to 0.
    pub const fn set_delay(&mut self, delay: f32) {
        self.delay = delay.max(0.0);
        self.elapsed = 0.0;
    }

    /// Adds `delay` seconds to the remaining delay.
    pub fn add_delay(&mut self, delay: f32) {
        self.delay += delay;
    }

    fn lerp(start: f32, end: f32, t: f32) -> f32 {
        (end - start).mul_add(t, start)
    }

    fn calculate_shake(&self, intensity: f32, t: f32) -> AnimEffect {
        let decay = 1.0 - t;

        // Casting is allow to create a consistent seed for the shake based on the animation ID, ensuring different animations shake differently.
        #[allow(clippy::cast_precision_loss)]
        let seed = (self.id & 0x7F_FFFF) as f32;

        let shake_x = seed
            .mul_add(SHAKE_FREQUENCY_X, self.elapsed)
            .sin();
        let shake_y = seed
            .mul_add(SHAKE_FREQUENCY_Y, self.elapsed)
            .cos();

        let offset = Vec2::new(shake_x, shake_y) * intensity * decay;
        AnimEffect::with_offset(offset)
    }
}

use glam::Vec2;
use crate::prelude::{AnimEffect, Animation, Easing};

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
    delay: f32
}


impl ActiveAnimation {
    #[must_use] 
    pub fn new(id: usize, animation: Animation, easing: Easing, delay: f32) -> Self {
        Self {
            id,
            animation,
            elapsed: 0.0,
            easing,
            paused: false,
            playback: 1.0,
            delay
        }
    }


    #[must_use] 
    pub fn duration(&self) -> f32 {
        match &self.animation {
            Animation::FadeIn { duration } => *duration,
            Animation::FadeOut { duration } => *duration,
            Animation::SlideIn { duration, .. } => *duration,
            Animation::SlideOut { duration, .. } => *duration,
            Animation::Scale { duration, .. } => *duration,
            Animation::Rotate { duration, .. } => *duration,
            Animation::Shake { duration, .. } => *duration,
        }
    }


    #[must_use] 
    pub fn progress(&self) -> f32 {
        let duration = self.duration();

        if duration <= 0.0 {
            1.0
        } else {
            (self.elapsed / duration).clamp(0.0, 1.0)
        }
    }


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


    #[must_use] 
    pub fn is_finished(&self) -> bool {
        if self.playback >= 0.0 {
            self.elapsed >= self.duration()
        } else {
            self.elapsed <= 0.0
        }
    }

    

    #[must_use] 
    pub fn eased_progress(&self) -> f32 {
        self.easing.apply(self.progress())
    }

    #[must_use]
    pub fn effect(&self, size: Vec2) -> AnimEffect {
        let t = self.eased_progress();

        match &self.animation {
            Animation::FadeIn { .. } => AnimEffect::with_opacity(t),

            Animation::FadeOut { .. } => AnimEffect::with_opacity(1.0 - t),

            Animation::SlideIn { from, distance, .. } => {
                let max_offset = from.to_vector() * *distance * size;
                AnimEffect::with_offset(max_offset * (1.0 - t))
            },

            Animation::SlideOut { to, distance, .. } => {
                let max_offset = to.to_vector() * *distance * size;
                AnimEffect::with_offset(max_offset * t)
            },

            Animation::Scale { from, to, .. } => {
                let scale = Self::lerp(*from, *to, t);
                AnimEffect::with_scale(Vec2::splat(scale))
            },

            Animation::Rotate { from, to, .. } => {
                AnimEffect::with_rotation(Self::lerp(*from, *to, t))
            },

            Animation::Shake { intensity, .. } => {
                self.calculate_shake(*intensity, t)
            },
        }
    }

    pub fn stop(&mut self) {
        self.paused = true;
    }

    pub fn set_delay(&mut self, delay: f32) {
        self.delay = delay.max(0.0);
        self.elapsed = 0.0;
    }

    pub fn add_delay(&mut self, delay: f32) {
        self.delay += delay;
    }

    fn lerp(start: f32, end: f32, t: f32) -> f32 {
        start + (end - start) * t
    }

    fn calculate_shake(&self, intensity: f32, t: f32) -> AnimEffect {
        let decay = 1.0 - t;

        let shake_x = (self.elapsed + self.id as f32 * SHAKE_FREQUENCY_X).sin();
        let shake_y = (self.elapsed + self.id as f32 * SHAKE_FREQUENCY_Y).cos();

        let offset = Vec2::new(shake_x, shake_y) * intensity * decay;
        AnimEffect::with_offset(offset)
    }
}


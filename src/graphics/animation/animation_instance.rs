use glam::Vec2;
use crate::prelude::{AnimEffect, Animation, Easing};

#[derive(Debug)]
pub struct AnimationInstance {
    pub id: usize,
    pub animation: Animation,
    pub elapsed: f32,
    pub easing: Easing,
    pub paused: bool,
    pub playback: f32,
    delay: f32
}


impl AnimationInstance {
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


    pub fn is_finished(&self) -> bool {
        if self.playback >= 0.0 {
            self.elapsed >= self.duration()
        } else {
            self.elapsed <= 0.0
        }
    }

    

    pub fn eased_progress(&self) -> f32 {
        self.easing.apply(self.progress())
    }
    
    pub fn effect(&self, size: Vec2) -> AnimEffect {
        let t = self.eased_progress();
        match &self.animation {
            Animation::FadeIn { .. } => {
                AnimEffect::with_opacity(t)

            },
            Animation::FadeOut { .. } => AnimEffect::with_opacity(1.0 - t),
            Animation::SlideIn { from, distance, .. } => {
                let dir_vec = from.to_vector();
                let x = dir_vec.x * distance * size.x;
                let y = dir_vec.y * distance * size.y;
                let start_offset = Vec2::new(x, y);
                let offset = start_offset * (1.0 - t);
                AnimEffect::with_offset(offset)
            },

            Animation::SlideOut { to, distance, .. } => {
                let dir_vec = to.to_vector();
                let x = dir_vec.x * distance * size.x;
                let y = dir_vec.y * distance * size.y;
                let end_offset = Vec2::new(x, y);
                let offset = end_offset * t;
                AnimEffect::with_offset(offset)
            }
            Animation::Scale { from, to, .. } => {
                let scale = from + (to - from) * t;
                AnimEffect::with_scale(Vec2::splat(scale))
            },
            Animation::Rotate { from, to, .. } => {
                let rotation = from + (to - from) * t;
                AnimEffect::with_rotation(rotation)
            },
            Animation::Shake { intensity, .. } => {
                let shake_x = (self.elapsed + self.id as f32 * 40.0).sin() * intensity * (1.0 - t);
                let shake_y = (self.elapsed + self.id as f32 * 50.0).cos() * intensity * (1.0 - t);
                AnimEffect::with_offset(Vec2::new(shake_x, shake_y))
            },

        }
    }

}


use crate::graphics::effects::animation::animation_mode::AnimationMode;
use glam::Vec4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PingPongDirection {
    Forward,
    Backward,
}

/// Plays through a sequence of UV frames at a given FPS.
///
/// Supports `PlayOnce`, `Loop`, and `PingPong` playback modes.
#[derive(Debug, Clone)]
pub struct SpriteAnimation {
    frames: Vec<Vec4>,
    current_frame: usize,
    elapsed: f32,
    frame_duration: f32,
    mode: AnimationMode,
    ping_pong_dir: PingPongDirection,
    paused: bool,
    finished: bool,
}

impl SpriteAnimation {
    #[must_use]
    pub fn new(frames: Vec<Vec4>, fps: f32, mode: AnimationMode) -> Self {
        assert!(!frames.is_empty(), "Animation must have at least one frame");
        assert!(fps > 0.0, "FPS must be positive");

        Self {
            frames,
            current_frame: 0,
            elapsed: 0.0,
            frame_duration: 1.0 / fps,
            mode,
            ping_pong_dir: PingPongDirection::Forward,
            paused: false,
            finished: false,
        }
    }

    #[must_use]
    pub fn from_grid(
        columns: usize,
        rows: usize,
        frame_count: usize,
        fps: f32,
        mode: AnimationMode,
    ) -> Self {
        assert!(columns > 0 && rows > 0, "Grid dimensions must be > 0");
        assert!(frame_count > 0, "Frame count must be > 0");
        assert!(
            frame_count <= columns * rows,
            "Frame count exceeds grid size"
        );

        let fw = 1.0 / columns as f32;
        let fh = 1.0 / rows as f32;

        let frames: Vec<Vec4> = (0..frame_count)
            .map(|i| {
                let col = (i % columns) as f32;
                let row = (i / columns) as f32;
                Vec4::new(col * fw, row * fh, fw, fh)
            })
            .collect();

        Self::new(frames, fps, mode)
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.paused || self.finished || self.frames.is_empty() {
            return;
        }
        self.elapsed += delta_time;
        if self.frame_duration > f32::EPSILON {
            #[allow(clippy::while_float)]
            while self.elapsed >= self.frame_duration {
                self.elapsed -= self.frame_duration;
                if self.step() {
                    self.finished = true;
                    break;
                }
            }
        } else {
            self.elapsed = 0.0;
        }
    }

    #[must_use]
    pub fn current_uv(&self) -> Vec4 {
        self.frames[self.current_frame]
    }

    #[must_use]
    pub const fn current_frame_index(&self) -> usize {
        self.current_frame
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.finished
    }

    pub const fn pause(&mut self) {
        self.paused = true;
    }

    pub const fn resume(&mut self) {
        self.paused = false;
    }

    pub const fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed = 0.0;
        self.finished = false;
        self.ping_pong_dir = PingPongDirection::Forward;
    }

    pub const fn set_frame(&mut self, frame: usize) {
        if frame < self.frames.len() {
            self.current_frame = frame;
            self.elapsed = 0.0;
        }
    }

    pub const fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    #[must_use]
    pub const fn frame_count(&self) -> usize {
        self.frames.len()
    }

    const fn step(&mut self) -> bool {
        let last_idx = self.frames.len() - 1;
        match self.mode {
            AnimationMode::PlayOnce => {
                if self.current_frame < last_idx {
                    self.current_frame += 1;
                    false
                } else {
                    true
                }
            }
            AnimationMode::Loop => {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
                false
            }
            AnimationMode::PingPong => {
                self.step_ping_pong(last_idx);
                false
            }
        }
    }

    const fn step_ping_pong(&mut self, last_idx: usize) {
        match self.ping_pong_dir {
            PingPongDirection::Forward => {
                if self.current_frame < last_idx {
                    self.current_frame += 1;
                } else {
                    self.ping_pong_dir = PingPongDirection::Backward;
                    if last_idx > 0 {
                        self.current_frame -= 1;
                    }
                }
            }
            PingPongDirection::Backward => {
                if self.current_frame > 0 {
                    self.current_frame -= 1;
                } else {
                    self.ping_pong_dir = PingPongDirection::Forward;
                    if last_idx > 0 {
                        self.current_frame += 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_loop() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 0.5, 0.5),
            Vec4::new(0.5, 0.0, 0.5, 0.5),
            Vec4::new(0.0, 0.5, 0.5, 0.5),
        ];
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
        assert_eq!(anim.current_frame_index(), 0);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        anim.update(0.2);
        assert_eq!(anim.current_frame_index(), 0);
        assert!(!anim.is_finished());
    }

    #[test]
    fn test_animation_play_once() {
        let frames = vec![Vec4::new(0.0, 0.0, 0.5, 0.5), Vec4::new(0.5, 0.0, 0.5, 0.5)];
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PlayOnce);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        assert!(!anim.is_finished());
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        assert!(anim.is_finished());
    }

    #[test]
    fn test_animation_ping_pong() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
        ];
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PingPong);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 2);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 0);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
    }

    #[test]
    fn test_animation_from_grid() {
        let anim = SpriteAnimation::from_grid(4, 4, 8, 10.0, AnimationMode::Loop);
        assert_eq!(anim.frame_count(), 8);
        let first_uv = anim.current_uv();
        assert_eq!(first_uv.x, 0.0);
        assert_eq!(first_uv.y, 0.0);
        assert_eq!(first_uv.z, 0.25);
        assert_eq!(first_uv.w, 0.25);
    }

    #[test]
    fn test_animation_pause_resume() {
        let frames = vec![Vec4::new(0.0, 0.0, 1.0, 1.0), Vec4::new(0.0, 0.0, 1.0, 1.0)];
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
        anim.pause();
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 0);
        anim.resume();
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
    }

    #[test]
    fn test_animation_reset() {
        let frames = vec![Vec4::new(0.0, 0.0, 1.0, 1.0), Vec4::new(0.0, 0.0, 1.0, 1.0)];
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PlayOnce);
        anim.update(0.2);
        assert!(anim.is_finished());
        anim.reset();
        assert_eq!(anim.current_frame_index(), 0);
        assert!(!anim.is_finished());
    }
}

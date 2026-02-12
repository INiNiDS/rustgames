use std::time::{Duration, Instant};

const TARGET_FPS: f32 = 60.0;
const MAX_DELTA_TIME: f32 = 0.1;

/// Frame-level timing: delta time, total elapsed time, FPS limiting, and lag
/// spike detection.
pub struct Time {
    start_time: Instant,
    last_update: Instant,
    delta: Duration,
    frame_count: u64,
    target_frame_duration: Duration,
    accumulated_duration: Duration,
    frame_start: Instant,
}

impl Time {
    #[must_use]
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            delta: Duration::from_secs_f32(1.0 / 60.0),
            frame_count: 0,
            target_frame_duration: Duration::from_secs_f32(1.0 / TARGET_FPS),
            accumulated_duration: Duration::ZERO,
            frame_start: now,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_update;

        let delta_secs = self.delta.as_secs_f32();
        if delta_secs > MAX_DELTA_TIME {
            self.delta = Duration::from_secs_f32(MAX_DELTA_TIME);
        }

        self.last_update = now;
        self.accumulated_duration += self.delta;
        self.frame_count += 1;
    }

    pub fn begin_frame(&mut self) {
        self.frame_start = Instant::now();
    }

    #[must_use]
    pub const fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    #[must_use]
    pub fn total_seconds(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    #[must_use]
    pub const fn frame_count(&self) -> u64 {
        self.frame_count
    }

    #[must_use]
    pub fn fps(&self) -> f32 {
        if self.delta.as_secs_f32() > 0.0 {
            1.0 / self.delta.as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_update = now;
        self.delta = Duration::from_secs_f32(1.0 / 60.0);
        self.frame_count = 0;
        self.accumulated_duration = Duration::ZERO;
    }

    pub fn limit_fps(&self) {
        let frame_time = self.frame_start.elapsed();
        if frame_time < self.target_frame_duration {
            std::thread::sleep(self.target_frame_duration.checked_sub(frame_time).unwrap());
        }
    }

    pub fn set_target_fps(&mut self, fps: f32) {
        self.target_frame_duration = Duration::from_secs_f32(1.0 / fps);
    }

    #[must_use]
    pub fn is_lag_spike(&self) -> bool {
        self.delta.as_secs_f32() > (1.0 / TARGET_FPS) * 2.0
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::VecDeque;

/// Rolling-window FPS counter. Maintains the last 60 frame times and computes
/// average, minimum, and maximum FPS.
pub struct FpsCounter {
    frame_times: VecDeque<f32>,
    total_time: f32,
    max_samples: usize,
}

impl FpsCounter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(60),
            total_time: 0.0,
            max_samples: 60,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.frame_times.push_back(delta_time);
        self.total_time += delta_time;

        while self.frame_times.len() > self.max_samples {
            if let Some(old_time) = self.frame_times.pop_front() {
                self.total_time -= old_time;
            }
        }
    }

    #[must_use]
    pub fn fps(&self) -> f32 {
        if self.frame_times.is_empty() || self.total_time == 0.0 {
            return 0.0;
        }

        self.frame_times.len() as f32 / self.total_time
    }

    #[must_use]
    pub fn frame_time_ms(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        (self.total_time / self.frame_times.len() as f32) * 1000.0
    }

    #[must_use]
    pub fn min_fps(&self) -> f32 {
        self.frame_times
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .map_or(
                0.0,
                |&max_time| if max_time > 0.0 { 1.0 / max_time } else { 0.0 },
            )
    }

    #[must_use]
    pub fn max_fps(&self) -> f32 {
        self.frame_times
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .map_or(
                0.0,
                |&min_time| if min_time > 0.0 { 1.0 / min_time } else { 0.0 },
            )
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_counter() {
        let mut counter = FpsCounter::new();

        for _ in 0..60 {
            counter.update(1.0 / 60.0);
        }

        let fps = counter.fps();
        assert!(
            (fps - 60.0).abs() < 1.0,
            "FPS should be approximately 60, got {fps}"
        );
    }

    #[test]
    fn test_frame_time() {
        let mut counter = FpsCounter::new();

        for _ in 0..60 {
            counter.update(1.0 / 60.0);
        }

        let frame_time = counter.frame_time_ms();
        assert!(
            (frame_time - 16.67).abs() < 1.0,
            "Frame time should be approximately 16.67ms, got {frame_time}"
        );
    }
}

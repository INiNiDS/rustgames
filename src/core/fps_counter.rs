//! Rolling-window FPS counter. Maintains the last 60 frame times and computes
//! average, minimum, and maximum FPS.

/// Fixed-size array for cache-friendly O(1) rolling window.
pub struct FpsCounter {
    /// Pre-allocated ring buffer for exactly 60 samples
    samples: [f32; 60],
    /// Current insertion index (wraps at 60)
    cursor: usize,
    /// Number of valid samples (grows to 60, then stays)
    len: usize,
    /// Running sum of all valid samples for O(1) average
    total_time: f32,
    /// Running maximum frame-time (= worst / lowest FPS). `f32::NAN` = stale.
    running_max_dt: f32,
    /// Running minimum frame-time (= best / highest FPS). `f32::NAN` = stale.
    running_min_dt: f32,
}

impl FpsCounter {
    /// Creates a new [`FpsCounter`] with a 60-sample rolling window.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            samples: [0.0; 60],
            cursor: 0,
            len: 0,
            total_time: 0.0,
            running_max_dt: f32::NAN,
            running_min_dt: f32::NAN,
        }
    }

    /// Records `delta_time` (in seconds) for the current frame.
    ///
    /// When the window is full the oldest sample is evicted.  Min/max are
    /// updated incrementally: a full O(60) rescan only happens when the
    /// evicted sample was the current min or max.
    pub fn update(&mut self, delta_time: f32) {
        let evicted = if self.len == 60 {
            let old = self.samples[self.cursor];
            self.total_time -= old;
            Some(old)
        } else {
            None
        };

        self.samples[self.cursor] = delta_time;
        self.total_time += delta_time;
        self.cursor = (self.cursor + 1) % 60;
        if self.len < 60 {
            self.len += 1;
        }

        // Incremental min/max update.
        match evicted {
            None => {
                // Window is growing — just extend min/max.
                self.running_max_dt = if self.running_max_dt.is_nan() {
                    delta_time
                } else {
                    self.running_max_dt.max(delta_time)
                };
                self.running_min_dt = if self.running_min_dt.is_nan() {
                    delta_time
                } else {
                    self.running_min_dt.min(delta_time)
                };
            }
            Some(old) => {
                let new_max = self.running_max_dt.max(delta_time);
                let new_min = self.running_min_dt.min(delta_time);

                // If the evicted sample was the max/min we must rescan.
                // Otherwise just clamp toward the new sample.
                let need_rescan_max = (old - self.running_max_dt).abs() < f32::EPSILON;
                let need_rescan_min = (old - self.running_min_dt).abs() < f32::EPSILON;

                if need_rescan_max || need_rescan_min {
                    self.refresh_minmax();
                } else {
                    self.running_max_dt = new_max;
                    self.running_min_dt = new_min;
                }
            }
        }
    }

    /// Returns the average FPS over the current sample window.
    /// Returns `0.0` if no frames have been recorded yet.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn fps(&self) -> f32 {
        if self.len == 0 || self.total_time <= 0.0 {
            return 0.0;
        }
        self.len as f32 / self.total_time
    }

    /// Returns the average frame time in milliseconds over the current window.
    /// Returns `0.0` if no frames have been recorded yet.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn frame_time_ms(&self) -> f32 {
        if self.len == 0 {
            return 0.0;
        }
        (self.total_time / self.len as f32) * 1000.0
    }

    /// Full O(60) rescan — only called when the evicted sample was min or max.
    fn refresh_minmax(&mut self) {
        let mut min_dt = f32::MAX;
        let mut max_dt = 0.0_f32;
        for &dt in &self.samples[..self.len] {
            if dt < min_dt { min_dt = dt; }
            if dt > max_dt { max_dt = dt; }
        }
        self.running_min_dt = if self.len == 0 { 0.0 } else { min_dt };
        self.running_max_dt = if self.len == 0 { 0.0 } else { max_dt };
    }

    /// Returns the lowest FPS recorded within the current sample window.
    #[must_use]
    pub fn min_fps(&mut self) -> f32 {
        if self.len == 0 { return 0.0; }
        if self.running_max_dt.is_nan() { self.refresh_minmax(); }
        if self.running_max_dt > 0.0 { 1.0 / self.running_max_dt } else { 0.0 }
    }

    /// Returns the highest FPS recorded within the current sample window.
    #[must_use]
    pub fn max_fps(&mut self) -> f32 {
        if self.len == 0 { return 0.0; }
        if self.running_min_dt.is_nan() { self.refresh_minmax(); }
        if self.running_min_dt > 0.0 { 1.0 / self.running_min_dt } else { 0.0 }
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

    #[test]
    fn min_max_are_cached_correctly() {
        let mut counter = FpsCounter::new();
        counter.update(1.0 / 30.0);  // slow frame → low fps
        counter.update(1.0 / 120.0); // fast frame → high fps
        assert!(counter.min_fps() < counter.max_fps());
    }
}
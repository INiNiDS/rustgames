use std::time::{Duration, Instant};

pub struct Time {
    start_time: Instant,
    last_update: Instant,
    delta: Duration,
    frame_count: u64,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            delta: Duration::from_secs_f32(1.0 / 60.0),
            frame_count: 0,
        }
    }
    
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_update;
        self.last_update = now;
        self.frame_count += 1;
    }
    
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
    pub fn total_seconds(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}
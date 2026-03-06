use glam::Vec2;
use rand::RngExt;

/// Trauma-based camera shake. Trauma decays linearly and the shake intensity
/// is proportional to `trauma²`, producing a natural dampening feel.
#[derive(Debug, Clone)]
pub struct TraumaShake {
    trauma: f32,
    max_trauma: f32,
    decay_rate: f32,
    max_offset: f32,
    offset: Vec2,
    time: f32,
}

impl TraumaShake {
    /// Creates a new [`TraumaShake`] with no active trauma.
    ///
    /// * `max_offset` — maximum pixel displacement when trauma is at 1.0.
    /// * `decay_rate` — amount of trauma lost per second.
    #[must_use]
    pub const fn new(max_offset: f32, decay_rate: f32) -> Self {
        Self {
            trauma: 0.0,
            max_trauma: 1.0,
            decay_rate,
            max_offset,
            offset: Vec2::ZERO,
            time: 0.0,
        }
    }

    /// Adds `amount` to the trauma accumulator, clamped to `max_trauma`.
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(self.max_trauma);
    }

    /// Advances the shake simulation by `delta_time` seconds.
    /// Decays trauma and recomputes the screen-space offset.
    pub fn update(&mut self, delta_time: f32) {
        self.trauma = self.decay_rate.mul_add(-delta_time, self.trauma).max(0.0);

        if self.trauma > 0.0 {
            self.time += delta_time;

            let shake = self.trauma * self.trauma;

            let mut rng = rand::rng();
            let x = self.time.mul_add(50.0, rng.random::<f32>()).sin();
            let y = self.time.mul_add(45.0, rng.random::<f32>()).cos();

            self.offset = Vec2::new(x, y) * self.max_offset * shake;
        } else {
            self.offset = Vec2::ZERO;
        }
    }

    /// Returns the current screen-space displacement vector in pixels.
    #[must_use]
    pub const fn offset(&self) -> Vec2 {
        self.offset
    }

    /// Returns `true` when trauma is greater than zero (shake still active).
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.trauma > 0.0
    }

    /// Returns the raw trauma value in the range `0.0..=max_trauma`.
    #[must_use]
    pub const fn trauma(&self) -> f32 {
        self.trauma
    }
}

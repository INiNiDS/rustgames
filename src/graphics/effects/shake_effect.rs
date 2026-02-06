use glam::Vec2;
use rand::Rng;

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
    #[must_use] 
    pub fn new(max_offset: f32, decay_rate: f32) -> Self {
        Self {
            trauma: 0.0,
            max_trauma: 1.0,
            decay_rate,
            max_offset,
            offset: Vec2::ZERO,
            time: 0.0,
        }
    }

    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(self.max_trauma);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.trauma = (self.trauma - self.decay_rate * delta_time).max(0.0);

        if self.trauma > 0.0 {
            self.time += delta_time;

            let shake = self.trauma * self.trauma;

            let mut rng = rand::rng();
            let x = (self.time * 50.0 + rng.random::<f32>()).sin();
            let y = (self.time * 45.0 + rng.random::<f32>()).cos();

            self.offset = Vec2::new(x, y) * self.max_offset * shake;
        } else {
            self.offset = Vec2::ZERO;
        }
    }

    #[must_use] 
    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    #[must_use] 
    pub fn is_active(&self) -> bool {
        self.trauma > 0.0
    }

    #[must_use] 
    pub fn trauma(&self) -> f32 {
        self.trauma
    }
}

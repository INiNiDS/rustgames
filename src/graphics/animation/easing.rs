#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear, EaseIn, EaseOut, EaseInOut, Bounce, Elastic,
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => t * (2.0 - t),
            Self::EaseInOut => if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t },
            Self::Bounce => {
                let (n1, d1) = (7.5625, 2.75);
                if t < 1.0 / d1 { n1 * t * t }
                else if t < 2.0 / d1 { let t = t - 1.5 / d1; n1 * t * t + 0.75 }
                else if t < 2.5 / d1 { let t = t - 2.25 / d1; n1 * t * t + 0.9375 }
                else { let t = t - 2.625 / d1; n1 * t * t + 0.984375 }
            },
            Self::Elastic => {
                if t == 0.0 || t == 1.0 { t }
                else {
                    let p = 0.3;
                    let s = p / 4.0;
                    -(2.0_f32.powf(10.0 * (t - 1.0))) * ((t - 1.0 - s) * (std::f32::consts::TAU) / p).sin()
                }
            }
        }
    }
}
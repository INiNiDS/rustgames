use std::f32::consts::TAU;

const N1: f32 = 7.5625;
const D1: f32 = 2.75;

/// Easing functions controlling animation interpolation curves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear, EaseIn, EaseOut, EaseInOut, Bounce, Elastic,
}

impl Easing {
    #[must_use]
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => t * (2.0 - t),
            Self::EaseInOut => if t < 0.5 {
                2.0 * t * t
            } else {
                -1.0 + (4.0 - 2.0 * t) * t
            },
            Self::Bounce => Self::bounce_out(t),
            Self::Elastic => Self::elastic_in(t),
        }
    }

    fn bounce_out(mut t: f32) -> f32 {

        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            t -= 1.5 / D1;
            N1 * t * t + 0.75
        } else if t < 2.5 / D1 {
            t -= 2.25 / D1;
            N1 * t * t + 0.9375
        } else {
            t -= 2.625 / D1;
            N1 * t * t + 0.984375
        }
    }

    fn elastic_in(t: f32) -> f32 {
        if t <= 0.0 || t >= 1.0 {
            return t;
        }

        const P: f32 = 0.3;
        let s = P / 4.0;

        let post_fix = t - 1.0;
        let amplitude = 2.0_f32.powf(10.0 * post_fix);
        let wave = ((post_fix - s) * TAU / P).sin();

        -(amplitude * wave)
    }
}
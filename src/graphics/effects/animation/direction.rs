use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left, Right, Top, Bottom,
    TopLeft, TopRight, BottomLeft, BottomRight,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::TopLeft => Self::BottomRight,
            Self::TopRight => Self::BottomLeft,
            Self::BottomLeft => Self::TopRight,
            Self::BottomRight => Self::TopLeft,
        }
    }

    pub fn to_vector(&self) -> Vec2 {
        let v = match self {
            Self::Left => Vec2::new(-1.0, 0.0),
            Self::Right => Vec2::new(1.0, 0.0),
            Self::Top => Vec2::new(0.0, 1.0),
            Self::Bottom => Vec2::new(0.0, -1.0),
            Self::TopLeft => Vec2::new(-1.0, 1.0),
            Self::TopRight => Vec2::new(1.0, 1.0),
            Self::BottomLeft => Vec2::new(-1.0, -1.0),
            Self::BottomRight => Vec2::new(1.0, -1.0),
        };
        v.normalize_or_zero()
    }
}
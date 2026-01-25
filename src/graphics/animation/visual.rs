use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct VisualState {
    pub opacity: f32,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub anchor: Vec2,
}

impl Default for VisualState {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            position: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
            anchor: Vec2::splat(0.5),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AnimEffect {
    pub opacity_mul: f32,
    pub offset_add: Vec2,
    pub scale_mul: Vec2,
    pub rotation_add: f32,
}

impl Default for AnimEffect {
    fn default() -> Self {
        Self {
            opacity_mul: 1.0,
            offset_add: Vec2::ZERO,
            scale_mul: Vec2::ONE,
            rotation_add: 0.0,
        }
    }
}

impl AnimEffect {
    pub fn with_opacity(opacity: f32) -> Self {
        Self { opacity_mul: opacity, ..Default::default() }
    }

    pub fn with_offset(offset: Vec2) -> Self {
        Self { offset_add: offset, ..Default::default() }
    }

    pub fn with_scale(scale: Vec2) -> Self {
        Self { scale_mul: scale, ..Default::default() }
    }

    pub fn with_rotation(rotation: f32) -> Self {
        Self { rotation_add: rotation, ..Default::default() }
    }

    pub fn combine(self, other: AnimEffect) -> Self {
        Self {
            opacity_mul: self.opacity_mul * other.opacity_mul,
            offset_add: self.offset_add + other.offset_add,
            scale_mul: self.scale_mul * other.scale_mul,
            rotation_add: self.rotation_add + other.rotation_add,
        }
    }

    pub fn apply_to(&self, state: VisualState) -> VisualState {
        VisualState {
            opacity: state.opacity * self.opacity_mul,
            position: state.position + self.offset_add,
            scale: state.scale * self.scale_mul,
            rotation: state.rotation + self.rotation_add,
            anchor: state.anchor,
        }
    }
}
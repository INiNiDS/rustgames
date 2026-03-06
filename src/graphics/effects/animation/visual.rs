use glam::Vec2;

/// The visual state of a renderable element: opacity, position, scale,
/// rotation, and anchor point.
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

/// Determines how an `AnimEffect` field is combined with the base
/// `VisualState` value.
#[derive(Debug, Clone, Copy)]
pub enum CombinedMode {
    /// Use the subsystem's default combination rule for this field.
    Default,
    /// Add the effect value to the base value.
    Add,
    /// Multiply the base value by the effect value.
    Mul,
    /// Replace the base value with the effect value entirely.
    Override,
}

/// Per-field combination modes for applying `AnimEffect` to a `VisualState`.
#[derive(Debug, Clone, Copy)]
pub struct CustomCombinedMode {
    pub(crate) opacity: CombinedMode,
    pub(crate) rotation: CombinedMode,
    pub(crate) scale: CombinedMode,
    pub(crate) position: CombinedMode,
}

impl CustomCombinedMode {
    /// Creates a [`CustomCombinedMode`] with explicit combination rules for
    /// each visual field.
    #[must_use]
    pub const fn new(
        opacity: CombinedMode,
        rotation: CombinedMode,
        scale: CombinedMode,
        position: CombinedMode,
    ) -> Self {
        Self {
            opacity,
            rotation,
            scale,
            position,
        }
    }

    /// Creates a [`CustomCombinedMode`] with a custom opacity rule and
    /// defaults for the other fields.
    #[must_use]
    pub fn with_opacity(opacity: CombinedMode) -> Self {
        Self {
            opacity,
            ..Default::default()
        }
    }

    /// Creates a [`CustomCombinedMode`] with a custom rotation rule and
    /// defaults for the other fields.
    #[must_use]
    pub fn with_rotation(rotation: CombinedMode) -> Self {
        Self {
            rotation,
            ..Default::default()
        }
    }

    /// Creates a [`CustomCombinedMode`] with a custom scale rule and defaults
    /// for the other fields.
    #[must_use]
    pub fn with_scale(scale: CombinedMode) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }

    /// Creates a [`CustomCombinedMode`] with a custom position rule and
    /// defaults for the other fields.
    #[must_use]
    pub fn with_position(position: CombinedMode) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
}

impl Default for CustomCombinedMode {
    fn default() -> Self {
        Self {
            opacity: CombinedMode::Mul,
            rotation: CombinedMode::Add,
            scale: CombinedMode::Mul,
            position: CombinedMode::Add,
        }
    }
}

/// A delta applied to a `VisualState` by an animation frame.
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

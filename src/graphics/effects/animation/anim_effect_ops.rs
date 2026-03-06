use glam::Vec2;

use crate::utils::blend_utils::{blend_val, blend_vec2};
use super::visual::{AnimEffect, CombinedMode, CustomCombinedMode, VisualState};

impl AnimEffect {
    #[must_use]
    pub fn with_opacity(opacity: f32) -> Self {
        Self {
            opacity_mul: opacity,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_offset(offset: Vec2) -> Self {
        Self {
            offset_add: offset,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_scale(scale: Vec2) -> Self {
        Self {
            scale_mul: scale,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_rotation(rotation: f32) -> Self {
        Self {
            rotation_add: rotation,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn combine(self, other: Self) -> Self {
        Self {
            opacity_mul: self.opacity_mul * other.opacity_mul,
            offset_add: self.offset_add + other.offset_add,
            scale_mul: self.scale_mul * other.scale_mul,
            rotation_add: self.rotation_add + other.rotation_add,
        }
    }

    #[must_use]
    pub fn apply_to(
        &self,
        state: VisualState,
        combined_mode: Option<CustomCombinedMode>,
    ) -> VisualState {
        combined_mode.map_or_else(
            || self.apply_to_default(state),
            |config| self.apply_to_config(state, config),
        )
    }

    #[must_use]
    pub fn apply_to_default(&self, state: VisualState) -> VisualState {
        self.apply_to_config(state, CustomCombinedMode::default())
    }

    #[must_use]
    pub fn apply_to_config(&self, state: VisualState, config: CustomCombinedMode) -> VisualState {
        VisualState {
            opacity: blend_val(state.opacity, self.opacity_mul, config.opacity, CombinedMode::Mul),
            position: blend_vec2(state.position, self.offset_add, config.position, CombinedMode::Add),
            scale: blend_vec2(state.scale, self.scale_mul, config.scale, CombinedMode::Mul),
            rotation: blend_val(state.rotation, self.rotation_add, config.rotation, CombinedMode::Add),
            anchor: state.anchor,
        }
    }
}

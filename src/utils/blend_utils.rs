use glam::Vec2;

use crate::graphics::effects::visual::CombinedMode;

/// Blends a scalar `base` value with a `delta` according to `mode`.
///
/// `default_mode` is used when `mode` is [`CombinedMode::Default`] and
/// encodes the *semantic default* for that field (e.g. [`CombinedMode::Mul`]
/// for opacity/rotation, [`CombinedMode::Add`] for position/scale).
#[must_use]
pub fn blend_val(base: f32, delta: f32, mode: CombinedMode, default_mode: CombinedMode) -> f32 {
    let resolved = resolve_mode(mode, default_mode);
    match resolved {
        CombinedMode::Mul => base * delta,
        CombinedMode::Add => base + delta,
        CombinedMode::Override => delta,
        CombinedMode::Default => unreachable!("resolve_mode never returns Default"),
    }
}

/// Blends a [`Vec2`] `base` value with a `delta` according to `mode`.
///
/// `default_mode` is used when `mode` is [`CombinedMode::Default`].
#[must_use]
pub fn blend_vec2(base: Vec2, delta: Vec2, mode: CombinedMode, default_mode: CombinedMode) -> Vec2 {
    let resolved = resolve_mode(mode, default_mode);
    match resolved {
        CombinedMode::Mul => base * delta,
        CombinedMode::Add => base + delta,
        CombinedMode::Override => delta,
        CombinedMode::Default => unreachable!("resolve_mode never returns Default"),
    }
}

/// Replaces [`CombinedMode::Default`] with the provided `default_mode`.
#[inline]
const fn resolve_mode(mode: CombinedMode, default_mode: CombinedMode) -> CombinedMode {
    match mode {
        CombinedMode::Default => default_mode,
        other => other,
    }
}

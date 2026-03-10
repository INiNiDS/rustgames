use crate::graphics::effects::TraumaShake;
use glam::{Mat4, Vec2, Vec3};

/// A 2D orthographic camera with panning, smooth zoom, target following,
/// bounds clamping, and trauma-based screen shake.
pub struct Camera {
    pub position: Vec3,
    pub zoom: f32,
    aspect: f32,
    trauma_shake: TraumaShake,
    target_position: Option<Vec3>,
    follow_speed: f32,
    velocity: Vec3,
    damping: f32,
    target_zoom: f32,
    zoom_speed: f32,
    bounds: Option<(Vec2, Vec2)>,
    viewport_height: f32,
}

impl Camera {
    /// Creates a new [`Camera`] sized to `width × height` pixels, with no
    /// follow target, zoom 1.0, and a default trauma shake configuration.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // Screen dimensions will never exceed 16,777,216
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: Vec3::ZERO,
            zoom: 1.0,
            aspect: width as f32 / height as f32,
            trauma_shake: TraumaShake::new(10.0, 1.0),
            target_position: None,
            follow_speed: 5.0,
            velocity: Vec3::ZERO,
            damping: 10.0,
            target_zoom: 1.0,
            zoom_speed: 5.0,
            bounds: None,
            viewport_height: height as f32,
        }
    }

    /// Updates the aspect ratio and viewport height when the window is resized.
    #[allow(clippy::cast_precision_loss)]
    pub fn resize(&mut self, width: u32, height: u32) {
        if height > 0 {
            self.aspect = width as f32 / height as f32;
            self.viewport_height = height as f32;
        }
    }

    /// Teleports the camera so that `(x, y)` is the center of the view.
    /// Applies bound clamping immediately.
    pub const fn move_to(&mut self, x: f32, y: f32) {
        self.position = Vec3::new(x, y, self.position.z);
        self.apply_bounds();
    }

    /// Moves the camera by `(dx, dy)` world units relative to its current
    /// position. Applies bound clamping immediately.
    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
        self.apply_bounds();
    }

    /// Sets the zoom level immediately. Values below `0.1` are clamped to
    /// `0.1`.
    pub const fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1);
        self.target_zoom = self.zoom;
    }

    /// Smoothly interpolates zoom toward `zoom` at `speed` units/s.
    pub const fn set_zoom_smooth(&mut self, zoom: f32, speed: f32) {
        self.target_zoom = zoom.max(0.1);
        self.zoom_speed = speed;
    }

    /// Adds `trauma` (0.0–1.0) to the screen-shake accumulator. Trauma decays
    /// over time; higher values produce a stronger shake.
    pub fn add_trauma(&mut self, trauma: f32) {
        self.trauma_shake.add_trauma(trauma);
    }

    /// Replaces the trauma shake configuration with a new `max_offset` and
    /// linear `decay_rate`.
    pub const fn configure_trauma_shake(&mut self, max_offset: f32, decay_rate: f32) {
        self.trauma_shake = TraumaShake::new(max_offset, decay_rate);
    }

    /// Enables smooth following of `target` with spring-like `damping`.
    /// The camera accelerates toward `target` at `speed` and decelerates via
    /// the damping coefficient.
    pub const fn follow_smooth(&mut self, target: Vec3, speed: f32, damping: f32) {
        self.target_position = Some(target);
        self.follow_speed = speed;
        self.damping = damping;
    }

    /// Enables simple linear following of `target` at `speed` units/s.
    pub const fn follow(&mut self, target: Vec3, speed: f32) {
        self.target_position = Some(target);
        self.follow_speed = speed;
    }

    /// Disables target following; the camera will remain at its current
    /// position.
    pub const fn stop_follow(&mut self) {
        self.target_position = None;
    }

    /// Constrains the camera center to the rectangle defined by `min` and
    /// `max`. Future calls to `move_to`, `move_by`, and the follow update will
    /// all respect these limits.
    pub const fn set_bounds(&mut self, min: Vec2, max: Vec2) {
        self.bounds = Some((min, max));
    }

    /// Removes the movement bounds set by [`set_bounds`][Self::set_bounds].
    pub const fn clear_bounds(&mut self) {
        self.bounds = None;
    }

    /// Advances the camera simulation by `delta_time` seconds: updates shake,
    /// smoothly interpolates zoom, and moves toward the follow target.
    pub fn update(&mut self, delta_time: f32) {
        self.trauma_shake.update(delta_time);
        if (self.zoom - self.target_zoom).abs() > 0.001 {
            let zoom_diff = self.target_zoom - self.zoom;
            self.zoom += zoom_diff * self.zoom_speed * delta_time;
        }

        self.update_position(delta_time);
    }

    /// Builds the combined view-projection matrix for the current frame,
    /// including the current shake offset.
    #[must_use]
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::from_translation(-self.position);

        let shake_offset = self.get_shake_offset();
        let shake_transform = Mat4::from_translation(shake_offset);

        let half_height = (self.viewport_height / 2.0) / self.zoom;
        let half_width = half_height * self.aspect;

        let projection = Mat4::orthographic_rh_gl(
            -half_width,
            half_width,
            -half_height,
            half_height,
            -100.0,
            100.0,
        );

        projection * shake_transform * view
    }

    /// Converts a `screen_pos` (physical pixels, top-left origin) to world
    /// coordinates for the given `screen_size`.
    #[must_use]
    pub fn screen_to_world(&self, screen_pos: Vec2, screen_size: Vec2) -> Vec2 {
        let ndc = Vec2::new(
            (screen_pos.x / screen_size.x).mul_add(2.0, -1.0),
            (screen_pos.y / screen_size.y).mul_add(-2.0, 1.0),
        );

        Vec2::new(
            (ndc.x * self.aspect).mul_add(self.zoom, self.position.x),
            ndc.y.mul_add(self.zoom, self.position.y),
        )
    }

    const fn apply_bounds(&mut self) {
        if let Some((min, max)) = self.bounds {
            self.position.x = self.position.x.clamp(min.x, max.x);
            self.position.y = self.position.y.clamp(min.y, max.y);
        }
    }

    const fn get_shake_offset(&self) -> Vec3 {
        let o = self.trauma_shake.offset();
        Vec3::new(o.x, o.y, 0.0)
    }

    fn update_position(&mut self, delta_time: f32) {
        if let Some(target) = self.target_position {
            let direction = target - self.position;

            self.velocity += direction * self.follow_speed * delta_time;
            self.velocity *= (-self.damping * delta_time).exp();

            self.position += self.velocity * delta_time;
            self.apply_bounds();
        }
    }
}

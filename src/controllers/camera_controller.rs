use crate::prelude::Camera;
use glam::{Mat4, Vec2, Vec3};

pub struct CameraController {
    camera: Camera
}

impl CameraController {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.resize(width, height);
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.camera.move_to(x, y);
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.camera.move_by(dx, dy);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.camera.set_zoom(zoom);
    }

    pub fn shake(&mut self, duration: f32, intensity: f32) {
        self.camera.shake(duration, intensity);
    }
    
    pub fn add_trauma(&mut self, trauma: f32) {
        self.camera.add_trauma(trauma);
    }
    
    pub fn set_zoom_smooth(&mut self, zoom: f32, speed: f32) {
        self.camera.set_zoom_smooth(zoom, speed);
    }
    
    pub fn follow_smooth(&mut self, target: Vec3, speed: f32, damping: f32) {
        self.camera.follow_smooth(target, speed, damping);
    }
    
    pub fn configure_trauma_shake(&mut self, max_offset: f32, max_angle: f32, decay_rate: f32) {
        self.camera.configure_trauma_shake(max_offset, max_angle, decay_rate);
    }

    pub fn follow(&mut self, target: Vec3, speed: f32) {
        self.camera.follow(target, speed);
    }

    pub fn stop_follow(&mut self) {
        self.camera.stop_follow();
    }

    pub fn set_bounds(&mut self, min: Vec2, max: Vec2) {
        self.camera.set_bounds(min, max);
    }

    pub fn clear_bounds(&mut self) {
        self.camera.clear_bounds();
    }

    pub fn update(&mut self, delta_time: f32) {
        self.camera.update(delta_time);
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        self.camera.build_view_projection_matrix()
    }

    pub fn screen_to_world(&self, screen_pos: Vec2, screen_size: Vec2) -> Vec2 {
        self.camera.screen_to_world(screen_pos, screen_size)
    }

    pub fn position(&self) -> Vec3 {
        self.camera.position
    }

    pub fn zoom(&self) -> f32 {
        self.camera.zoom
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}
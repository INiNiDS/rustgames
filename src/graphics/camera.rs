use glam::{Mat4, Vec2, Vec3};
use rand::Rng;

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

    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    pub fn is_active(&self) -> bool {
        self.trauma > 0.0
    }

    pub fn trauma(&self) -> f32 {
        self.trauma
    }
}


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

    pub fn resize(&mut self, width: u32, height: u32) {
        if height > 0 {
            self.aspect = width as f32 / height as f32;
            self.viewport_height = height as f32;
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.position = Vec3::new(x, y, self.position.z);
        self.apply_bounds();
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
        self.apply_bounds();
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1);
        self.target_zoom = self.zoom;
    }

    pub fn set_zoom_smooth(&mut self, zoom: f32, speed: f32) {
        self.target_zoom = zoom.max(0.1);
        self.zoom_speed = speed;
    }

    pub fn add_trauma(&mut self, trauma: f32) {
        self.trauma_shake.add_trauma(trauma);
    }

    pub fn configure_trauma_shake(&mut self, max_offset: f32, decay_rate: f32) {
        self.trauma_shake = TraumaShake::new(max_offset, decay_rate);
    }

    pub fn follow_smooth(&mut self, target: Vec3, speed: f32, damping: f32) {
        self.target_position = Some(target);
        self.follow_speed = speed;
        self.damping = damping;
    }

    pub fn follow(&mut self, target: Vec3, speed: f32) {
        self.target_position = Some(target);
        self.follow_speed = speed;
    }

    pub fn stop_follow(&mut self) {
        self.target_position = None;
    }

    pub fn set_bounds(&mut self, min: Vec2, max: Vec2) {
        self.bounds = Some((min, max));
    }

    pub fn clear_bounds(&mut self) {
        self.bounds = None;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.trauma_shake.update(delta_time);
        if (self.zoom - self.target_zoom).abs() > 0.001 {
            let zoom_diff = self.target_zoom - self.zoom;
            self.zoom += zoom_diff * self.zoom_speed * delta_time;
        }

        if let Some(target) = self.target_position {
            let direction = target - self.position;

            self.velocity += direction * self.follow_speed * delta_time;
            self.velocity *= (-self.damping * delta_time).exp();
            
            self.position += self.velocity * delta_time;
            self.apply_bounds();
        }
    }

    fn apply_bounds(&mut self) {
        if let Some((min, max)) = self.bounds {
            self.position.x = self.position.x.clamp(min.x, max.x);
            self.position.y = self.position.y.clamp(min.y, max.y);
        }
    }

    fn get_shake_offset(&self) -> Vec3 {
        let mut offset = Vec3::ZERO;

        let trauma_offset = self.trauma_shake.offset();
        offset += Vec3::new(trauma_offset.x, trauma_offset.y, 0.0);
        
        offset
    }

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
            100.0
        );

        projection * shake_transform * view
    }
    
    pub fn screen_to_world(&self, screen_pos: Vec2, screen_size: Vec2) -> Vec2 {
        let ndc = Vec2::new(
            (screen_pos.x / screen_size.x) * 2.0 - 1.0,
            1.0 - (screen_pos.y / screen_size.y) * 2.0,
        );

        Vec2::new(
            ndc.x * self.aspect * self.zoom + self.position.x,
            ndc.y * self.zoom + self.position.y,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trauma_shake_decay() {
        let mut shake = TraumaShake::new(10.0, 0.1);
        shake.add_trauma(1.0);
        
        assert_eq!(shake.trauma(), 1.0);
        assert!(shake.is_active());

        shake.update(0.5);
        assert!((shake.trauma() - 0.5).abs() < 0.01);

        shake.update(0.5);
        assert_eq!(shake.trauma(), 0.0);
        assert!(!shake.is_active());
    }
    
    #[test]
    fn test_trauma_shake_clamping() {
        let mut shake = TraumaShake::new(10.0, 0.1);
        shake.add_trauma(0.8);
        shake.add_trauma(0.5);

        assert_eq!(shake.trauma(), 1.0);
    }
    
    #[test]
    fn test_camera_smooth_zoom() {
        let mut camera = Camera::new(800, 600);
        camera.set_zoom_smooth(2.0, 5.0);
        
        assert_eq!(camera.zoom, 1.0);
        
        camera.update(0.1);
        
        assert!(camera.zoom > 1.0);
    }
    
    #[test]
    fn test_camera_bounds() {
        let mut camera = Camera::new(800, 600);
        camera.set_bounds(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0));
        
        camera.move_to(150.0, 150.0);
        
        assert_eq!(camera.position.x, 100.0);
        assert_eq!(camera.position.y, 100.0);
    }
}

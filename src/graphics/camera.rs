use glam::{Mat4, Vec2, Vec3};

const SHAKE_PATTERN_MULTIPLIER: f32 = 1000.0;

#[derive(Debug, Clone)]
pub struct ShakeEffect {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
    offset: Vec2,
}

impl ShakeEffect {
    pub fn new(intensity: f32, duration: f32) -> Self {
        Self {
            intensity,
            duration,
            elapsed: 0.0,
            offset: Vec2::ZERO,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.elapsed += delta_time;
        
        if self.is_active() {
            let progress = self.elapsed / self.duration;
            let decay = 1.0 - progress;
            
            let t = self.elapsed * 20.0;
            let x = (t.sin() * SHAKE_PATTERN_MULTIPLIER).fract() - 0.5;
            let y = (t.cos() * SHAKE_PATTERN_MULTIPLIER).fract() - 0.5;
            
            self.offset = Vec2::new(x, y) * self.intensity * decay;
        } else {
            self.offset = Vec2::ZERO;
        }
    }
    
    pub fn is_active(&self) -> bool {
        self.elapsed < self.duration
    }
    
    pub fn offset(&self) -> Vec2 {
        self.offset
    }
}

pub struct Camera {
    pub position: Vec3,
    pub zoom: f32,
    aspect: f32,
    shake: Option<ShakeEffect>,
    target_position: Option<Vec3>,
    follow_speed: f32,
    bounds: Option<(Vec2, Vec2)>, // (min, max)
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: Vec3::ZERO,
            zoom: 1.0,
            aspect: width as f32 / height as f32,
            shake: None,
            target_position: None,
            follow_speed: 5.0,
            bounds: None,
        }
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        if height > 0 {
            self.aspect = width as f32 / height as f32;
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
    }
    
    pub fn shake(&mut self, duration: f32, intensity: f32) {
        self.shake = Some(ShakeEffect::new(intensity, duration));
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
        if let Some(ref mut shake) = self.shake {
            shake.update(delta_time);
            if !shake.is_active() {
                self.shake = None;
            }
        }
        
        if let Some(target) = self.target_position {
            let direction = target - self.position;
            self.position += direction * self.follow_speed * delta_time;
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
        if let Some(ref shake) = self.shake {
            let offset = shake.offset();
            Vec3::new(offset.x, offset.y, 0.0)
        } else {
            Vec3::ZERO
        }
    }
    
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let shake_offset = self.get_shake_offset();
        let final_position = self.position + shake_offset;
        
        let view = Mat4::from_translation(-final_position);
        let projection = Mat4::orthographic_rh_gl(
            -self.aspect * self.zoom,
            self.aspect * self.zoom,
            -1.0 * self.zoom,
            1.0 * self.zoom,
            -100.0,
            100.0
        );
        
        projection * view
    }
    
    pub fn screen_to_world(&self, screen_pos: Vec2, screen_size: Vec2) -> Vec2 {
        let normalized = Vec2::new(
            (screen_pos.x / screen_size.x) * 2.0 - 1.0,
            1.0 - (screen_pos.y / screen_size.y) * 2.0,
        );
        
        Vec2::new(
            normalized.x * self.aspect * self.zoom + self.position.x,
            normalized.y * self.zoom + self.position.y,
        )
    }
    
    pub fn world_to_screen(&self, world_pos: Vec2, screen_size: Vec2) -> Vec2 {
        let relative = Vec2::new(
            world_pos.x - self.position.x,
            world_pos.y - self.position.y,
        );
        
        let normalized = Vec2::new(
            relative.x / (self.aspect * self.zoom),
            relative.y / self.zoom,
        );
        
        Vec2::new(
            (normalized.x + 1.0) * 0.5 * screen_size.x,
            (1.0 - normalized.y) * 0.5 * screen_size.y,
        )
    }
}

use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub zoom: f32,
    aspect: f32
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: Vec3::ZERO,
            zoom: 1.0,
            aspect: width as f32 / height as f32
        }
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        if height > 0 {
            self.aspect = width as f32 / height as f32;
        }
    }
    
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::from_translation(-self.position);
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
}

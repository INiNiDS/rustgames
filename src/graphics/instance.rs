use glam::{Mat4, Vec2, Vec4};
use wgpu::{BufferAddress, VertexBufferLayout, VertexFormat};

/// Production-grade instance data for GPU instancing.
/// Memory layout follows std140 rules for WGPU compatibility.
/// Each instance represents a single sprite with full transform and UV data.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteInstance {
    /// 4x4 model transformation matrix (position, rotation, scale)
    /// Aligned to 16 bytes (mat4x4<f32> in WGSL)
    pub model: [[f32; 4]; 4],
    
    /// UV rectangle for texture atlas support (x, y, width, height)
    /// Aligned to 16 bytes (vec4<f32> in WGSL)
    pub uv_rect: [f32; 4],
    
    /// Color tint (r, g, b, a)
    /// Aligned to 16 bytes (vec4<f32> in WGSL)
    pub color: [f32; 4],
}

impl SpriteInstance {
    /// Create a new instance with full transformation data.
    /// 
    /// # Arguments
    /// * `position` - World position (x, y)
    /// * `size` - Size in world units (width, height)
    /// * `rotation` - Rotation in radians
    /// * `uv_rect` - UV coordinates (x, y, width, height) normalized [0, 1]
    /// * `color` - RGBA color tint [0, 1]
    pub fn new(
        position: Vec2,
        size: Vec2,
        rotation: f32,
        uv_rect: Vec4,
        color: Vec4,
    ) -> Self {
        // Build transformation matrix: Translation * Rotation * Scale
        let translation = Mat4::from_translation(position.extend(0.0));
        let rotation_mat = Mat4::from_rotation_z(rotation);
        let scale = Mat4::from_scale(size.extend(1.0));
        
        let model = translation * rotation_mat * scale;
        
        Self {
            model: model.to_cols_array_2d(),
            uv_rect: uv_rect.to_array(),
            color: color.to_array(),
        }
    }
    
    /// Create a simple instance with default UV and color.
    pub fn simple(position: Vec2, size: Vec2) -> Self {
        Self::new(
            position,
            size,
            0.0,
            Vec4::new(0.0, 0.0, 1.0, 1.0), // Full texture UV
            Vec4::ONE, // White color (no tint)
        )
    }
    
    /// Descriptor for instance vertex buffer layout.
    /// This tells WGPU how to read instance data in the shader.
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteInstance>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // Model matrix (4 vec4s = 16 floats)
                // Row 0
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: VertexFormat::Float32x4,
                },
                // Row 1
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32x4,
                },
                // Row 2
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 4,
                    format: VertexFormat::Float32x4,
                },
                // Row 3
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 5,
                    format: VertexFormat::Float32x4,
                },
                // UV rect (vec4)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: 6,
                    format: VertexFormat::Float32x4,
                },
                // Color (vec4)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 20]>() as BufferAddress,
                    shader_location: 7,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instance_size_alignment() {
        // Verify proper alignment for GPU
        let size = std::mem::size_of::<SpriteInstance>();
        // Should be 24 floats = 96 bytes
        assert_eq!(size, 96, "SpriteInstance size must be 96 bytes");
        
        // Verify 16-byte alignment (std140 requirement)
        assert_eq!(size % 16, 0, "SpriteInstance must be 16-byte aligned");
    }
    
    #[test]
    fn test_instance_creation() {
        let instance = SpriteInstance::simple(Vec2::new(100.0, 200.0), Vec2::new(64.0, 64.0));
        
        // Check UV rect is full texture
        assert_eq!(instance.uv_rect, [0.0, 0.0, 1.0, 1.0]);
        
        // Check color is white
        assert_eq!(instance.color, [1.0, 1.0, 1.0, 1.0]);
    }
    
    #[test]
    fn test_instance_with_rotation() {
        let instance = SpriteInstance::new(
            Vec2::ZERO,
            Vec2::ONE,
            std::f32::consts::PI / 2.0,
            Vec4::new(0.0, 0.0, 0.5, 0.5),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
        );
        
        // Verify UV rect
        assert_eq!(instance.uv_rect, [0.0, 0.0, 0.5, 0.5]);
        
        // Verify color
        assert_eq!(instance.color, [1.0, 0.0, 0.0, 1.0]);
    }
}

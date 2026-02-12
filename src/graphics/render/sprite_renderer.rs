use crate::graphics::Camera;
use crate::graphics::render::instance::SpriteInstance;
use crate::graphics::render::pipeline;
use crate::graphics::render::texture::Texture;
use crate::graphics::sprite::{QUAD_INDICES, QUAD_VERTICES};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

/// Instanced sprite renderer using WGPU. Manages the render pipeline, vertex
/// and instance buffers, and camera/texture bind groups.
pub struct SpriteRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    instance_capacity: usize,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
}

impl SpriteRenderer {
    #[must_use]
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = pipeline::create_shader(device);
        let camera_bgl = pipeline::create_camera_bind_group_layout(device);
        let texture_bgl = pipeline::create_texture_bind_group_layout(device);

        let camera_buffer = Self::create_camera_buffer(device);
        let camera_bind_group = Self::create_camera_bind_group(device, &camera_bgl, &camera_buffer);

        let layout = pipeline::create_pipeline_layout(device, &camera_bgl, &texture_bgl);
        let render_pipeline = pipeline::create_render_pipeline(device, config, &shader, &layout);

        let (vertex_buffer, index_buffer, num_indices) = Self::create_quad_buffers(device);
        let initial_capacity = 1000;
        let instance_buffer = Self::create_instance_buffer(device, initial_capacity);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            instance_buffer,
            instance_capacity: initial_capacity,
            texture_bind_group_layout: texture_bgl,
            camera_bind_group,
            camera_buffer,
        }
    }

    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) {
        let matrix = camera.build_view_projection_matrix();
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[matrix]));
    }

    pub fn render(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        device: &wgpu::Device,
        texture: &Texture,
        instance_count: u32,
        start_instance: u32,
    ) {
        if instance_count == 0 {
            return;
        }
        let bind_group = self.create_texture_bind_group(device, texture);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(
            0..self.num_indices,
            0,
            start_instance..(start_instance + instance_count),
        );
    }

    pub fn prepare_batch(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        all_instances: &[SpriteInstance],
    ) {
        if all_instances.is_empty() {
            return;
        }
        if all_instances.len() > self.instance_capacity {
            self.resize_instance_buffer(device, all_instances.len());
        }
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(all_instances),
        );
    }

    fn resize_instance_buffer(&mut self, device: &wgpu::Device, new_capacity: usize) {
        #[allow(clippy::cast_sign_loss)]
        let new_capacity = (new_capacity as f32 * 1.5) as usize;
        self.instance_buffer = Self::create_instance_buffer(device, new_capacity);
        self.instance_capacity = new_capacity;
    }

    fn create_instance_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
        let size = (capacity * size_of::<SpriteInstance>()) as u64;
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_camera_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[glam::Mat4::IDENTITY]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn create_camera_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    fn create_quad_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
        let vb = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ib = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        (vb, ib, QUAD_INDICES.len() as u32)
    }

    fn create_texture_bind_group(
        &self,
        device: &wgpu::Device,
        texture: &Texture,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture_bind_group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_buffer_capacity() {
        let initial_capacity = 1000;
        let instance_size = size_of::<SpriteInstance>();
        let buffer_size = initial_capacity * instance_size;
        assert_eq!(buffer_size, initial_capacity * 96);
    }
}

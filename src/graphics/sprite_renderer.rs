use crate::graphics::sprite::Vertex;
use crate::graphics::texture::Texture;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::graphics::camera::Camera;

pub struct SpriteRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
}

impl SpriteRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = Self::create_shader(device);
        let texture_bind_group_layout = Self::create_texture_bind_group_layout(device);
        let camera_bind_group_layout = Self::create_camera_bind_group_layout(device);

        let camera_buffer = Self::create_camera_buffer(device);
        let camera_bind_group =
            Self::create_camera_bind_group(device, &camera_bind_group_layout, &camera_buffer);

        let pipeline_layout =
            Self::create_pipeline_layout(device, &texture_bind_group_layout, &camera_bind_group_layout);
        let render_pipeline =
            Self::create_render_pipeline(device, config, &shader, &pipeline_layout);

        let (vertex_buffer, index_buffer, num_indices) = Self::create_quad_buffers(device);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture_bind_group_layout,
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
    ) {
        let bind_group = self.create_texture_bind_group(device, texture);

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    fn create_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        })
    }

    fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }

    fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
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

    fn create_pipeline_layout(
        device: &wgpu::Device,
        texture_layout: &wgpu::BindGroupLayout,
        camera_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sprite Render Pipeline Layout"),
            bind_group_layouts: &[texture_layout, camera_layout],
            immediate_size: 0,
        })
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &wgpu::ShaderModule,
        layout: &wgpu::PipelineLayout,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            cache: None,
            multiview_mask: None,
        })
    }

    fn create_quad_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(super::sprite::QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(super::sprite::QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = super::sprite::QUAD_INDICES.len() as u32;

        (vertex_buffer, index_buffer, num_indices)
    }

    fn create_texture_bind_group(&self, device: &wgpu::Device, texture: &Texture) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("diffuse_bind_group"),
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
use crate::graphics::render::instance::SpriteInstance;
use crate::graphics::sprite::Vertex;

/// Creates the WGSL shader module for instanced sprite rendering.
pub fn create_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Instanced Sprite Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    })
}

/// Creates the bind group layout for texture sampling.
pub fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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

/// Creates the bind group layout for the camera uniform buffer.
pub fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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

/// Creates the pipeline layout combining camera and texture bind groups.
pub fn create_pipeline_layout(
    device: &wgpu::Device,
    camera_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Instanced Sprite Render Pipeline Layout"),
        bind_group_layouts: &[camera_layout, texture_layout],
        immediate_size: 0,
    })
}

/// Returns the primitive state used by the render pipeline.
const fn primitive_state() -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
    }
}

/// Returns the multisample state used by the render pipeline.
const fn multisample_state() -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    }
}

/// Creates the full render pipeline for instanced sprite drawing.
pub fn create_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
) -> wgpu::RenderPipeline {
    let targets = [Some(wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Instanced Render Pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[Vertex::desc(), SpriteInstance::desc()],
        },
        primitive: primitive_state(),
        depth_stencil: None,
        multisample: multisample_state(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &targets,
        }),
        cache: None,
        multiview_mask: None,
    })
}

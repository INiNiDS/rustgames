use crate::graphics::Transition;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TransitionUniform {
    progress: f32,
    type_id: u32,
    dir_x: f32,
    dir_y: f32,
}

pub struct TransitionRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    uniform_buffer: wgpu::Buffer,
    sampler: wgpu::Sampler,
}

impl TransitionRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let uniform_buffer = Self::create_uniform_buffer(device);
        let sampler = Self::create_sampler(device);
        let bind_group_layout = Self::create_bind_group_layout(device);
        let pipeline = Self::create_pipeline(device, format, &bind_group_layout);
        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
            sampler,
        }
    }
    fn create_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transition Uniform"),
            size: std::mem::size_of::<TransitionUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
    fn create_sampler(device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        })
    }
    fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Transition BGL"),
            entries: &Self::bind_group_entries(),
        })
    }
    const fn bind_group_entries() -> [wgpu::BindGroupLayoutEntry; 5] {
        [
            Self::texture_entry(0),
            Self::sampler_entry(1),
            Self::texture_entry(2),
            Self::sampler_entry(3),
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ]
    }
    const fn texture_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        }
    }
    const fn sampler_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        }
    }
    fn create_pipeline(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::include_wgsl!("transition.wgsl"));
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Transition Layout"),
            bind_group_layouts: &[bind_group_layout],
            immediate_size: 0,
        });
        Self::build_render_pipeline(device, &layout, &shader, format)
    }
    fn build_render_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        shader: &wgpu::ShaderModule,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let targets = [Some(wgpu::ColorTargetState {
            format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        let comp = wgpu::PipelineCompilationOptions::default();
        let vtx = wgpu::VertexState {
            module: shader,
            entry_point: Option::from("vs_main"),
            compilation_options: comp.clone(),
            buffers: &[],
        };
        let frag = wgpu::FragmentState {
            module: shader,
            entry_point: Option::from("fs_main"),
            compilation_options: comp,
            targets: &targets,
        };
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Transition Pipeline"),
            layout: Some(layout),
            vertex: vtx,
            fragment: Some(frag),
            multiview_mask: None,
            cache: None,
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        })
    }
    fn resolve_transition(transition: &Transition) -> (u32, f32, f32) {
        match transition {
            Transition::Instant => (0, 0.0, 0.0),
            Transition::Fade(_) => (1, 0.0, 0.0),
            Transition::FadeToBlack(_) => (2, 0.0, 0.0),
            Transition::Dissolve(_) => (3, 0.0, 0.0),
            Transition::Wipe { direction, .. } => {
                (4, direction.to_vector().x, direction.to_vector().y)
            }
        }
    }
    fn create_render_bind_group<'a>(
        &'a self,
        device: &wgpu::Device,
        old_view: &'a wgpu::TextureView,
        new_view: &'a wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transition Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(old_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(new_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
            ],
        })
    }
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
        old_view: &wgpu::TextureView,
        new_view: &wgpu::TextureView,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        transition: &Transition,
        progress: f32,
    ) {
        let (type_id, dir_x, dir_y) = Self::resolve_transition(transition);
        let uniform = TransitionUniform {
            progress,
            type_id,
            dir_x,
            dir_y,
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform));
        let bind_group = self.create_render_bind_group(device, old_view, new_view);
        Self::execute_pass(encoder, output_view, &self.pipeline, &bind_group);
    }
    fn execute_pass(
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        bind_group: &wgpu::BindGroup,
    ) {
        let attachment = Self::color_attachment(output_view);
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Transition Pass"),
            color_attachments: &[Some(attachment)],
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
    const fn color_attachment(view: &wgpu::TextureView) -> wgpu::RenderPassColorAttachment<'_> {
        wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        }
    }
}

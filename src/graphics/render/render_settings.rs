use crate::graphics::effects::{AnimationSystem, VisualState};
use crate::graphics::render::TextureSystem;
use crate::graphics::{Camera, Color, SpriteRenderer, VfxSystem};
use crate::prelude::WindowConfig;
use crate::text::font::DEFAULT_NORMAL_FONT;
use crate::text::TextSystem;
use std::sync::Arc;
use wgpu::{Device, PresentMode, Queue, Surface, SurfaceConfiguration};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::Window;

/// Aggregates all GPU resources and controllers needed for rendering a frame.
pub struct RenderSettings {
    pub(crate) background_color: Color,
    pub(crate) base: VisualState,
    pub(crate) sprite_renderer: SpriteRenderer,
    pub(crate) max_width_text: f32,
    pub(crate) max_height_text: f32,
    pub(crate) surface: Surface<'static>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    pub(crate) window: Arc<Window>,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) camera: Camera,
    pub(crate) text_system: TextSystem,
    pub(crate) animation_system: AnimationSystem,
    pub(crate) texture_controller: TextureSystem,
    pub(crate) vfx_system: VfxSystem
}

impl RenderSettings {
    pub async fn new(window: Arc<Window>) -> Self {
        let (surface, adapter, device, queue) = Self::init_graphics(window.clone()).await;

        let size = window.inner_size();
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .map(|mut c| {
                c.present_mode = PresentMode::Fifo;
                c
            })
            .expect("Surface/Adapter mismatch");

        surface.configure(&device, &config);

        let (camera, text_system, sprite_renderer) =
            Self::configure_inner_modules(size, &device, &config);

        Self {
            window,
            surface,
            config,
            device: device.clone(),
            queue: queue.clone(),
            camera,
            text_system,
            sprite_renderer,
            background_color: Color::WHITE,
            base: VisualState::default(),
            max_width_text: size.width as f32,
            max_height_text: size.height as f32,
            animation_system: AnimationSystem::new(),
            texture_controller: TextureSystem::new(device, queue),
            vfx_system: VfxSystem::new(),
        }
    }

    pub fn get_texture_controller(&self) -> &TextureSystem {
        &self.texture_controller
    }

    pub fn get_animation_system(&self) -> &AnimationSystem {
        &self.animation_system
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_text_system(&self) -> &TextSystem {
        &self.text_system
    }

    pub fn get_vfx_system(&self) -> &VfxSystem {
        &self.vfx_system
    }

    pub fn get_background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn get_sprite_renderer(&self) -> &SpriteRenderer {
        &self.sprite_renderer
    }

    pub fn get_visual_state(&self) -> &VisualState {
        &self.base
    }

    pub fn get_max_width_text(&self) -> f32 {
        self.max_width_text
    }

    pub fn get_max_height_text(&self) -> f32 {
        self.max_height_text
    }
    
    pub fn get_surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub fn get_device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn get_queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn get_config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    pub fn get_text_system_mut(&mut self) -> &mut TextSystem {
        &mut self.text_system
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn get_texture_controller_mut(&mut self) -> &mut TextureSystem {
        &mut self.texture_controller
    }

    pub fn get_vfx_system_mut(&mut self) -> &mut VfxSystem {
        &mut self.vfx_system
    }

    pub fn get_animation_system_mut(&mut self) -> &mut AnimationSystem {
        &mut self.animation_system
    }

    pub fn set_window_config(&mut self, config: WindowConfig) {
        self.config.present_mode = if config.vsync { PresentMode::Fifo } else { PresentMode::Immediate };

        self.surface.configure(&self.device, &self.config);

        self.window.set_title(&config.title);

        self.window.set_resizable(config.resizable);

        self.background_color = config.background_color;

        if config.fullscreen {
            self.window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        } else {
            self.window.set_fullscreen(None);
        }

        let new_size = LogicalSize::new(config.width, config.height);
        let _ = self.window.request_inner_size(new_size);
    }

    async fn init_graphics(window: Arc<Window>) -> (Surface<'static>, wgpu::Adapter, Device, Queue) {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("No suitable GPU adapter found");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("Failed to create device");

        (surface, adapter, device, queue)
    }

    fn configure_inner_modules(size: PhysicalSize<u32>, device: &Device, config: &SurfaceConfiguration) -> (Camera, TextSystem, SpriteRenderer) {
        let camera = Camera::new(size.width, size.height);
        let sprite_renderer = SpriteRenderer::new(device, config);
        let text_system = TextSystem::new(device, config, DEFAULT_NORMAL_FONT, None, None, None, None);
        (camera, text_system, sprite_renderer)
    }
}
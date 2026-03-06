use crate::graphics::effects::{AnimationSystem, VisualState};
use crate::graphics::render::TextureSystem;
use crate::graphics::{Camera, Color, SpriteRenderer, VfxSystem};
use crate::prelude::WindowConfig;
use crate::text::font::DEFAULT_NORMAL_FONT;
use crate::text::text_system::TextSystem;
use crate::translation::{DictionarySystem, LanguageSystem, TranslationSystem};
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
    pub(crate) texture_system: TextureSystem,
    pub(crate) vfx_system: VfxSystem,
    pub(crate) language_system: LanguageSystem,
    pub(crate) translation_system: TranslationSystem,
    pub(crate) dictionary_system: DictionarySystem,
}

impl RenderSettings {
    pub async fn new(window: Arc<Window>) -> Self {
        let (surface, adapter, device, queue) = Self::init_graphics(window.clone()).await;
        let size = window.inner_size();
        let device = Arc::new(device);
        let queue = Arc::new(queue);
        let config = Self::create_config(&surface, &adapter, size);
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
            texture_system: TextureSystem::new(device, queue),
            vfx_system: VfxSystem::new(),
            language_system: Default::default(),
            translation_system: Default::default(),
            dictionary_system: Default::default(),
        }
    }

    fn create_config(
        surface: &Surface<'static>,
        adapter: &wgpu::Adapter,
        size: PhysicalSize<u32>,
    ) -> SurfaceConfiguration {
        surface
            .get_default_config(adapter, size.width, size.height)
            .map(|mut c| {
                c.present_mode = PresentMode::Fifo;
                c
            })
            .expect("Surface/Adapter mismatch")
    }

    pub fn set_window_config(&mut self, config: &WindowConfig) {
        self.config.present_mode = if config.vsync {
            PresentMode::Fifo
        } else {
            PresentMode::Immediate
        };
        self.surface.configure(&self.device, &self.config);
        self.window.set_title(&config.title);
        self.window.set_resizable(config.resizable);
        self.background_color = config.background_color;
        Self::apply_fullscreen(&self.window, config);
        let _ = self
            .window
            .request_inner_size(LogicalSize::new(config.width, config.height));

        let lang = config.language.clone();
        let lang_id = lang.id;
        self.language_system.add_language(lang);
        self.language_system.set_current_language(lang_id);
    }

    fn apply_fullscreen(window: &Window, config: &WindowConfig) {
        if config.fullscreen {
            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        } else {
            window.set_fullscreen(None);
        }
    }

    async fn init_graphics(
        window: Arc<Window>,
    ) -> (Surface<'static>, wgpu::Adapter, Device, Queue) {
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window)
            .expect("Failed to create surface");
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

    fn configure_inner_modules(
        size: PhysicalSize<u32>,
        device: &Device,
        config: &SurfaceConfiguration,
    ) -> (Camera, TextSystem, SpriteRenderer) {
        let camera = Camera::new(size.width, size.height);
        let sprite_renderer = SpriteRenderer::new(device, config);
        let text_system = TextSystem::new(
            device,
            config,
            DEFAULT_NORMAL_FONT,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        (camera, text_system, sprite_renderer)
    }
}

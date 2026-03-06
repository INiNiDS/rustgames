use crate::error::GraphicsError;
use crate::graphics::effects::{AnimationSystem, VisualState};
use crate::graphics::render::TextureSystem;
use crate::graphics::{Camera, Color, SpriteRenderer, VfxSystem};
use crate::prelude::WindowConfig;
use crate::text::font::{FontConfig, DEFAULT_NORMAL_FONT};
use crate::text::text_system::TextSystem;
use std::sync::Arc;
use wgpu::{Device, PresentMode, Queue, Surface, SurfaceConfiguration};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::Window;
use crate::translation::{DictionarySystem, LanguageSystem, TranslationSystem};

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
    pub(crate) dictionary_system: DictionarySystem
}

impl RenderSettings {
    /// Creates all GPU resources.
    ///
    /// # Errors
    /// Returns [`GraphicsError`] if surface, adapter or device creation fails.
    pub async fn new(window: Arc<Window>) -> Result<Self, GraphicsError> {
        let (surface, adapter, device, queue) = Self::init_graphics(window.clone()).await?;
        let size = window.inner_size();

        // Wrap early for shared ownership
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let config = Self::create_config(&surface, &adapter, size)?;
        surface.configure(&device, &config);

        // Grouping related logic
        let (camera, text_system, sprite_renderer) =
            Self::configure_inner_modules(size, &device, &config);

        Ok(Self {
            window,
            surface,
            config,
            device: Arc::clone(&device),
            queue: Arc::clone(&queue),
            camera,
            text_system,
            sprite_renderer,
            background_color: Color::WHITE,
            base: VisualState::default(),
            // Casting allow here monitors, so it's not a problem.
            #[allow(clippy::cast_precision_loss)]
            max_width_text: size.width as f32,
            #[allow(clippy::cast_precision_loss)]
            max_height_text: size.height as f32,
            animation_system: AnimationSystem::new(),
            texture_system: TextureSystem::new(device, queue),
            vfx_system: VfxSystem::new(),
            language_system: LanguageSystem::default(),
            translation_system: TranslationSystem::default(),
            dictionary_system: DictionarySystem::default(),
        })
    }

    fn create_config(
        surface: &Surface<'static>,
        adapter: &wgpu::Adapter,
        size: PhysicalSize<u32>,
    ) -> Result<SurfaceConfiguration, GraphicsError> {
        surface
            .get_default_config(adapter, size.width, size.height)
            .map(|mut c| {
                c.present_mode = PresentMode::Fifo;
                c
            })
            .ok_or(GraphicsError::SurfaceConfigMismatch)
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
    ) -> Result<(Surface<'static>, wgpu::Adapter, Device, Queue), GraphicsError> {
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window)
            .map_err(GraphicsError::SurfaceCreationFailed)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .map_err(|_| GraphicsError::AdapterNotFound)?;
        let (device, queue): (Device, Queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(GraphicsError::DeviceCreationFailed)?;
        Ok((surface, adapter, device, queue))
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
            &FontConfig {
                normal: DEFAULT_NORMAL_FONT.to_string(),
                bold: None,
                italic: None,
                medium: None,
                semibold: None,
                light: None,
                extrabold: None,
            },
        );
        (camera, text_system, sprite_renderer)
    }
}

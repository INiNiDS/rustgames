use std::sync::Arc;
use wgpu::{Device, PresentMode, Queue, Surface, SurfaceConfiguration};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::Window;
use crate::controllers::{AnimationController, CameraController, TextController, TextureController, TypewriterController};
use crate::graphics::effects::VisualState;
use crate::graphics::{Camera, Color, SpriteRenderer};
use crate::prelude::WindowConfig;
use crate::text::font::DEFAULT_NORMAL_FONT;
use crate::text::TextSystem;
use crate::text::typewriter::TypewriterInstance;
// pub struct RenderSettings {
//     background_color: Color,
//     base: VisualState,
//     sprite_renderer: SpriteRenderer,
//     max_width_text: f32,
//     max_height_text: f32,
//     surface: Surface<'static>,
//     device: Arc<Device>,
//     queue: Arc<Queue>,
//     window: Arc<winit::window::Window>,
//     config: SurfaceConfiguration,
//     camera_controller: CameraController,
//     text_controller: TextController,
//     typewriter_controller: TypewriterController,
//     animation_controller: AnimationController,
//     texture_controller: TextureController,
// }



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
    pub(crate) window: Arc<winit::window::Window>,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) camera_controller: CameraController,
    pub(crate) text_controller: TextController,
    pub(crate) typewriter_controller: TypewriterController,
    pub(crate) animation_controller: AnimationController,
    pub(crate) texture_controller: TextureController,
}

impl RenderSettings {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    experimental_features: Default::default(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                },
            )
            .await
            .expect("Failed to create device");

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface isn't supported by the adapter.");

        config.present_mode = PresentMode::Fifo;
        surface.configure(&device, &config);

        let device_arc = Arc::new(device);
        let queue_arc = Arc::new(queue);
        let (camera_controller, text_controller, sprite_renderer, typewriter_controller) =
            Self::configure_inner_modules(size, &device_arc, &config);

        Self {
            background_color: Color::WHITE,
            base: VisualState::default(),
            sprite_renderer,
            max_width_text: size.width as f32,
            max_height_text: size.height as f32,
            surface,
            device: device_arc.clone(),
            queue: queue_arc.clone(),
            window,
            config,
            camera_controller,
            text_controller,
            typewriter_controller,
            animation_controller: AnimationController::new(),
            texture_controller: TextureController::new(device_arc, queue_arc),
        }
    }

    pub fn get_camera_controller(&self) -> &CameraController {
        &self.camera_controller
    }

    pub fn get_texture_controller(&self) -> &TextureController {
        &self.texture_controller
    }

    pub fn get_typewriter_controller(&self) -> &TypewriterController {
        &self.typewriter_controller
    }

    pub fn get_animation_controller(&self) -> &AnimationController {
        &self.animation_controller
    }

    pub fn get_text_controller(&self) -> &TextController {
        &self.text_controller
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

    pub fn get_window(&self) -> &Arc<winit::window::Window> {
        &self.window
    }

    pub fn get_config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    pub fn get_text_controller_mut(&mut self) -> &mut TextController {
        &mut self.text_controller
    }

    pub fn get_camera_controller_mut(&mut self) -> &mut CameraController {
        &mut self.camera_controller
    }

    pub fn get_texture_controller_mut(&mut self) -> &mut TextureController {
        &mut self.texture_controller
    }

    pub fn get_animation_controller_mut(&mut self) -> &mut AnimationController {
        &mut self.animation_controller
    }

    pub fn get_typewriter_controller_mut(&mut self) -> &mut TypewriterController {
        &mut self.typewriter_controller
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


    fn configure_inner_modules(size: PhysicalSize<u32>, device: &Device, config: &SurfaceConfiguration) -> (CameraController, TextController, SpriteRenderer, TypewriterController) {
        let typewriter_controller = TypewriterController::new(TypewriterInstance::new());
        let camera = Camera::new(size.width, size.height);
        let camera_controller = CameraController::new(camera);
        let sprite_renderer = SpriteRenderer::new(device, config);
        let text_system = TextSystem::new(device, config, DEFAULT_NORMAL_FONT, None, None, None, None);
        let text_controller = TextController::new(text_system);
        (camera_controller, text_controller, sprite_renderer, typewriter_controller)
    }
}
use std::sync::Arc;
use wgpu::{Device, PresentMode, Queue, Surface, SurfaceConfiguration};
use winit::dpi::LogicalSize;
use crate::controllers::{AnimationController, CameraController, TextController, TextureController, TypewriterController};
use crate::graphics::effects::VisualState;
use crate::graphics::{Color, SpriteRenderer};
use crate::prelude::WindowConfig;

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
    pub fn new(
        camera_controller: CameraController,
        sprite_renderer: SpriteRenderer,
        text_controller: TextController,
        typewriter_controller: TypewriterController,
        animation_controller: AnimationController,
        base: VisualState,
        max_width_text: f32,
        max_height_text: f32,
        surface: Surface<'static>,
        device: Arc<Device>,
        queue: Arc<Queue>,
        window: Arc<winit::window::Window>,
        config: SurfaceConfiguration,
        texture_controller: TextureController,
        background_color: Color,
    ) -> Self {
        Self {
            camera_controller,
            sprite_renderer,
            text_controller,
            typewriter_controller,
            animation_controller,
            base,
            max_width_text,
            max_height_text,
            surface,
            device,
            queue,
            window,
            config,
            texture_controller,
            background_color,
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
}
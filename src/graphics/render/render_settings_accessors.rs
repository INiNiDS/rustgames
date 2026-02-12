use crate::graphics::effects::{AnimationSystem, VisualState};
use crate::graphics::render::TextureSystem;
use crate::graphics::{Camera, Color, SpriteRenderer, VfxSystem};
use crate::text::text_system::TextSystem;
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

use super::RenderSettings;

impl RenderSettings {
    pub const fn get_texture_controller(&self) -> &TextureSystem {
        &self.texture_system
    }

    pub const fn get_animation_system(&self) -> &AnimationSystem {
        &self.animation_system
    }

    pub const fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub const fn get_text_system(&self) -> &TextSystem {
        &self.text_system
    }

    pub const fn get_vfx_system(&self) -> &VfxSystem {
        &self.vfx_system
    }

    pub const fn get_background_color(&self) -> &Color {
        &self.background_color
    }

    pub const fn get_sprite_renderer(&self) -> &SpriteRenderer {
        &self.sprite_renderer
    }

    pub const fn get_visual_state(&self) -> &VisualState {
        &self.base
    }

    pub const fn get_max_width_text(&self) -> f32 {
        self.max_width_text
    }

    pub const fn get_max_height_text(&self) -> f32 {
        self.max_height_text
    }

    pub const fn get_surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub const fn get_device(&self) -> &Arc<Device> {
        &self.device
    }

    pub const fn get_queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub const fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    pub const fn get_config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    pub const fn get_text_system_mut(&mut self) -> &mut TextSystem {
        &mut self.text_system
    }

    pub const fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub const fn get_texture_controller_mut(&mut self) -> &mut TextureSystem {
        &mut self.texture_system
    }

    pub const fn get_vfx_system_mut(&mut self) -> &mut VfxSystem {
        &mut self.vfx_system
    }

    pub const fn get_animation_system_mut(&mut self) -> &mut AnimationSystem {
        &mut self.animation_system
    }
}

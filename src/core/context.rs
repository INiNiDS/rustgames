use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use crate::controllers::text_controller::TextController;
use crate::controllers::texture_controller::TextureController;
use crate::controllers::typewriter_controller::TypewriterController;
use crate::core::CameraController;
use crate::core::time::Time;
use crate::graphics::{AnimationController, VisualState};
use crate::graphics::sprite_renderer::SpriteRenderer;

pub struct Context<'a> {
    pub time: &'a Time,
}

pub struct RenderContext {
    pub camera_controller: CameraController,
    pub sprite_renderer: SpriteRenderer,
    pub text_controller: TextController,
    pub typewriter_controller: TypewriterController,
    pub animation_controller: AnimationController,
    pub base: VisualState,
    pub max_width_text: f32,
    pub max_height_text: f32,
    pub surface: Surface<'static>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub config: SurfaceConfiguration,
    pub texture_controller: TextureController
}
pub mod renderer;
pub mod texture;
pub mod sprite;
pub mod sprite_renderer;
pub mod camera;
pub mod color;
pub mod animation;
pub mod effects;
pub mod instance;
pub mod sprite_animation;

// Re-export commonly used types
pub use color::Color;
pub use sprite::{Sprite, Vertex};
pub use camera::Camera;
pub use texture::Texture;
pub use renderer::Renderer;
pub use animation::{Animation, Transition, Easing, AnimationController, Direction, AnimationInstance, VisualState, AnimEffect, TimelineStep, AnimationGroupID, TimelineBuilder};
pub use effects::{VisualEffect, ParticleEffect, Particle, EffectManager};
pub use instance::SpriteInstance;
pub use sprite_animation::{SpriteAnimation, AnimationMode};
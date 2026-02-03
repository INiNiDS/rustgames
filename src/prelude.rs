// Prelude module for convenient imports
// Re-export commonly used types for easy access

#![allow(unused_imports)]

// Core types
pub use crate::core::{
    Engine, Time, Context, RenderContext,
};

// Graphics types
pub use crate::graphics::{
    Color, Sprite, Camera, Texture, Renderer,
    Animation, Transition, Easing, AnimationController, Direction, AnimationInstance, VisualState, AnimEffect, TimelineStep, AnimationGroupID, TimelineBuilder,
    VisualEffect, ParticleEffect, EffectManager,
};

// Window types
pub use crate::window::{
    Window, WindowConfig,
    Event, EventHandler, EventQueue,
};

// Text types
pub use crate::text::{
    Font,
    TypewriterEffect, TextSpeed,
    TextAlignment, VerticalAlignment, TextStyle,
};

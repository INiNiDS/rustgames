
use crate::core::Engine;

/// The core game trait. Implement this to define custom game logic.
///
/// The engine calls `init` once when the window is ready, then calls `update`
/// each frame before rendering.
pub trait Game {
    /// Called once after the window and renderer have been initialised.
    fn init(&mut self, engine: &mut Engine);
    /// Called every frame. Use `engine` to access input, rendering, and audio.
    fn update(&mut self, engine: &mut Engine);
}
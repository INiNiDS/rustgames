use crate::core::Engine;

/// The core game trait. Implement this to define custom game logic.
///
/// The engine calls `init` once when the window is ready, then calls `update`
/// each frame before rendering.
pub trait Game {
    /// Called once after the window and renderer have been initialized.
    fn init(&mut self, engine: &mut Engine);
    /// Called every frame. Use `engine` to access rendering, and audio.
    fn update(&mut self, engine: &mut Engine);
    /// Internal method called by the engine to trigger an update. Do not call this directly.
    fn handle_update(&mut self, engine: &mut Engine);
}

/// Controls how a `SpriteAnimation` progresses through its frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    /// Play through all frames once and stop on the last frame.
    PlayOnce,
    /// Play through all frames repeatedly from the start.
    Loop,
    /// Play forward to the last frame then backward to the first, repeating.
    PingPong,
}

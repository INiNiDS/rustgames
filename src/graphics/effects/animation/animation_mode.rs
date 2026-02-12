/// Controls how a `SpriteAnimation` progresses through its frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    PlayOnce,
    Loop,
    PingPong,
}

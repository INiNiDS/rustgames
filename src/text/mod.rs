pub mod text;
pub mod font;
pub mod typewriter;
pub mod text_renderer;

pub use font::{Font};
pub use typewriter::{TypewriterEffect, TextSpeed};
pub use text_renderer::{TextAlignment, VerticalAlignment, TextStyle, TextWrapper, RichTextParser, TextSegment};
pub use text::TextSystem;
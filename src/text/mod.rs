pub mod font;
pub mod typewriter;
pub mod text_renderer;
pub mod alignment;
pub mod text_wrapper;
pub mod text_style;
pub mod typewriter_instance;
pub mod text_system;

pub use crate::text::typewriter_instance::TypewriterInstance;
pub use alignment::{TextAlignment, VerticalAlignment};
pub use font::Font;
pub use text_renderer::{RichTextParser, StyledSegment};
pub use text_style::{FontWeight, TextAttributes, TextStyle};
pub use text_wrapper::TextWrapper;
pub use typewriter::{TextSpeed, TypewriterEffect};
pub use text_system::TextSystem;


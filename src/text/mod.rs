pub mod alignment;
pub mod font;
pub(crate) mod text_font_ops;
pub mod text_renderer;
pub mod text_style;
pub mod text_system;
pub mod text_wrapper;
pub mod typewriter;

pub use crate::text::typewriter::TypewriterInstance;
pub use alignment::{TextAlignment, VerticalAlignment};
pub use font::Font;
pub use text_renderer::{RichTextParser, StyledSegment};
pub use text_style::{FontWeight, TextAttributes, TextShadow, TextStyle, TextWrapMode};
pub use text_system::TextSystem;
pub use text_wrapper::TextWrapper;
pub use typewriter::TypewriterEffect;
pub use typewriter::punctuation_config::PunctuationConfig;
pub use typewriter::text_speed::TextSpeed;

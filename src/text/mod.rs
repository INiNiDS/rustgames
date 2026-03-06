pub mod alignment;
pub mod font;
pub(crate) mod text_font_ops;
pub mod text_renderer;
pub(crate) mod text_section;
pub mod text_style;
pub mod text_system;
pub(crate) mod text_typewriter_ops;
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

/// Data bundle passed to [`TextSystem::add_text_by_id`] for creating a
/// translation-aware typewriter effect.
pub struct TextData {
    /// Fallback text shown when no translation is found for `text_id`.
    pub text: String,
    /// Translation key ID, generated via [`generate_id_from_name`].
    /// `0` means no translation lookup is performed.
    pub text_id: u32,
    /// Reveal speed for the typewriter effect.
    pub speed: TextSpeed,
    /// Horizontal screen position in pixels.
    pub x: f32,
    /// Vertical screen position in pixels.
    pub y: f32,
    /// Text style (size, color, alignment, etc.).
    pub style: TextStyle,
    /// Punctuation-based pause configuration.
    pub punctuation_config: PunctuationConfig,
}
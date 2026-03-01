//! Text-related errors for [`crate::text::TextSystem`] and [`crate::text::Font`].

use thiserror::Error;
use crate::error::Diagnostic;

/// Error type for [`crate::text::TextSystem`] and [`crate::text::Font`].
#[derive(Debug, Error)]
pub enum TextError {
    #[error("{}", Self::fmt_load(_0, _1))]
    FontLoadFailed(String, #[source] std::io::Error),

    #[error("{}", Self::fmt_invalid(_0))]
    InvalidFontData(String),

    #[error("{}", Self::fmt_gpu_queue(_0))]
    GpuQueueFailed(String),
}

impl TextError {
    fn fmt_load(path: &str, source: &std::io::Error) -> String {
        Diagnostic {
            code: "T001",
            title: "Failed to load font file",
            location: "TextSystem::new() / Font::load()",
            what: &format!("could not read the font file `{path}`"),
            why:  "the file does not exist at the given path, or the process lacks read permission",
            fix:  "pass a valid path to TextSystem::new(); built-in fonts are at \
                   `src/static/font/` — use the `DEFAULT_NORMAL_FONT` / `DEFAULT_BOLD_FONT` \
                   constants as a fallback",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_invalid(name: &str) -> String {
        Diagnostic {
            code: "T002",
            title: "Invalid font data",
            location: "Font::from_bytes() / Font::load()",
            what: &format!("the bytes for font `{name}` are not a valid TrueType/OpenType font"),
            why:  "the file may be corrupted, truncated, or is not a .ttf/.otf file",
            fix:  "replace the font file with a valid TTF or OTF; \
                   verify with `fc-validate font.ttf` or open it in a font viewer",
            note: None,
        }
        .to_string()
    }

    fn fmt_gpu_queue(inner: &str) -> String {
        Diagnostic {
            code: "T003",
            title: "GPU text queue failed",
            location: "TextSystem::draw()",
            what: "wgpu_text could not upload the queued text sections to the GPU",
            why:  "the text pipeline may be in an invalid state after a resize or device reset",
            fix:  "ensure `TextSystem::resize()` is called on every `WindowResized` event; \
                   if the error persists, recreate the TextSystem via `TextSystem::rebuild_brush()`",
            note: Some(inner.to_string()),
        }
        .to_string()
    }
}

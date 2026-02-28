pub mod events;

use crate::graphics::Color;
pub use events::*;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::window::Window as WinitWindow;
use crate::translation::language::Language;

/// Configuration used to create and customize the application window.
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub fullscreen: bool,
    pub vsync: bool,
    pub background_color: Color,
    pub language: Language
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Game Window".to_string(),
            width: 800,
            height: 600,
            resizable: true,
            fullscreen: false,
            vsync: true,
            background_color: Color::WHITE,
            language: Language::resolve("en_us").unwrap(),
        }
    }
}

impl WindowConfig {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            ..Default::default()
        }
    }
}

/// Thin wrapper around `winit::Window` exposing common operations.
pub struct Window {
    inner: Arc<WinitWindow>,
}

impl Window {
    #[must_use]
    pub const fn new(inner: Arc<WinitWindow>) -> Self {
        Self { inner }
    }

    #[must_use]
    pub const fn inner(&self) -> &Arc<WinitWindow> {
        &self.inner
    }

    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }

    #[must_use]
    pub fn size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
    }

    pub fn set_size(&self, width: u32, height: u32) {
        let size = LogicalSize::new(width, height);
        let _ = self.inner.request_inner_size(size);
    }

    #[must_use]
    pub fn is_fullscreen(&self) -> bool {
        self.inner.fullscreen().is_some()
    }

    pub fn set_fullscreen(&self, enabled: bool) {
        if enabled {
            self.inner
                .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        } else {
            self.inner.set_fullscreen(None);
        }
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        self.inner.set_cursor_visible(visible);
    }

    #[must_use]
    pub fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }

    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }
}

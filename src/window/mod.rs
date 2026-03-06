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
    /// Text shown in the window title bar.
    pub title: String,
    /// Initial width of the client area in physical pixels.
    pub width: u32,
    /// Initial height of the client area in physical pixels.
    pub height: u32,
    /// Whether the user can resize the window by dragging its border.
    pub resizable: bool,
    /// If `true`, the window starts in borderless-fullscreen mode.
    pub fullscreen: bool,
    /// If `true`, the swap-chain uses `Fifo` (`VSync`); otherwise `Immediate`.
    pub vsync: bool,
    /// Color used to clear the render target at the start of each frame.
    pub background_color: Color,
    /// Initial active language used by the translation system.
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
            language: Language::resolve("en_us")
                .unwrap(),
        }
    }
}

impl WindowConfig {
    /// Creates a [`WindowConfig`] with the given title and dimensions,
    /// using sensible defaults for everything else.
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
    /// Wraps a `winit` window handle in a [`Window`].
    #[must_use]
    pub const fn new(inner: Arc<WinitWindow>) -> Self {
        Self { inner }
    }

    /// Returns the underlying `Arc<winit::Window>`.
    #[must_use]
    pub const fn inner(&self) -> &Arc<WinitWindow> {
        &self.inner
    }

    /// Updates the window title bar text.
    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }

    /// Returns the current inner (client-area) size as `(width, height)` in
    /// physical pixels.
    #[must_use]
    pub fn size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
    }

    /// Requests that the window be resized to the given logical dimensions.
    pub fn set_size(&self, width: u32, height: u32) {
        let size = LogicalSize::new(width, height);
        let _ = self.inner.request_inner_size(size);
    }

    /// Returns `true` if the window is currently in fullscreen mode.
    #[must_use]
    pub fn is_fullscreen(&self) -> bool {
        self.inner.fullscreen().is_some()
    }

    /// Enters or exits borderless fullscreen mode.
    pub fn set_fullscreen(&self, enabled: bool) {
        if enabled {
            self.inner
                .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        } else {
            self.inner.set_fullscreen(None);
        }
    }

    /// Shows or hides the mouse cursor while it is over this window.
    pub fn set_cursor_visible(&self, visible: bool) {
        self.inner.set_cursor_visible(visible);
    }

    /// Returns the `HiDPI` scale factor for this window (e.g. `2.0` on Retina
    /// displays).
    #[must_use]
    pub fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }

    /// Schedules a redraw, causing `winit` to emit a `RedrawRequested` event.
    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }
}

use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::window::Window as WinitWindow;

/// Configuration for window creation
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub fullscreen: bool,
    pub vsync: bool,
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

/// Wrapper around winit::Window with additional functionality
pub struct Window {
    inner: Arc<WinitWindow>,
}

impl Window {
    /// Create a new window (note: actual creation happens via event loop)
    pub fn new(inner: Arc<WinitWindow>) -> Self {
        Self { inner }
    }
    
    /// Get the inner winit window
    pub fn inner(&self) -> &Arc<WinitWindow> {
        &self.inner
    }
    
    /// Set the window title
    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }
    
    /// Get the current window size
    pub fn size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
    }
    
    /// Set the window size
    pub fn set_size(&self, width: u32, height: u32) {
        let size = LogicalSize::new(width, height);
        let _ = self.inner.request_inner_size(size);
    }
    
    /// Check if window is in fullscreen mode
    pub fn is_fullscreen(&self) -> bool {
        self.inner.fullscreen().is_some()
    }
    
    /// Set fullscreen mode
    pub fn set_fullscreen(&self, enabled: bool) {
        if enabled {
            self.inner.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        } else {
            self.inner.set_fullscreen(None);
        }
    }
    
    /// Set cursor visibility
    pub fn set_cursor_visible(&self, visible: bool) {
        self.inner.set_cursor_visible(visible);
    }
    
    /// Get the scale factor for HiDPI displays
    pub fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }
    
    /// Request a redraw
    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }
}

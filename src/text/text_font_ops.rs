use crate::prelude::Font;
use crate::text::text_system::TextSystem;
use wgpu::{Device, SurfaceConfiguration};

/// Extension methods for updating individual font slots in the `TextSystem`.
impl TextSystem {
    pub fn update_normal_font(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        path: &str,
    ) {
        self.update_font_slot(device, config, 0, path);
    }

    pub fn update_bold_font(&mut self, device: &Device, config: &SurfaceConfiguration, path: &str) {
        self.update_font_slot(device, config, 1, path);
    }

    pub fn update_italic_font(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        path: &str,
    ) {
        self.update_font_slot(device, config, 2, path);
    }

    pub fn update_medium_font(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        path: &str,
    ) {
        self.update_font_slot(device, config, 3, path);
    }

    pub fn update_semibold_font(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        path: &str,
    ) {
        self.update_font_slot(device, config, 4, path);
    }

    pub fn set_font_by_id(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        font: &Font,
        id: usize,
    ) {
        if id < self.fonts.len() {
            self.fonts[id] = font.to_font_arc();
            self.rebuild_brush(device, config);
        }
    }

    fn update_font_slot(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        slot: usize,
        path: &str,
    ) {
        match Font::load(path) {
            Ok(font) => {
                self.fonts[slot] = font.to_font_arc();
                self.rebuild_brush(device, config);
            }
            Err(e) => {
                use crate::error::TextError;
                let diag = TextError::FontLoadFailed(path.to_string(), e);
                eprintln!("{diag}");
            }
        }
    }
}

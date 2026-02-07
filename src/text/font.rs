use wgpu_text::glyph_brush::ab_glyph::FontArc;

pub const DEFAULT_NORMAL_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/Caveat-Regular.ttf"
);
pub const DEFAULT_BOLD_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/Caveat-Bold.ttf"
);
pub const DEFAULT_MEDIUM_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/Caveat-Medium.ttf"
);
pub const DEFAULT_SEMIBOLD_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/Caveat-SemiBold.ttf"
);

/// A loaded font with raw byte data and a `FontArc` handle for GPU text
/// rendering.
pub struct Font {
    pub name: String,
    pub data: Vec<u8>,
    pub font_arc: FontArc
}


impl Font {
    pub fn load(path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read(path)?;
        let name = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let font_arc = FontArc::try_from_vec(data.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        Ok(Self { name, data , font_arc })
    }
    
    pub fn from_bytes(name: impl Into<String>, data: Vec<u8>) -> Self {
        let font_arc = FontArc::try_from_vec(data.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)).unwrap();
        
        Self {
            name: name.into(),
            data,
            font_arc
        }
    }
    
    pub fn to_font_arc(&self) -> FontArc {
        self.font_arc.clone()
    }

    pub fn default_font() -> Self {
        let data = std::fs::read(DEFAULT_NORMAL_FONT)
            .expect("Could not find the default font file at the specified path");

        let font_arc = FontArc::try_from_vec(data.clone())
            .expect("Default font data is invalid");

        Self {
            name: "Caveat".to_string(),
            data,
            font_arc
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::default_font()
    }}

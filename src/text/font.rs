use crate::error::TextError;
use wgpu_text::glyph_brush::ab_glyph::FontArc;

pub const DEFAULT_NORMAL_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/font/Caveat-Regular.ttf"
);
pub const DEFAULT_BOLD_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/font/Caveat-Bold.ttf"
);
pub const DEFAULT_MEDIUM_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/font/Caveat-Medium.ttf"
);
pub const DEFAULT_SEMIBOLD_FONT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/font/Caveat-SemiBold.ttf"
);

/// A loaded font with raw byte data and a `FontArc` handle for GPU text
/// rendering.
pub struct Font {
    pub name: String,
    pub data: Vec<u8>,
    pub font_arc: FontArc,
}

impl Font {
    /// Loads a font from the given file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or contains invalid font data.
    pub fn load(path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read(path)?;
        let name = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let font_arc = FontArc::try_from_vec(data.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(Self {
            name,
            data,
            font_arc,
        })
    }

    /// Creates a [`Font`] from raw bytes.
    ///
    /// # Errors
    /// Returns [`TextError::InvalidFontData`] if the bytes are not a valid TTF/OTF.
    pub fn from_bytes(name: impl Into<String>, data: Vec<u8>) -> Result<Self, TextError> {
        let name = name.into();
        let font_arc = FontArc::try_from_vec(data.clone())
            .map_err(|_| TextError::InvalidFontData(name.clone()))?;
        Ok(Self { name, data, font_arc })
    }

    #[must_use]
    pub fn to_font_arc(&self) -> FontArc {
        self.font_arc.clone()
    }

    /// Returns the default bundled font.
    ///
    /// # Errors
    /// Returns [`TextError`] if the embedded font file cannot be read or parsed.
    pub fn default_font() -> Result<Self, TextError> {
        let data = std::fs::read(DEFAULT_NORMAL_FONT)
            .map_err(|e| TextError::FontLoadFailed(DEFAULT_NORMAL_FONT.to_string(), e))?;
        let font_arc = FontArc::try_from_vec(data.clone())
            .map_err(|_| TextError::InvalidFontData("Caveat-Regular".to_string()))?;
        Ok(Self {
            name: "Caveat".to_string(),
            data,
            font_arc,
        })
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::default_font().expect("bundled default font is missing or corrupted")
    }
}

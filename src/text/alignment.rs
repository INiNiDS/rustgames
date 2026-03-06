/// Horizontal text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Vertical text alignment within its bounding box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Align text to the top of the bounding box.
    Top,
    /// Center text vertically within the bounding box.
    Middle,
    /// Align text to the bottom of the bounding box.
    Bottom,
}

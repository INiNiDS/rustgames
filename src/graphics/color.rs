use std::str::FromStr;

pub(crate) const NAMED_COLORS: &[(Color, &str)] = &[
    (Color::WHITE, "White"),
    (Color::BLACK, "Black"),
    (Color::RED, "Red"),
    (Color::GREEN, "Green"),
    (Color::BLUE, "Blue"),
    (Color::YELLOW, "Yellow"),
    (Color::CYAN, "Cyan"),
    (Color::MAGENTA, "Magenta"),
    (Color::GRAY, "Gray"),
    (Color::DARK_GRAY, "Dark Gray"),
    (Color::LIGHT_GRAY, "Light Gray"),
    (Color::ORANGE, "Orange"),
    (Color::PURPLE, "Purple"),
    (Color::BROWN, "Brown"),
    (Color::PINK, "Pink"),
    (Color::GOLD, "Gold"),
];

/// An RGBA color stored as four `f32` values in the range `0.0.=1.0`.
///
/// Provides named constants for common colors, conversions from hex strings,
/// u8 tuples, and linear interpolation.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const MAGENTA: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub const GRAY: Self = Self {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const DARK_GRAY: Self = Self {
        r: 0.25,
        g: 0.25,
        b: 0.25,
        a: 1.0,
    };
    pub const LIGHT_GRAY: Self = Self {
        r: 0.75,
        g: 0.75,
        b: 0.75,
        a: 1.0,
    };
    pub const ORANGE: Self = Self {
        r: 1.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    pub const PURPLE: Self = Self {
        r: 0.5,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };
    pub const BROWN: Self = Self {
        r: 0.6,
        g: 0.3,
        b: 0.2,
        a: 1.0,
    };
    pub const PINK: Self = Self {
        r: 1.0,
        g: 0.75,
        b: 0.8,
        a: 1.0,
    };
    pub const GOLD: Self = Self {
        r: 1.0,
        g: 0.84,
        b: 0.0,
        a: 1.0,
    };

    /// Creates a new [`Color`] from four `f32` components.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates an opaque [`Color`] from three `f32` components (`alpha = 1.0`).
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a [`Color`] from four `f32` components (alias for [`new`][Self::new]).
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates an opaque [`Color`] from three `u8` channel values.
    #[must_use]
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: 1.0,
        }
    }

    /// Parses a 6-character hex string (without `#`) into `(r, g, b, 255)`.
    /// Returns `None` if the string is too short or contains non-hex characters.
    #[must_use]
    pub fn parse_rgb_hex(hex: &str) -> Option<(u8, u8, u8, u8)> {
        let r = u8::from_str_radix(hex.get(0..2)?, 16).ok()?;
        let g = u8::from_str_radix(hex.get(2..4)?, 16).ok()?;
        let b = u8::from_str_radix(hex.get(4..6)?, 16).ok()?;
        Some((r, g, b, 255))
    }

    /// Parses an 8-character hex string (without `#`) into `(r, g, b, a)`.
    /// Returns `None` if the string is too short or contains non-hex characters.
    #[must_use]
    pub fn parse_rgba_hex(hex: &str) -> Option<(u8, u8, u8, u8)> {
        let r = u8::from_str_radix(hex.get(0..2)?, 16).ok()?;
        let g = u8::from_str_radix(hex.get(2..4)?, 16).ok()?;
        let b = u8::from_str_radix(hex.get(4..6)?, 16).ok()?;
        let a = u8::from_str_radix(hex.get(6..8)?, 16).ok()?;
        Some((r, g, b, a))
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            return Self::parse_tuple(trimmed).ok_or_else(|| format!("Unknown color name: '{s}'"));
        }

        if trimmed.contains(',') {
            return Self::parse_tuple(trimmed).ok_or_else(|| format!("Unknown color name: '{s}'"));
        }

        if trimmed.starts_with('#') || trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            return Self::from_hex(trimmed).ok_or_else(|| format!("Invalid hex color: '{s}'"));
        }

        Self::parse_named(trimmed).ok_or_else(|| format!("Unknown color name: '{s}'"))
    }
}

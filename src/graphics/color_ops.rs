use super::color::Color;

impl Color {
    #[must_use]
    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: f32::from(a) / 255.0,
        }
    }

    #[must_use]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn to_u32(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        let a = (self.a * 255.0) as u32;
        (r << 24) | (g << 16) | (b << 8) | a
    }

    #[must_use]
    pub const fn with_alpha(self, alpha: f32) -> Self {
        Self { a: alpha, ..self }
    }

    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: (other.r - self.r).mul_add(t, self.r),
            g: (other.g - self.g).mul_add(t, self.g),
            b: (other.b - self.b).mul_add(t, self.b),
            a: (other.a - self.a).mul_add(t, self.a),
        }
    }

    #[must_use]
    pub fn to_wgpu_color(&self) -> wgpu::Color {
        wgpu::Color {
            r: f64::from(self.r),
            g: f64::from(self.g),
            b: f64::from(self.b),
            a: f64::from(self.a),
        }
    }

    #[must_use]
    pub fn get_name(&self) -> Option<&'static str> {
        self.primary_color_name()
            .or_else(|| self.secondary_color_name())
    }

    #[must_use]
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        let (r, g, b, a) = match hex.len() {
            6 => Self::parse_rgb_hex(hex)?,
            8 => Self::parse_rgba_hex(hex)?,
            _ => return None,
        };

        Some(Self::from_rgba_u8(r, g, b, a))
    }

    pub fn parse_tuple(rgb: &str) -> Option<Self> {
        let inner = rgb.trim_matches(|c| c == '(' || c == ')');
        let parts: Vec<&str> = inner.split(',').map(|s| rgb.trim()).collect();

        if parts.len() != 3 || parts.len() != 4 {
            return None
        }

        let r = parts[0].parse::<u8>().ok()?;
        let g = parts[1].parse::<u8>().ok()?;
        let b = parts[2].parse::<u8>().ok()?;
        let a = if parts.len() == 4 {
            parts[3].parse::<u8>().ok()?
        } else {
            255
        };

        Some(Self::from_rgba_u8(r, g, b, a))
    }

    pub fn parse_named(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "white" => Some(Self::WHITE),
            "black" => Some(Self::BLACK),
            "red" => Some(Self::RED),
            "green" => Some(Self::GREEN),
            "blue" => Some(Self::BLUE),
            "yellow" => Some(Self::YELLOW),
            "cyan" => Some(Self::CYAN),
            "magenta" => Some(Self::MAGENTA),
            "gray" => Some(Self::GRAY),
            "dark gray" | "dark_gray" => Some(Self::DARK_GRAY),
            "light gray" | "light_gray" => Some(Self::LIGHT_GRAY),
            "orange" => Some(Self::ORANGE),
            "purple" => Some(Self::PURPLE),
            "brown" => Some(Self::BROWN),
            "pink" => Some(Self::PINK),
            "gold" => Some(Self::GOLD),
            "transparent" => Some(Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }),
            _ => None,
        }
    }

    fn primary_color_name(&self) -> Option<&'static str> {
        if *self == Self::WHITE {
            return Some("White");
        }
        if *self == Self::BLACK {
            return Some("Black");
        }
        if *self == Self::RED {
            return Some("Red");
        }
        if *self == Self::GREEN {
            return Some("Green");
        }
        if *self == Self::BLUE {
            return Some("Blue");
        }
        if *self == Self::YELLOW {
            return Some("Yellow");
        }
        if *self == Self::CYAN {
            return Some("Cyan");
        }
        if *self == Self::MAGENTA {
            return Some("Magenta");
        }
        None
    }

    fn secondary_color_name(&self) -> Option<&'static str> {
        if *self == Self::GRAY {
            return Some("Gray");
        }
        if *self == Self::DARK_GRAY {
            return Some("Dark Gray");
        }
        if *self == Self::LIGHT_GRAY {
            return Some("Light Gray");
        }
        if *self == Self::ORANGE {
            return Some("Orange");
        }
        if *self == Self::PURPLE {
            return Some("Purple");
        }
        if *self == Self::BROWN {
            return Some("Brown");
        }
        if *self == Self::PINK {
            return Some("Pink");
        }
        if *self == Self::GOLD {
            return Some("Gold");
        }
        if self.a == 0.0 && self.r == 0.0 && self.g == 0.0 && self.b == 0.0 {
            return Some("Transparent");
        }
        None
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<[f32; 4]> for Color {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self::rgba(r, g, b, a)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self {
            r: (r & 0xff) as u8,
            g: (g & 0xff) as u8,
            b: (b & 0xff) as u8,
            a: (a & 0xff) as u8,
        }
    }

    pub fn with_default_alpha(r: u32, g: u32, b: u32) -> Self {
        Self::new(r, g, b, 0xff)
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn mix(color1: Color, colors: &[Color]) -> Self {
        let mut all_colors = Vec::with_capacity(colors.len() + 1);
        all_colors.extend_from_slice(colors);
        all_colors.push(color1);

        let count = all_colors.len() as u32;

        let (mut a, mut r, mut g, mut b) = (0u32, 0u32, 0u32, 0u32);
        for c in &all_colors {
            a += c.a as u32;
            r += c.r as u32;
            g += c.g as u32;
            b += c.b as u32;
        }

        Self::new(r / count, g / count, b / count, a / count)
    }

    pub fn from_rgb(code: u32) -> Self {
        Self::with_default_alpha((code >> 16) & 0xff, (code >> 8) & 0xff, code & 0xff)
    }

    pub fn from_argb(code: u32) -> Self {
        Self::new(
            (code >> 16) & 0xff,
            (code >> 8) & 0xff,
            code & 0xff,
            (code >> 24) & 0xff,
        )
    }

    pub fn to_argb(&self) -> u32 {
        ((self.a as u32) << 24)
            | ((self.r as u32) << 16)
            | ((self.g as u32) << 8)
            | (self.b as u32)
    }

    pub fn from_rgba(code: u32) -> Self {
        Self::new(
            (code >> 24) & 0xff,
            (code >> 16) & 0xff,
            (code >> 8) & 0xff,
            code & 0xff,
        )
    }

    pub fn to_rgba(&self) -> u32 {
        ((self.r as u32) << 24)
            | ((self.g as u32) << 16)
            | ((self.b as u32) << 8)
            | (self.a as u32)
    }

    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

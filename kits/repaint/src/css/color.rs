
mod named_color; pub use named_color::*;
use repaint::Color;

pub enum CssColor {
    Named(CssNamedColors),
    HexNoAlpha(u32),
    Hex(u32),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
    Hsl(f32, f32, f32),
    Hsla(f32, f32, f32, f32),
}

impl CssColor {
    pub fn rgba(&self) -> [u8; 4] {
        match self {
            Self::Named(color) => color.rgba(),
            Self::HexNoAlpha(color) => {
                let r = (color >> 16) as u8;
                let g = (color >> 8) as u8;
                let b = *color as u8;
                [r, g, b, 255]
            },
            Self::Hex(color) => {
                let r = (color >> 24) as u8;
                let g = (color >> 16) as u8;
                let b = (color >> 8) as u8;
                let a = *color as u8;
                [r, g, b, a]
            },
            Self::Rgb(r, g, b) => [*r, *g, *b, 255],
            Self::Rgba(r, g, b, a) => [*r, *g, *b, *a],
            Self::Hsl(h, s, l) => {
                let hsl = hsl::HSL {
                    h: *h as f64,
                    s: *s as f64,
                    l: *l as f64,
                };
                let rgb = hsl.to_rgb();
                [rgb.0, rgb.1, rgb.2, 255]
            },
            Self::Hsla(h, s, l, a) => {
                let hsl = hsl::HSL {
                    h: *h as f64,
                    s: *s as f64,
                    l: *l as f64,
                };
                let rgb = hsl.to_rgb();
                [rgb.0, rgb.1, rgb.2, (a * 255.0) as u8]
            },
        }
    }

    pub fn rgb(&self) -> [u8; 3] {
        let rgba = self.rgba();
        [rgba[0], rgba[1], rgba[2]]
    }

    pub fn hsla(&self) -> [f32; 4] {
        match self {
            Self::Named(color) => color.hsla(),
            Self::HexNoAlpha(color) => {
                let r = (color >> 16) as u8;
                let g = (color >> 8) as u8;
                let b = *color as u8;
                let hsl = hsl::HSL::from_rgb(&[r, g, b]);
                [hsl.h as f32, hsl.s as f32, hsl.l as f32, 1.0]
            },
            Self::Hex(color) => {
                let r = (color >> 24) as u8;
                let g = (color >> 16) as u8;
                let b = (color >> 8) as u8;
                let a = *color as u8;
                let hsl = hsl::HSL::from_rgb(&[r, g, b]);
                [hsl.h as f32, hsl.s as f32, hsl.l as f32, a as f32 / 255.0]
            },
            Self::Rgb(r, g, b) => {
                let hsl = hsl::HSL::from_rgb(&[*r, *g, *b]);
                [hsl.h as f32, hsl.s as f32, hsl.l as f32, 1.0]
            },
            Self::Rgba(r, g, b, a) => {
                let hsl = hsl::HSL::from_rgb(&[*r, *g, *b]);
                [hsl.h as f32, hsl.s as f32, hsl.l as f32, *a as f32 / 255.0]
            },
            Self::Hsl(h, s, l) => [*h, *s, *l, 1.0],
            Self::Hsla(h, s, l, a) => [*h, *s, *l, *a],
        }
    }

    pub fn hsl(&self) -> [f32; 3] {
        let hsla = self.hsla();
        [hsla[0], hsla[1], hsla[2]]
    }
}

impl From<CssColor> for Color {
    fn from(color: CssColor) -> Self {
        let rgba = color.rgba();
        Self::new(rgba[0] as f32 / 255.0, rgba[1] as f32 / 255.0, rgba[2] as f32 / 255.0, rgba[3] as f32 / 255.0)
    }
}

impl From<CssNamedColors> for CssColor {
    fn from(color: CssNamedColors) -> Self {
        Self::Named(color)
    }
}

impl From<u32> for CssColor {
    fn from(color: u32) -> Self {
        Self::Hex(color)
    }
}
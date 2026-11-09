use gpui_kit::Hsla;

use super::Hsv;

pub struct PaletteRecipe {
    /// Absolute HSV saturation, 0..=1. Higher values move right in the spectrum.
    pub background_saturation: f32,
    /// Absolute HSV brightness, 0..=1. Lower values move down in the spectrum.
    pub background_brightness: f32,

    /// Absolute HSV saturation, 0..=1. Higher values move right in the spectrum.
    pub foreground_saturation: f32,
    /// Absolute HSV brightness, 0..=1. Lower values move down in the spectrum.
    pub foreground_brightness: f32,
}

impl Default for PaletteRecipe {
    fn default() -> Self {
        Self {
            // Matches the previous mixed background for the initial #3B82F6.
            background_saturation: 0.01835,
            background_brightness: 0.98413,
            foreground_saturation: 1.0,
            foreground_brightness: 0.055,
        }
    }
}

pub struct Palette {
    pub background: Hsla,
    pub foreground: Hsla,
}

impl PaletteRecipe {
    pub fn generate(&self, seed: Hsla) -> Palette {
        Palette {
            background: Hsv::new(
                seed.h,
                self.background_saturation,
                self.background_brightness,
            )
            .to_color(),
            foreground: Hsv::new(
                seed.h,
                self.foreground_saturation,
                self.foreground_brightness,
            )
            .to_color(),
        }
    }
}

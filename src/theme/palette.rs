//! Palette recipe: the small set of numbers that generates theme colors.
//!
//! A recipe does not store colors. It stores *anchors* (saturation and
//! brightness) per theme mode. Hue always comes from the seed color, so changing
//! the seed re-tints the whole palette while keeping the light/dark feel.
//!
//! See `docs/theme-architecture.md` for the full picture.

use gpui_kit::{Hsla, component::ThemeMode};

use super::Hsv;

/// A saturation/brightness pair in HSV space. Both values are absolute, 0..=1.
///
/// Hue is intentionally missing: it is supplied by the seed in
/// [`PaletteRecipe::generate`].
#[derive(Clone, Copy)]
pub struct Anchor {
    /// Higher values move right in the spectrum.
    pub saturation: f32,
    /// Lower values move down in the spectrum.
    pub brightness: f32,
}

/// Anchors for a single theme mode (light or dark).
#[derive(Clone, Copy)]
pub struct ModeRecipe {
    pub background: Anchor,
    pub foreground: Anchor,
}

/// Recipe for both modes. Light and dark are tuned independently, because a
/// dark theme is not just an inverted light theme (dark backgrounds usually
/// need a visible tint, light foregrounds need near-zero saturation).
pub struct PaletteRecipe {
    pub light: ModeRecipe,
    pub dark: ModeRecipe,
}

impl PaletteRecipe {
    /// Returns the anchors that belong to `mode`.
    pub fn mode(&self, mode: ThemeMode) -> &ModeRecipe {
        match mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }
}

impl Default for PaletteRecipe {
    fn default() -> Self {
        Self {
            light: ModeRecipe {
                // Matches the previous mixed background for the initial #3B82F6.
                background: Anchor {
                    saturation: 0.01835,
                    brightness: 0.98413,
                },
                // Near-black, fully saturated: reads as a tinted black.
                foreground: Anchor {
                    saturation: 1.0,
                    brightness: 0.055,
                },
            },
            dark: ModeRecipe {
                // Tinted near-black so the seed hue stays visible.
                background: Anchor {
                    saturation: 0.30,
                    brightness: 0.08,
                },
                // Near-white with a hint of tint.
                foreground: Anchor {
                    saturation: 0.03,
                    brightness: 0.97,
                },
            },
        }
    }
}

/// Colors produced by [`PaletteRecipe::generate`].
///
/// Only the roles driven by the recipe live here. Every other role falls back
/// to library defaults inside `PaletteState`.
pub struct Palette {
    pub background: Hsla,
    pub foreground: Hsla,
}

impl PaletteRecipe {
    /// Builds the generated colors for `mode`.
    ///
    /// `seed` supplies only the hue. Saturation and brightness come from the
    /// matching [`ModeRecipe`]. Pure function: same inputs, same output.
    pub fn generate(&self, seed: Hsla, mode: ThemeMode) -> Palette {
        let recipe = self.mode(mode);
        Palette {
            background: Hsv::new(
                seed.h,
                recipe.background.saturation,
                recipe.background.brightness,
            )
            .to_color(),
            foreground: Hsv::new(
                seed.h,
                recipe.foreground.saturation,
                recipe.foreground.brightness,
            )
            .to_color(),
        }
    }
}

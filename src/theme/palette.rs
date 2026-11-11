//! Palette recipe: the small set of numbers that generates theme colors.
//!
//! A recipe does not store colors. It stores one [`Rule`] per driven role, per
//! theme mode (tables live in `recipe/`). Hue always comes from the seed color,
//! so changing the seed re-tints the whole palette while keeping the
//! light/dark feel.
//!
//! See `docs/theme-architecture.md` for the full picture.

use std::collections::HashMap;

use gpui_kit::{Hsla, component::ThemeMode};

use super::{
    ColorRole, Hsv,
    color::contrast_ratio,
    gpui_bridge,
    recipe::{DARK, LIGHT},
};

/// Minimum contrast ratio for the light option of [`Rule::Contrast`] to win.
/// 3.0 is the WCAG minimum for large text and UI components.
const MIN_LIGHT_CONTRAST: f32 = 3.0;

/// An absolute saturation/brightness pair in HSV space, both 0..=1.
///
/// Hue is missing on purpose: [`Rule::Contrast`] takes it from its base role.
#[derive(Clone, Copy)]
pub struct Tone {
    pub saturation: f32,
    pub brightness: f32,
}

/// How one role's color is derived. All values are HSV, saturation and
/// brightness in 0..=1 (deltas may be negative; results are clamped).
///
/// Hue comes from the seed, except for [`Rule::Contrast`], which uses the hue
/// of its base role.
#[derive(Clone, Copy)]
pub enum Rule {
    /// Absolute saturation/brightness, ignoring the seed's own.
    Anchor { saturation: f32, brightness: f32 },
    /// The seed, shifted by a delta.
    Shift { saturation: f32, brightness: f32 },
    /// A readable color on top of `base`: `light` when it reaches
    /// [`MIN_LIGHT_CONTRAST`] against `base`, `dark` otherwise.
    ///
    /// `base` is read from the colors built so far, so if `base` has its own
    /// rule, that rule must come earlier in the table.
    Contrast {
        base: ColorRole,
        dark: Tone,
        light: Tone,
    },
}

/// Rule table for one theme mode. See `recipe/light.rs` and `recipe/dark.rs`.
pub struct ModeRecipe {
    pub rules: &'static [(ColorRole, Rule)],
}

/// Recipe for both modes. Light and dark are tuned independently, because a
/// dark theme is not just an inverted light theme (dark backgrounds usually
/// need a visible tint, light foregrounds need near-zero saturation).
pub struct PaletteRecipe {
    pub light: ModeRecipe,
    pub dark: ModeRecipe,
}

impl Default for PaletteRecipe {
    fn default() -> Self {
        Self {
            light: ModeRecipe { rules: LIGHT },
            dark: ModeRecipe { rules: DARK },
        }
    }
}

/// Colors produced by [`PaletteRecipe::generate`], keyed by role.
///
/// Holds every role of `ColorRole::ALL`, so indexing with `palette[&role]`
/// never panics. Primary is the seed, roles with a rule are generated, and
/// every other role keeps the library default of the mode.
pub type Palette = HashMap<ColorRole, Hsla>;

impl PaletteRecipe {
    /// Returns the rule table that belongs to `mode`.
    pub fn mode(&self, mode: ThemeMode) -> &ModeRecipe {
        match mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }

    /// Builds the full palette for `mode`.
    ///
    /// Applies the mode's rules in order, so a `Rule::Contrast` can read a role
    /// generated before it. Pure function: same inputs, same output.
    pub fn generate(&self, seed: Hsla, mode: ThemeMode) -> Palette {
        let seed_hsv = Hsv::from_color(seed);
        // Start from the library defaults so a rule can use any role as its
        // base, even one that has no rule of its own (e.g. `Danger`). The seed
        // is the Primary role.
        let defaults = gpui_bridge::default_colors(mode);
        let mut out: Palette = ColorRole::ALL
            .into_iter()
            .map(|role| (role, gpui_bridge::default_color(role, &defaults)))
            .collect();
        out.insert(ColorRole::Primary, seed);
        for (role, rule) in self.mode(mode).rules {
            let color = match *rule {
                Rule::Anchor {
                    saturation,
                    brightness,
                } => Hsv::new(seed_hsv.hue, saturation, brightness),
                Rule::Shift {
                    saturation,
                    brightness,
                } => Hsv::new(
                    seed_hsv.hue,
                    seed_hsv.saturation + saturation,
                    seed_hsv.brightness + brightness,
                ),
                Rule::Contrast { base, dark, light } => {
                    let base_color = *out.get(&base).expect("every role is seeded");

                    let hue = Hsv::from_color(base_color).hue;
                    let dark = Hsv::new(hue, dark.saturation, dark.brightness);
                    let light = Hsv::new(hue, light.saturation, light.brightness);

                    // Prefer light: WCAG ratios favor dark text on saturated
                    // mid-tones (pink, orange) where white reads better.
                    if contrast_ratio(base_color, light.to_color()) >= MIN_LIGHT_CONTRAST {
                        light
                    } else {
                        dark
                    }
                }
            };
            out.insert(*role, color.to_color());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `(contrast_role, base)` pairs where `base` gets its rule at or after
    /// the `Contrast` rule that reads it. `generate` would then read a stale
    /// value (the library default) instead of the generated one.
    fn order_violations(rules: &[(ColorRole, Rule)]) -> Vec<(ColorRole, ColorRole)> {
        rules
            .iter()
            .enumerate()
            .filter_map(|(index, (role, rule))| match *rule {
                Rule::Contrast { base, .. } => rules[index..]
                    .iter()
                    .any(|(later, _)| *later == base)
                    .then_some((*role, base)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn contrast_bases_come_before_their_rule() {
        for (name, rules) in [("LIGHT", LIGHT), ("DARK", DARK)] {
            assert_eq!(
                order_violations(rules),
                [],
                "{name}: move each base rule above the Contrast rule that reads it"
            );
        }
    }

    #[test]
    fn order_check_catches_base_defined_later() {
        const TONE: Tone = Tone {
            saturation: 0.,
            brightness: 0.,
        };
        let rules = [
            (
                ColorRole::ButtonPrimaryForeground,
                Rule::Contrast {
                    base: ColorRole::ButtonPrimary,
                    dark: TONE,
                    light: TONE,
                },
            ),
            (
                ColorRole::ButtonPrimary,
                Rule::Shift {
                    saturation: 0.,
                    brightness: 0.,
                },
            ),
        ];
        assert_eq!(
            order_violations(&rules),
            [(ColorRole::ButtonPrimaryForeground, ColorRole::ButtonPrimary)]
        );
    }
}

use std::collections::HashMap;

use gpui_kit::{component::ThemeMode, rgb};

use super::{ColorRole, Hsv, PaletteRecipe};

/// Initial seed color (`#8CC671`).
const DEFAULT_PRIMARY: u32 = 0x8CC671;

/// One editable color: which role it is, its current value, and whether the
/// user edited it by hand.
pub struct ColorVariable {
    pub role: ColorRole,
    pub value: Hsv,
    /// `true` after a manual edit in the active mode. Customized variables
    /// are skipped by `regenerate`, so the recipe never overwrites a user
    /// choice.
    pub customized: bool,
}

/// Source of truth for every color role shown in the editor.
///
/// Holds one [`ColorVariable`] per [`ColorRole`] for the active mode, the
/// manual edits of each mode, the recipe that generates the rest, and the
/// active theme mode. The mode is *not* decided here: the UI reads it from
/// `Theme::global(cx).mode` and pushes it in via [`set_mode`].
///
/// Primary is the seed and is shared by both modes. Every other manual edit
/// belongs to the mode it was made in, so switching modes keeps the edits of
/// each one.
///
/// [`set_mode`]: PaletteState::set_mode
pub struct PaletteState {
    recipe: PaletteRecipe,
    mode: ThemeMode,
    variables: Vec<ColorVariable>,
    /// Manual edits per mode, Primary excluded.
    overrides: HashMap<ThemeMode, HashMap<ColorRole, Hsv>>,
}

impl PaletteState {
    /// Creates the state for `mode`.
    ///
    /// Primary starts as `DEFAULT_PRIMARY`. Roles with a recipe rule are
    /// generated. Every other role starts from the library defaults of `mode`.
    pub fn new(mode: ThemeMode) -> Self {
        let recipe = PaletteRecipe::default();

        let primary = Hsv::from_color(rgb(DEFAULT_PRIMARY).into());
        let generated = recipe.generate(primary.to_color(), mode);
        let variables = ColorRole::ALL
            .into_iter()
            .map(|role| ColorVariable {
                role,
                value: Hsv::from_color(generated[&role]),
                customized: false,
            })
            .collect();
        Self {
            recipe,
            mode,
            variables,
            overrides: HashMap::new(),
        }
    }

    /// Returns the variable for `role`.
    ///
    /// Panics if the role is missing. That cannot happen: `new` registers all
    /// of `ColorRole::ALL`.
    pub fn variable(&self, role: ColorRole) -> &ColorVariable {
        self.variables
            .iter()
            .find(|variable| variable.role == role)
            .expect("registered color role")
    }

    /// Mutable twin of [`variable`](Self::variable). Same panic guarantee.
    fn variable_mut(&mut self, role: ColorRole) -> &mut ColorVariable {
        self.variables
            .iter_mut()
            .find(|variable| variable.role == role)
            .expect("registered color role")
    }

    /// Applies a manual edit and marks the role as customized.
    ///
    /// Non-Primary edits are stored for the active mode only. Editing Primary
    /// regenerates everything else, because the seed hue changed: the
    /// customizations of *both* modes are cleared.
    pub fn set(&mut self, role: ColorRole, value: Hsv) {
        let value = Hsv::new(value.hue, value.saturation, value.brightness);
        let variable = self.variable_mut(role);
        variable.value = value;
        variable.customized = true;
        if role == ColorRole::Primary {
            self.regenerate_from_primary();
        } else {
            self.overrides
                .entry(self.mode)
                .or_default()
                .insert(role, value);
        }
    }

    /// Drops the manual edit of `role` in the active mode and restores its
    /// generated value. The other mode keeps its own edit.
    ///
    /// Primary has no generated value, so it goes back to `DEFAULT_PRIMARY`.
    pub fn reset(&mut self, role: ColorRole) {
        let variable = self.variable_mut(role);
        variable.customized = false;
        if role == ColorRole::Primary {
            variable.value = Hsv::from_color(rgb(DEFAULT_PRIMARY).into());
            self.regenerate_from_primary();
        } else {
            if let Some(overrides) = self.overrides.get_mut(&self.mode) {
                overrides.remove(&role);
            }
            self.regenerate();
        }
    }

    /// Switches the active mode and rebuilds every color for it.
    ///
    /// No-op when the mode is unchanged. Edits made earlier in `mode` are
    /// restored; every other role is regenerated from the recipe.
    pub fn set_mode(&mut self, mode: ThemeMode) {
        if self.mode == mode {
            return;
        }

        self.mode = mode;

        let overrides = self.overrides.get(&mode);
        for variable in &mut self.variables {
            if variable.role == ColorRole::Primary {
                continue;
            }
            match overrides.and_then(|overrides| overrides.get(&variable.role)) {
                Some(value) => {
                    variable.value = *value;
                    variable.customized = true;
                }
                None => variable.customized = false,
            }
        }

        self.regenerate();
    }

    /// Clears every customization except Primary, in both modes, then
    /// regenerates. Called when the seed hue changes.
    fn regenerate_from_primary(&mut self) {
        self.overrides.clear();
        for variable in &mut self.variables {
            if variable.role != ColorRole::Primary {
                variable.customized = false;
            }
        }
        self.regenerate();
    }

    /// Recomputes every non-customized variable for the current mode.
    ///
    /// Primary is the seed and never regenerated. Every other value is read
    /// from [`PaletteRecipe::generate`], which already resolves rule-driven
    /// roles and library-default fallbacks. To drive another role, add a rule
    /// in `recipe/light.rs` / `recipe/dark.rs`.
    fn regenerate(&mut self) {
        let generated = self.recipe.generate(
            self.variable(ColorRole::Primary).value.to_color(),
            self.mode,
        );

        for variable in &mut self.variables {
            if variable.customized || variable.role == ColorRole::Primary {
                continue;
            }
            variable.value = Hsv::from_color(generated[&variable.role]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: Hsv = Hsv {
        hue: 0.,
        saturation: 0.8,
        brightness: 0.9,
    };

    fn hover(palette: &PaletteState) -> Hsv {
        palette.variable(ColorRole::ButtonPrimaryHover).value
    }

    #[test]
    fn changing_primary_regenerates_dependent_roles() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        let before = hover(&palette);

        palette.set(ColorRole::Primary, RED);

        assert_eq!(palette.variable(ColorRole::Primary).value, RED);
        assert_ne!(hover(&palette), before);
    }

    #[test]
    fn changing_primary_clears_other_customizations() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::ButtonPrimaryHover, Hsv::new(0.5, 0.5, 0.5));

        palette.set(ColorRole::Primary, RED);

        assert!(!palette.variable(ColorRole::ButtonPrimaryHover).customized);
    }

    #[test]
    fn switching_mode_keeps_primary_and_regenerates() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Primary, RED);

        palette.set_mode(ThemeMode::Dark);

        let mut expected = PaletteState::new(ThemeMode::Dark);
        expected.set(ColorRole::Primary, RED);
        for role in ColorRole::ALL {
            assert_eq!(
                palette.variable(role).value,
                expected.variable(role).value,
                "{role:?}"
            );
        }
    }

    const GREY: Hsv = Hsv {
        hue: 0.5,
        saturation: 0.5,
        brightness: 0.5,
    };

    const BLUE: Hsv = Hsv {
        hue: 0.6,
        saturation: 0.7,
        brightness: 0.8,
    };

    fn background(palette: &PaletteState) -> &ColorVariable {
        palette.variable(ColorRole::Background)
    }

    fn generated(mode: ThemeMode, role: ColorRole) -> Hsv {
        PaletteState::new(mode).variable(role).value
    }

    #[test]
    fn light_edit_survives_round_trip() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Background, GREY);

        palette.set_mode(ThemeMode::Dark);
        palette.set_mode(ThemeMode::Light);

        assert_eq!(background(&palette).value, GREY);
        assert!(background(&palette).customized);
    }

    #[test]
    fn dark_edit_survives_round_trip() {
        let mut palette = PaletteState::new(ThemeMode::Dark);
        palette.set(ColorRole::Background, GREY);

        palette.set_mode(ThemeMode::Light);
        palette.set_mode(ThemeMode::Dark);

        assert_eq!(background(&palette).value, GREY);
        assert!(background(&palette).customized);
    }

    #[test]
    fn edits_do_not_leak_between_modes() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Background, GREY);

        palette.set_mode(ThemeMode::Dark);

        assert_eq!(
            background(&palette).value,
            generated(ThemeMode::Dark, ColorRole::Background)
        );
        assert!(!background(&palette).customized);

        palette.set(ColorRole::Background, BLUE);
        palette.set_mode(ThemeMode::Light);
        assert_eq!(background(&palette).value, GREY);
        palette.set_mode(ThemeMode::Dark);
        assert_eq!(background(&palette).value, BLUE);
    }

    #[test]
    fn reset_only_affects_active_mode() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Background, GREY);
        palette.set_mode(ThemeMode::Dark);
        palette.set(ColorRole::Background, BLUE);

        palette.reset(ColorRole::Background);

        assert_eq!(
            background(&palette).value,
            generated(ThemeMode::Dark, ColorRole::Background)
        );
        assert!(!background(&palette).customized);

        palette.set_mode(ThemeMode::Light);
        assert_eq!(background(&palette).value, GREY);
    }

    #[test]
    fn changing_primary_clears_customizations_in_both_modes() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Background, GREY);
        palette.set_mode(ThemeMode::Dark);
        palette.set(ColorRole::Background, BLUE);

        palette.set(ColorRole::Primary, RED);
        assert!(!background(&palette).customized);

        palette.set_mode(ThemeMode::Light);
        let mut expected = PaletteState::new(ThemeMode::Light);
        expected.set(ColorRole::Primary, RED);
        assert!(!background(&palette).customized);
        assert_eq!(
            background(&palette).value,
            expected.variable(ColorRole::Background).value
        );
    }
}

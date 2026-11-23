use std::collections::HashMap;

use gpui_kit::{Hsla, component::ThemeMode, rgb};

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
    /// `true` while the user locked the variable in the active mode. Locked
    /// variables keep their value when Primary changes. Separate from
    /// `customized`: a locked value may be generated or hand-edited.
    pub locked: bool,
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
    /// Locked roles per mode, with the value each one is pinned to. Primary
    /// is never locked.
    locks: HashMap<ThemeMode, HashMap<ColorRole, Hsv>>,
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
                locked: false,
            })
            .collect();
        Self {
            recipe,
            mode,
            variables,
            overrides: HashMap::new(),
            locks: HashMap::new(),
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
            self.sync_lock(role);
        }
    }

    /// Locks or unlocks `role` in the active mode.
    ///
    /// A locked role keeps its current value when Primary changes. Unlocking
    /// only clears the flag: the value stays until the next Primary change
    /// regenerates it. Primary is the seed and cannot be locked.
    pub fn set_locked(&mut self, role: ColorRole, locked: bool) {
        if role == ColorRole::Primary {
            return;
        }
        let mode = self.mode;
        let variable = self.variable_mut(role);
        variable.locked = locked;
        let value = variable.value;
        let locks = self.locks.entry(mode).or_default();
        if locked {
            locks.insert(role, value);
        } else {
            locks.remove(&role);
        }
    }

    /// Flips the lock of `role` in the active mode. See [`set_locked`](Self::set_locked).
    pub fn toggle_locked(&mut self, role: ColorRole) {
        let locked = self.variable(role).locked;
        self.set_locked(role, !locked);
    }

    /// Keeps the pinned value of a locked role in step with its current value.
    fn sync_lock(&mut self, role: ColorRole) {
        let variable = self.variable(role);
        if !variable.locked {
            return;
        }
        let value = variable.value;
        self.locks.entry(self.mode).or_default().insert(role, value);
    }

    /// Drops the manual edit of `role` in the active mode and restores its
    /// generated value. The other mode keeps its own edit.
    ///
    /// Primary has no generated value, so it goes back to `DEFAULT_PRIMARY`.
    /// A locked role takes its generated value and stays locked.
    pub fn reset(&mut self, role: ColorRole) {
        if role == ColorRole::Primary {
            let variable = self.variable_mut(role);
            variable.customized = false;
            variable.value = Hsv::from_color(rgb(DEFAULT_PRIMARY).into());
            self.regenerate_from_primary();
            return;
        }

        if let Some(overrides) = self.overrides.get_mut(&self.mode) {
            overrides.remove(&role);
        }
        let generated = self.generate()[&role];
        let variable = self.variable_mut(role);
        variable.customized = false;
        variable.value = Hsv::from_color(generated);
        self.sync_lock(role);
    }

    /// Switches the active mode and rebuilds every color for it.
    ///
    /// No-op when the mode is unchanged. Edits and locks made earlier in
    /// `mode` are restored; every other role is regenerated from the recipe.
    pub fn set_mode(&mut self, mode: ThemeMode) {
        if self.mode == mode {
            return;
        }

        self.mode = mode;

        let overrides = self.overrides.get(&mode);
        let locks = self.locks.get(&mode);
        for variable in &mut self.variables {
            if variable.role == ColorRole::Primary {
                continue;
            }
            let edit = overrides.and_then(|overrides| overrides.get(&variable.role));
            let pinned = locks.and_then(|locks| locks.get(&variable.role));
            variable.customized = edit.is_some();
            variable.locked = pinned.is_some();
            if let Some(value) = pinned.or(edit) {
                variable.value = *value;
            }
        }

        self.regenerate();
    }

    /// Clears every customization except Primary and locked roles, in both
    /// modes, then regenerates. Called when the seed hue changes.
    fn regenerate_from_primary(&mut self) {
        for (mode, overrides) in &mut self.overrides {
            let locks = self.locks.get(mode);
            overrides.retain(|role, _| locks.is_some_and(|locks| locks.contains_key(role)));
        }
        for variable in &mut self.variables {
            if variable.role != ColorRole::Primary && !variable.locked {
                variable.customized = false;
            }
        }
        self.regenerate();
    }

    /// Recipe output for the current Primary and mode.
    fn generate(&self) -> HashMap<ColorRole, Hsla> {
        self.recipe.generate(
            self.variable(ColorRole::Primary).value.to_color(),
            self.mode,
        )
    }

    /// Recomputes every variable that is neither customized nor locked, for
    /// the current mode.
    ///
    /// Primary is the seed and never regenerated. Every other value is read
    /// from [`PaletteRecipe::generate`], which already resolves rule-driven
    /// roles and library-default fallbacks. To drive another role, add a rule
    /// in `recipe/light.rs` / `recipe/dark.rs`.
    fn regenerate(&mut self) {
        let generated = self.generate();

        for variable in &mut self.variables {
            if variable.customized || variable.locked || variable.role == ColorRole::Primary {
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

    #[test]
    fn locked_variable_keeps_generated_value_after_primary_change() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        let before = background(&palette).value;
        palette.set_locked(ColorRole::Background, true);

        palette.set(ColorRole::Primary, RED);

        assert_eq!(background(&palette).value, before);
        assert!(background(&palette).locked);
        assert_ne!(
            hover(&palette),
            generated(ThemeMode::Light, ColorRole::ButtonPrimaryHover)
        );
    }

    #[test]
    fn locked_edit_survives_primary_change() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Background, GREY);
        palette.set_locked(ColorRole::Background, true);

        palette.set(ColorRole::Primary, RED);

        assert_eq!(background(&palette).value, GREY);
        assert!(background(&palette).customized);
    }

    #[test]
    fn unlocking_keeps_value_until_next_primary_change() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Background, GREY);
        palette.set_locked(ColorRole::Background, true);
        palette.set(ColorRole::Primary, RED);

        palette.set_locked(ColorRole::Background, false);
        assert_eq!(background(&palette).value, GREY);
        assert!(!background(&palette).locked);

        palette.set(ColorRole::Primary, BLUE);
        let mut expected = PaletteState::new(ThemeMode::Light);
        expected.set(ColorRole::Primary, BLUE);
        assert_eq!(
            background(&palette).value,
            expected.variable(ColorRole::Background).value
        );
        assert!(!background(&palette).customized);
    }

    #[test]
    fn lock_is_per_mode_and_survives_primary_change_in_other_mode() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        let light_background = background(&palette).value;
        palette.set_locked(ColorRole::Background, true);

        palette.set_mode(ThemeMode::Dark);
        assert!(!background(&palette).locked);
        palette.set(ColorRole::Primary, RED);

        let mut expected = PaletteState::new(ThemeMode::Dark);
        expected.set(ColorRole::Primary, RED);
        assert_eq!(
            background(&palette).value,
            expected.variable(ColorRole::Background).value
        );

        palette.set_mode(ThemeMode::Light);
        assert!(background(&palette).locked);
        assert_eq!(background(&palette).value, light_background);
    }

    #[test]
    fn editing_locked_variable_updates_pinned_value() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set_locked(ColorRole::Background, true);
        palette.set(ColorRole::Background, GREY);

        palette.set_mode(ThemeMode::Dark);
        palette.set(ColorRole::Primary, RED);
        palette.set_mode(ThemeMode::Light);

        assert_eq!(background(&palette).value, GREY);
    }

    #[test]
    fn reset_on_locked_variable_restores_generated_and_keeps_lock() {
        let mut palette = PaletteState::new(ThemeMode::Light);
        palette.set(ColorRole::Background, GREY);
        palette.set_locked(ColorRole::Background, true);

        palette.reset(ColorRole::Background);

        assert_eq!(
            background(&palette).value,
            generated(ThemeMode::Light, ColorRole::Background)
        );
        assert!(background(&palette).locked);
        assert!(!background(&palette).customized);
    }

    #[test]
    fn primary_cannot_be_locked() {
        let mut palette = PaletteState::new(ThemeMode::Light);

        palette.set_locked(ColorRole::Primary, true);

        assert!(!palette.variable(ColorRole::Primary).locked);
    }
}

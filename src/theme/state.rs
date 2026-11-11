use gpui_kit::{component::ThemeMode, rgb};

use super::{ColorRole, Hsv, PaletteRecipe};

/// Initial seed color (`#8CC671`).
const DEFAULT_PRIMARY: u32 = 0x8CC671;

/// One editable color: which role it is, its current value, and whether the
/// user edited it by hand.
pub struct ColorVariable {
    pub role: ColorRole,
    pub value: Hsv,
    /// `true` after a manual edit. Customized variables are skipped by
    /// `regenerate`, so the recipe never overwrites a user choice.
    pub customized: bool,
}

/// Source of truth for every color role shown in the editor.
///
/// Holds one [`ColorVariable`] per [`ColorRole`], the recipe that generates
/// them, and the active theme mode. The mode is *not* decided here: the UI
/// reads it from `Theme::global(cx).mode` and pushes it in via [`set_mode`].
///
/// [`set_mode`]: PaletteState::set_mode
pub struct PaletteState {
    recipe: PaletteRecipe,
    mode: ThemeMode,
    variables: Vec<ColorVariable>,
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
    /// Editing Primary also regenerates everything else, because the seed hue
    /// changed (all other customizations are cleared).
    pub fn set(&mut self, role: ColorRole, value: Hsv) {
        let variable = self.variable_mut(role);
        variable.value = Hsv::new(value.hue, value.saturation, value.brightness);
        variable.customized = true;
        if role == ColorRole::Primary {
            self.regenerate_from_primary();
        }
    }

    /// Drops the manual edit of `role` and restores its generated value.
    ///
    /// Primary has no generated value, so it goes back to `DEFAULT_PRIMARY`.
    pub fn reset(&mut self, role: ColorRole) {
        let variable = self.variable_mut(role);
        variable.customized = false;
        if role == ColorRole::Primary {
            variable.value = Hsv::from_color(rgb(DEFAULT_PRIMARY).into());
            self.regenerate_from_primary();
        } else {
            self.regenerate();
        }
    }

    /// Switches the active mode and regenerates all colors for it.
    ///
    /// No-op when the mode is unchanged. Manual edits are cleared, because
    /// overrides are not stored per mode yet.
    pub fn set_mode(&mut self, mode: ThemeMode) {
        if self.mode == mode {
            return;
        }

        self.mode = mode;

        for variable in &mut self.variables {
            variable.customized = false;
        }

        self.regenerate();
    }

    /// Clears every customization except Primary, then regenerates.
    /// Called when the seed hue changes.
    fn regenerate_from_primary(&mut self) {
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

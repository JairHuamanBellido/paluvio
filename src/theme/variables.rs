use gpui_kit::rgb;

use super::{Hsv, PaletteRecipe};

/// Stable identity shared by the variable list, editors, and theme mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorRole {
    Primary,
    Background,
    Foreground,
}

impl ColorRole {
    pub const ALL: [Self; 3] = [Self::Primary, Self::Background, Self::Foreground];

    pub fn name(self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Background => "Background",
            Self::Foreground => "Foreground",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Background => "background",
            Self::Foreground => "foreground",
        }
    }
}

pub struct ColorVariable {
    pub role: ColorRole,
    pub value: Hsv,
    pub customized: bool,
}

/// Session state. Recipe supplies defaults; edits override individual variables.
pub struct PaletteState {
    recipe: PaletteRecipe,
    variables: Vec<ColorVariable>,
}

impl PaletteState {
    pub fn new() -> Self {
        let recipe = PaletteRecipe::default();
        let primary = Hsv::from_color(rgb(0x3b82f6).into());
        let generated = recipe.generate(primary.to_color());
        Self {
            recipe,
            variables: vec![
                ColorVariable {
                    role: ColorRole::Primary,
                    value: primary,
                    customized: false,
                },
                ColorVariable {
                    role: ColorRole::Background,
                    value: Hsv::from_color(generated.background),
                    customized: false,
                },
                ColorVariable {
                    role: ColorRole::Foreground,
                    value: Hsv::from_color(generated.foreground),
                    customized: false,
                },
            ],
        }
    }

    pub fn variable(&self, role: ColorRole) -> &ColorVariable {
        self.variables
            .iter()
            .find(|variable| variable.role == role)
            .expect("registered color role")
    }

    pub fn set(&mut self, role: ColorRole, value: Hsv) {
        let variable = self
            .variables
            .iter_mut()
            .find(|variable| variable.role == role)
            .expect("registered color role");
        variable.value = Hsv::new(value.hue, value.saturation, value.brightness);
        variable.customized = true;
        if role == ColorRole::Primary {
            self.regenerate_from_primary();
        }
    }

    pub fn reset(&mut self, role: ColorRole) {
        let variable = self
            .variables
            .iter_mut()
            .find(|variable| variable.role == role)
            .expect("registered color role");
        variable.customized = false;
        if role == ColorRole::Primary {
            variable.value = Hsv::from_color(rgb(0x3b82f6).into());
            self.regenerate_from_primary();
        } else {
            self.regenerate();
        }
    }

    fn regenerate_from_primary(&mut self) {
        for variable in &mut self.variables {
            if variable.role != ColorRole::Primary {
                variable.customized = false;
            }
        }
        self.regenerate();
    }

    fn regenerate(&mut self) {
        let generated = self
            .recipe
            .generate(self.variable(ColorRole::Primary).value.to_color());
        for variable in &mut self.variables {
            if variable.customized {
                continue;
            }
            let color = match variable.role {
                ColorRole::Primary => continue,
                ColorRole::Background => generated.background,
                ColorRole::Foreground => generated.foreground,
            };
            variable.value = Hsv::from_color(color);
        }
    }

    pub fn export(&self) -> String {
        self.variables
            .iter()
            .map(|variable| variable.value.rust_value(variable.role.key()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

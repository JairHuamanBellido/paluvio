mod color;
mod group;

mod palette;
mod role;
mod variables;

pub use color::Hsv;
pub use group::COLOR_GROUP;
pub use palette::PaletteRecipe;
pub use role::ColorRole;
pub use variables::{ColorVariable, PaletteState};

use gpui_kit::{App, component::Theme};

/// Writes generated colors into the live gpui-component `Theme`.
///
/// Only Background and Foreground are mapped so far. Map more roles here as
/// the recipe starts driving them.
pub fn apply(palette: &PaletteState, cx: &mut App) {
    Theme::update(cx, |theme| {
        theme.colors.background = palette.variable(ColorRole::Background).value.to_color();
        theme.colors.foreground = palette.variable(ColorRole::Foreground).value.to_color();
    });
}

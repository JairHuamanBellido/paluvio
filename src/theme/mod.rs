mod color;
mod palette;
mod variables;

pub use color::Hsv;
pub use palette::PaletteRecipe;
pub use variables::{ColorRole, ColorVariable, PaletteState};

use gpui_kit::{App, component::Theme};

pub fn apply(palette: &PaletteState, cx: &mut App) {
    Theme::update(cx, |theme| {
        theme.colors.background = palette.variable(ColorRole::Background).value.to_color();
        theme.colors.foreground = palette.variable(ColorRole::Foreground).value.to_color();
    });
}

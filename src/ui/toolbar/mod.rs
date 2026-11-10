use gpui_kit::*;
mod theme_mode;

pub use theme_mode::{ThemeModeSelector, ThemePreference};

pub fn toolbar(theme_mode: &Entity<ThemeModeSelector>) -> impl IntoElement {
    div().child(theme_mode.clone())
}

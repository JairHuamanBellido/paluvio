use gpui_kit::{component::ActiveTheme, *};
mod primary_color;
mod theme_mode;

pub use theme_mode::{ThemeModeSelector, ThemePreference};

use crate::{theme::Hsv, ui::color_editor::ColorEditor};
use primary_color::primary_color_picker;

pub fn toolbar(
    theme_mode: &Entity<ThemeModeSelector>,
    primary: Hsv,
    primary_editor: &Entity<ColorEditor>,
    cx: &App,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_3()
        .child(theme_mode.clone())
        .child(div().w(px(1.)).h(px(20.)).bg(cx.theme().border))
        .child(primary_color_picker(primary, primary_editor, cx))
}

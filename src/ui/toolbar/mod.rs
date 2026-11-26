use gpui_kit::{component::ActiveTheme, *};
mod content_tabs;
mod palette_strategy;
mod primary_color;
mod theme_mode;

pub use content_tabs::content_tabs;
pub use palette_strategy::{PaletteStrategyChanged, PaletteStrategySelector};
pub use theme_mode::{ThemeModeSelector, ThemePreference};

use crate::{theme::Hsv, ui::color_editor::ColorEditor};
use primary_color::primary_color_picker;

pub fn toolbar(
    palette_strategy: &Entity<PaletteStrategySelector>,
    theme_mode: &Entity<ThemeModeSelector>,
    primary: Hsv,
    primary_editor: &Entity<ColorEditor>,
    cx: &App,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .child(palette_strategy.clone())
                .child(theme_mode.clone()),
        )
        .child(div().w(px(1.)).h(px(20.)).bg(cx.theme().border))
        .child(primary_color_picker(primary, primary_editor, cx))
}

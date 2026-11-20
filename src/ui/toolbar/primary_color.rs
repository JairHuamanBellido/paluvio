use gpui_kit::{
    component::{ActiveTheme, button::Button, popover::Popover},
    *,
};

use crate::{theme::Hsv, ui::color_editor::ColorEditor};

/// Toolbar shortcut for editing the Primary seed color.
///
/// Reuses the same `ColorEditor` as the sidebar row, so both stay in sync.
pub fn primary_color_picker(
    value: Hsv,
    editor: &Entity<ColorEditor>,
    cx: &App,
) -> impl IntoElement {
    Popover::new("primary-color-popup")
        .anchor(Anchor::TopRight)
        .p_3()
        .trigger(
            Button::new("primary-color-trigger")
                .accessibility_label("Edit primary color")
                .tooltip("Edit primary color")
                .border_0()
                .dropdown_caret(true)
                .gap_2()
                .child(
                    div()
                        .size(px(16.))
                        .flex_shrink_0()
                        .rounded_full()
                        .bg(value.to_color())
                        .border_1()
                        .border_color(cx.theme().border),
                )
                // Fixed slot: hex glyphs have different widths in Manrope, so
                // a content-sized label would resize the button while dragging.
                .child(
                    div()
                        .w(px(68.))
                        .flex_shrink_0()
                        .text_sm()
                        .whitespace_nowrap()
                        .child(value.to_hex()),
                ),
        )
        .child(editor.clone())
}

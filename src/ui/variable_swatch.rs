use gpui_kit::{
    component::{ActiveTheme, button::Button, popover::Popover},
    *,
};

use super::color_editor::ColorEditor;
use crate::theme::ColorVariable;

pub fn variable_swatch(
    variable: &ColorVariable,
    editor: Entity<ColorEditor>,
    cx: &App,
) -> impl IntoElement {
    let color = variable.value.to_color();
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(variable.role.name())
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    Popover::new(("color-popup", editor.entity_id()))
                        .anchor(Anchor::TopLeft)
                        .p_3()
                        .trigger(
                            Button::new(("edit-color", editor.entity_id()))
                                .accessibility_label(format!("Edit {} color", variable.role.name()))
                                .w(px(28.))
                                .h(px(28.))
                                .p_0()
                                .rounded_md()
                                .bg(color)
                                .border_1()
                                .border_color(cx.theme().border)
                                .child(div().size_full().rounded_md().bg(color)),
                        )
                        .child(editor),
                )
                .child(div().text_sm().child(variable.value.to_hex())),
        )
}

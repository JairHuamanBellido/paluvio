use gpui_kit::{
    component::{
        ActiveTheme,
        button::{Button, ButtonCustomVariant, ButtonVariants},
        popover::Popover,
    },
    *,
};

use super::color_editor::ColorEditor;
use crate::theme::ColorVariable;

fn swatch_dot(color: Hsla, border: Hsla) -> Div {
    div()
        .size(px(16.))
        .flex_shrink_0()
        .rounded_full()
        .bg(color)
        .border_1()
        .border_color(border)
}

fn swatch_label(name: &'static str) -> Div {
    div().min_w_0().text_ellipsis().child(name)
}

fn swatch_row(color: Hsla, name: &'static str, border: Hsla) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .items_center()
        .gap_3()
        .text_sm()
        .child(swatch_dot(color, border))
        .child(swatch_label(name))
}

pub fn variable_swatch(
    variable: &ColorVariable,
    editor: Entity<ColorEditor>,
    cx: &App,
) -> impl IntoElement {
    let color = variable.value.to_color();
    let row_style = ButtonCustomVariant::new(cx)
        .foreground(cx.theme().sidebar_foreground)
        .hover(rgb(0xffffff).into())
        .active(rgb(0xffffff).into());
    div().w_full().child(
        Popover::new(("color-popup", editor.entity_id()))
            .w_full()
            .anchor(Anchor::TopLeft)
            .p_3()
            .trigger(
                Button::new(("edit-color", editor.entity_id()))
                    .accessibility_label(format!("Edit {} color", variable.role.name()))
                    .custom(row_style)
                    .w_full()
                    .h_auto()
                    .px_3()
                    .py_2()
                    .rounded_lg()
                    .cursor_pointer()
                    .child(swatch_row(color, variable.role.name(), cx.theme().border)),
            )
            .child(editor),
    )
}

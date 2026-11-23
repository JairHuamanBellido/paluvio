use std::rc::Rc;

use gpui_kit::{
    component::{
        ActiveTheme, Sizable,
        button::{Button, ButtonCustomVariant, ButtonVariants},
        popover::Popover,
    },
    prelude::FluentBuilder,
    *,
};

use super::color_editor::ColorEditor;
use crate::theme::{ColorRole, ColorVariable};

/// Called with the role whose lock was toggled.
pub type ToggleLock = Rc<dyn Fn(&ColorRole, &mut Window, &mut App)>;

/// Hover group of one swatch row; reveals the lock toggle.
const SWATCH_GROUP: &str = "variable-swatch";

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

/// Lock toggle at the right of a row. Hidden until the row is hovered,
/// always shown while the variable is locked.
fn lock_toggle(variable: &ColorVariable, on_toggle: ToggleLock, cx: &App) -> impl IntoElement {
    let role = variable.role;
    let locked = variable.locked;
    let (icon, label) = if locked {
        (icon!("lock-simple-icon.svg"), "Unlock")
    } else {
        (icon!("lock-simple-open-icon.svg"), "Lock")
    };
    let color = if locked {
        cx.theme().sidebar_foreground
    } else {
        cx.theme().muted_foreground
    };
    let style = ButtonCustomVariant::new(cx)
        .foreground(color)
        .hover(cx.theme().border)
        .active(cx.theme().border);

    div()
        .absolute()
        .top_0()
        .bottom_0()
        .right_2()
        .flex()
        .items_center()
        .when(!locked, |this| {
            this.invisible()
                .group_hover(SWATCH_GROUP, |this| this.visible())
        })
        .child(
            Button::new(("lock-color", role as usize))
                .accessibility_label(format!("{label} {} color", role.name()))
                .tooltip(label)
                .custom(style)
                .small()
                .icon(icon)
                .toggled(locked)
                .cursor_pointer()
                .on_click(move |_, window, cx| {
                    cx.stop_propagation();
                    on_toggle(&role, window, cx);
                }),
        )
}

pub fn variable_swatch(
    variable: &ColorVariable,
    editor: Entity<ColorEditor>,
    on_toggle_lock: ToggleLock,
    cx: &App,
) -> impl IntoElement {
    let color = variable.value.to_color();
    let row_style = ButtonCustomVariant::new(cx)
        .foreground(cx.theme().sidebar_foreground)
        .hover(cx.theme().muted)
        .active(cx.theme().muted);
    let lockable = variable.role != ColorRole::Primary;
    div()
        .relative()
        .w_full()
        .group(SWATCH_GROUP)
        .child(
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
                        .pl_3()
                        .pr_10()
                        .py_2()
                        .rounded_lg()
                        .cursor_pointer()
                        .child(swatch_row(color, variable.role.name(), cx.theme().border)),
                )
                .child(editor),
        )
        .when(lockable, |this| {
            this.child(lock_toggle(variable, on_toggle_lock, cx))
        })
}

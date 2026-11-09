use gpui_kit::{component::ActiveTheme, *};

use crate::{
    theme::{COLOR_GROUP, ColorRole, PaletteState},
    ui::{color_editor::ColorEditor, variable_swatch::variable_swatch},
};

pub fn app_sidebar(
    palette: &PaletteState,
    editors: &[(ColorRole, Entity<ColorEditor>)],
    cx: &App,
) -> impl IntoElement {
    let sections = COLOR_GROUP
        .iter()
        .map(|group| {
            let swatch_colors_rows = group.roles.iter().map(|role| {
                let (_, editor) = editors
                    .iter()
                    .find(|(editor_role, _)| editor_role == role)
                    .expect("registered color editor");

                variable_swatch(palette.variable(*role), editor.clone(), cx).into_any_element()
            });

            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .px_3()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(cx.theme().muted_foreground)
                        .child(group.name),
                )
                .child(div().flex().flex_col().gap_1().children(swatch_colors_rows))
                .into_any_element()
        })
        .collect::<Vec<_>>();
    div()
        .id("color-sidebar")
        .w(px(300.))
        .min_w(px(300.))
        .max_w(px(300.))
        .h_full()
        .flex_shrink_0()
        .overflow_y_scroll()
        .px_3()
        .flex()
        .flex_col()
        .gap_5()
        .child(div().flex().flex_col().gap_4().children(sections))
}

mod search;

use gpui_kit::{component::ActiveTheme, *};

pub use search::{matches_query, search_bar};

use crate::{
    theme::{COLOR_GROUP, ColorRole, PaletteState},
    ui::{
        color_editor::ColorEditor,
        variable_swatch::{ToggleLock, variable_swatch},
    },
};

use search::empty_state;

pub fn app_sidebar(
    palette: &PaletteState,
    editors: &[(ColorRole, Entity<ColorEditor>)],
    search: impl IntoElement,
    query: SharedString,
    on_clear_search: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_toggle_lock: ToggleLock,
    cx: &App,
) -> impl IntoElement {
    let sections = COLOR_GROUP
        .iter()
        .filter_map(|group| {
            let roles = group
                .roles
                .iter()
                .filter(|role| matches_query(role.name(), &query))
                .collect::<Vec<_>>();
            if roles.is_empty() {
                return None;
            }

            let swatch_colors_rows = roles.into_iter().map(|role| {
                let (_, editor) = editors
                    .iter()
                    .find(|(editor_role, _)| editor_role == role)
                    .expect("registered color editor");

                variable_swatch(
                    palette.variable(*role),
                    editor.clone(),
                    on_toggle_lock.clone(),
                    cx,
                )
                .into_any_element()
            });

            Some(
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
                    .into_any_element(),
            )
        })
        .collect::<Vec<_>>();

    let list = if sections.is_empty() {
        empty_state(&query, on_clear_search, cx).into_any_element()
    } else {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .pb_5()
            .children(sections)
            .into_any_element()
    };

    div()
        .w(px(300.))
        .min_w(px(300.))
        .max_w(px(300.))
        .h_full()
        .flex_shrink_0()
        .px_3()
        .flex()
        .flex_col()
        .gap_4()
        .child(search)
        .child(
            div()
                .id("color-sidebar")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .child(list),
        )
}

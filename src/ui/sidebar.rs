use gpui_kit::{component::ActiveTheme, *};

pub fn color_sidebar(
    rows: impl IntoIterator<Item = AnyElement>,
    footer: impl IntoElement,
    cx: &App,
) -> impl IntoElement {
    div()
        .id("color-sidebar")
        .w(px(300.))
        .min_w(px(300.))
        .max_w(px(300.))
        .h_full()
        .flex_shrink_0()
        .overflow_y_scroll()
        .p_4()
        .bg(cx.theme().sidebar)
        .text_color(cx.theme().sidebar_foreground)
        .border_r_1()
        .border_color(cx.theme().sidebar_border)
        .flex()
        .flex_col()
        .gap_5()
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .child("Paluvio"),
        )
        .child(
            div()
                .rounded_lg()
                .border_1()
                .border_color(cx.theme().sidebar_border)
                .p_4()
                .flex()
                .flex_col()
                .gap_5()
                .child(div().font_weight(FontWeight::SEMIBOLD).child("Colors"))
                .children(rows),
        )
        .child(footer)
}

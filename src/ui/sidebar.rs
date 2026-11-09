use gpui_kit::*;

pub fn color_sidebar(
    rows: impl IntoIterator<Item = AnyElement>,
    footer: impl IntoElement,
    _: &App,
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
        .border_r_1()
        .flex()
        .flex_col()
        .gap_5()
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .child("Paluvio"),
        )
        .child(div().flex().flex_col().gap_1().children(rows))
        .child(footer)
}

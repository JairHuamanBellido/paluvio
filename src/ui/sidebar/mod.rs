use gpui_kit::*;

pub fn color_sidebar(sections: impl IntoIterator<Item = AnyElement>, _: &App) -> impl IntoElement {
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
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .child("Paluvio"),
        )
        .child(div().flex().flex_col().gap_4().children(sections))
}

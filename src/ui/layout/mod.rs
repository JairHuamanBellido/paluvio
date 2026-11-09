use gpui_kit::{component::ActiveTheme, *};

pub fn app_layout(
    sidebar: impl IntoElement,
    content: impl IntoElement,
    header: impl IntoElement,
    cx: &App,
) -> impl IntoElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(rgb(0xffffff))
        .text_color(cx.theme().foreground)
        .font_family("Manrope")
        .font_weight(FontWeight::NORMAL)
        .child(header)
        .child(
            div()
                .flex()
                .flex_1()
                .overflow_hidden()
                .child(sidebar)
                .child(content),
        )
}

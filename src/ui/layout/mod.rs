use gpui_kit::{component::ActiveTheme, *};

pub fn app_layout(
    sidebar: impl IntoElement,
    content: impl IntoElement,
    header: impl IntoElement,
    cx: &App,
) -> impl IntoElement {
    let is_dark_mode = cx.theme().is_dark();
    let background_color = if is_dark_mode {
        rgb(0x000000)
    } else {
        rgb(0xffffff)
    };
    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(background_color)
        .text_color(cx.theme().foreground)
        .font_family("Manrope")
        .font_weight(FontWeight::NORMAL)
        .child(header)
        .child(
            div()
                .flex()
                .flex_1()
                .min_h_0()
                .min_w_0()
                .overflow_hidden()
                .child(sidebar)
                .child(content),
        )
}

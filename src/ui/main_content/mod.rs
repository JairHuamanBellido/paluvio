use gpui_kit::{component::ActiveTheme, *};
pub fn main_content(_: &mut Window, cx: &App) -> impl IntoElement {
    div()
        .id("palette-preview")
        .flex_1()
        .min_w_0()
        .min_h_0()
        .overflow_y_scroll()
        .bg(cx.theme().background)
        .text_color(cx.theme().foreground)
        .mx_10()
        .rounded(px(24.))
        .p_8()
        .child(
            div().flex().flex_col().gap_6().child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .child("Welcome to Main Content!!"),
            ),
        )
}

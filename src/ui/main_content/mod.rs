mod containers;
mod showcase;

use gpui_kit::{component::ActiveTheme, *};

pub use containers::{Containers, ContentTab};

/// Main area: scrolls and frames whichever container the active tab shows.
pub fn main_content(container: AnyView, cx: &App) -> impl IntoElement {
    div()
        .id("main-content")
        .flex_1()
        .min_w_0()
        .min_h_0()
        .overflow_y_scroll()
        .bg(cx.theme().background)
        .text_color(cx.theme().foreground)
        .rounded_tl(px(24.))
        .rounded_bl(px(24.))
        .p_8()
        .child(container)
}

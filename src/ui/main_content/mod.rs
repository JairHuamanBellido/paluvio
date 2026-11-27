mod containers;
mod showcase;

use gpui_kit::{component::ActiveTheme, prelude::FluentBuilder, *};

pub use containers::{Containers, ContentTab};

/// Corner radius of the main area's left corners.
pub const MAIN_RADIUS: Pixels = px(24.);

/// Main area: scrolls and frames whichever container the active tab shows.
/// `padded` adds space around the container; see [`ContentTab::padded`].
pub fn main_content(container: AnyView, padded: bool, cx: &App) -> impl IntoElement {
    div()
        .id("main-content")
        .flex_1()
        .min_w_0()
        .min_h_0()
        .overflow_y_scroll()
        .bg(cx.theme().background)
        .text_color(cx.theme().foreground)
        .rounded_l(MAIN_RADIUS)
        .when(padded, |this| this.p_8())
        .child(container)
}

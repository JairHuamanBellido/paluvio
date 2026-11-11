//! Shared shell for every showcase card: surface background, radius, padding.
//!
//! `Theme` has no `surface` color; the library defines its surface token as
//! `popover` (see `Theme::color_tokens`), so the card uses that.

use gpui_kit::{component::ActiveTheme, *};

/// Corner radius of every card. Also used by sections that must match it
/// (overflow clipping is rectangular).
pub const CARD_RADIUS: Pixels = px(24.);

pub fn card(cx: &App) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .w_full()
        .p_6()
        .rounded(CARD_RADIUS)
        .bg(cx.theme().popover)
        .text_color(cx.theme().popover_foreground)
}

/// Standard card heading: bold title over a muted one-line subtitle.
///
/// Returns a `Div` so a card can adjust it (e.g. its own padding).
pub fn card_header(title: &'static str, subtitle: &'static str, cx: &App) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .child(title),
        )
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(subtitle),
        )
}

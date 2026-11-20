//! Sidebar search: the query input, name matching, and the no-results state.

use gpui_kit::{
    component::{
        ActiveTheme, Sizable,
        button::{Button, ButtonVariants},
        input::{Input, InputState},
    },
    *,
};

/// Case-insensitive partial match of a colour variable name against `query`.
///
/// A blank query matches everything, so clearing it restores the full list.
pub fn matches_query(name: &str, query: &str) -> bool {
    let query = query.trim();
    query.is_empty() || name.to_lowercase().contains(&query.to_lowercase())
}

/// Search input styled from the live palette (`input` surface, `foreground` text),
/// so it follows theme and palette changes.
pub fn search_bar(state: &Entity<InputState>, cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    let has_query = !state.read(cx).value().is_empty();

    let input = Input::new(state)
        .aria_label("Search a color")
        .bordered(false)
        .bg(theme.input)
        .text_color(theme.foreground)
        .rounded(px(12.))
        .h(px(40.))
        .px(px(12.))
        .cleanable(true);

    // The clear button takes this slot while there is a query.
    if has_query {
        input
    } else {
        input.suffix(icon!("magnifying-glass-icon.svg").text_color(theme.muted_foreground))
    }
}

/// Shown in place of the list when no colour variable matches the query.
pub fn empty_state(
    query: &str,
    on_clear: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    cx: &App,
) -> impl IntoElement {
    let theme = cx.theme();

    div()
        .pt_10()
        .px_4()
        .flex()
        .flex_col()
        .items_center()
        .gap_3()
        .text_center()
        .child(
            div()
                .size(px(48.))
                .rounded_full()
                .bg(theme.input)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    icon!("magnifying-glass-icon.svg")
                        .size(px(20.))
                        .text_color(theme.muted_foreground),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.foreground)
                        .child("No colors found"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(format!(
                            "Nothing matches \u{201c}{}\u{201d}. Try another name.",
                            query.trim()
                        )),
                ),
        )
        .child(
            Button::new("clear-color-search")
                .label("Clear search")
                .small()
                .ghost()
                .on_click(on_clear),
        )
}

#[cfg(test)]
mod tests {
    use super::matches_query;

    #[test]
    fn blank_query_matches_everything() {
        assert!(matches_query("Primary", ""));
        assert!(matches_query("Primary", "   "));
    }

    #[test]
    fn matches_partial_name_ignoring_case() {
        assert!(matches_query("Primary Foreground", "FORE"));
        assert!(matches_query("Primary Foreground", "ary fo"));
        assert!(matches_query("Button Hover", "  hover "));
    }

    #[test]
    fn rejects_non_matching_names() {
        assert!(!matches_query("Primary", "accent"));
    }
}

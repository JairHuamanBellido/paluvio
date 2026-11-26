use gpui_kit::{
    component::{
        Sizable,
        tab::{Tab, TabBar},
    },
    *,
};

use crate::ui::main_content::ContentTab;

/// Segmented tab bar listing every [`ContentTab`]. Tabs come from
/// [`ContentTab::ALL`], so a new tab needs no change here.
pub fn content_tabs(
    active: ContentTab,
    on_select: impl Fn(ContentTab, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    TabBar::new("content-tabs")
        .segmented()
        .small()
        .selected_index(active.index())
        .children(
            ContentTab::ALL
                .iter()
                .map(|tab| Tab::new().label(tab.label())),
        )
        .on_click(move |index, window, cx| {
            if let Some(tab) = ContentTab::ALL.get(*index) {
                on_select(*tab, window, cx);
            }
        })
}

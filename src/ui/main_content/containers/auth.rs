//! Auth container. Placeholder until the auth screens land.

use gpui_kit::{component::ActiveTheme, *};

pub struct AuthContainer;

impl Render for AuthContainer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(cx.theme().muted_foreground)
            .child("Auth container")
    }
}

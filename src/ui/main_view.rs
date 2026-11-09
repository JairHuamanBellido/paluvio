use gpui_kit::{
    component::{Theme, ThemeMode},
    *,
};

use super::{
    color_editor::{ColorEditor, ColorEditorEvent},
    sidebar::app_sidebar,
};
use crate::{
    theme::{self, ColorRole, PaletteState},
    ui::{header::app_header, layout::app_layout, main_content::main_content},
};

pub struct MainView {
    palette: PaletteState,
    editors: Vec<(ColorRole, Entity<ColorEditor>)>,
    _subscriptions: Vec<Subscription>,
}

impl MainView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let palette = PaletteState::new();

        Theme::change(ThemeMode::Light, Some(window), cx);
        theme::apply(&palette, cx);

        let mut editors: Vec<(ColorRole, Entity<ColorEditor>)> = Vec::new();
        let mut subscriptions = Vec::new();

        for role in ColorRole::ALL {
            let editor =
                cx.new(|cx| ColorEditor::new(role, palette.variable(role).value, window, cx));
            subscriptions.push(cx.subscribe_in(
                &editor,
                window,
                move |this, _, event, window, cx| {
                    match event {
                        ColorEditorEvent::Change(value) => this.palette.set(role, *value),
                        ColorEditorEvent::Reset => this.palette.reset(role),
                    }
                    // Programmatic synchronization does not emit editor changes.
                    for (role, editor) in &this.editors {
                        let value = this.palette.variable(*role).value;
                        editor.update(cx, |editor, cx| editor.set_value(value, window, cx));
                    }
                    theme::apply(&this.palette, cx);
                    cx.notify();
                },
            ));
            editors.push((role, editor));
        }
        Self {
            palette,
            editors,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for MainView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        app_layout(
            app_sidebar(&self.palette, &self.editors, cx),
            main_content(window, cx),
            app_header(),
            cx,
        )
    }
}

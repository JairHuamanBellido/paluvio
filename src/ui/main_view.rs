use gpui_kit::{
    component::{ActiveTheme, Theme, ThemeMode, button::Button},
    *,
};

use super::{
    color_editor::{ColorEditor, ColorEditorEvent},
    sidebar::color_sidebar,
    variable_swatch::variable_swatch,
};
use crate::theme::{self, ColorRole, PaletteState};

pub struct MainView {
    palette: PaletteState,
    editors: Vec<(ColorRole, Entity<ColorEditor>)>,
    copied: bool,
    _subscriptions: Vec<Subscription>,
}

impl MainView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let palette = PaletteState::new();
        Theme::change(ThemeMode::Light, Some(window), cx);
        theme::apply(&palette, cx);
        let mut editors = Vec::new();
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
                    this.copied = false;
                    theme::apply(&this.palette, cx);
                    cx.notify();
                },
            ));
            editors.push((role, editor));
        }
        Self {
            palette,
            editors,
            copied: false,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self
            .editors
            .iter()
            .map(|(role, editor)| {
                variable_swatch(self.palette.variable(*role), editor.clone(), cx).into_any_element()
            })
            .collect::<Vec<_>>();
        let export = Button::new("copy-palette")
            .label(if self.copied {
                "Palette copied"
            } else {
                "Copy palette"
            })
            .on_click(cx.listener(|this, _, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(this.palette.export()));
                this.copied = true;
                cx.notify();
            }));
        let primary = self.palette.variable(ColorRole::Primary).value.to_color();
        div()
            .font_family("Manrope")
            .font_weight(FontWeight::NORMAL)
            .size_full()
            .flex()
            .overflow_hidden()
            .child(color_sidebar(rows, export, cx))
            .child(
                div()
                    .id("palette-preview")
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .overflow_y_scroll()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .p_8()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_6()
                            .child(div().text_sm().child("Palette preview"))
                            .child(
                                div()
                                    .text_3xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child("Make it yours."),
                            )
                            .child("Select a color in the sidebar to update your palette.")
                            .child(div().h(px(8.)).w(px(80.)).rounded_full().bg(primary))
                            .child(
                                div()
                                    .text_lg()
                                    .child("Background and foreground update live."),
                            ),
                    ),
            )
    }
}

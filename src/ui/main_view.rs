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
    ui::{
        header::Header,
        layout::app_layout,
        main_content::{Showcase, main_content},
        toolbar::{ThemeModeSelector, ThemePreference, toolbar},
    },
};

pub struct MainView {
    palette: PaletteState,
    editors: Vec<(ColorRole, Entity<ColorEditor>)>,
    _subscriptions: Vec<Subscription>,
    theme_mode: Entity<ThemeModeSelector>,
    showcase: Entity<Showcase>,
}

impl MainView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Theme::sync_system_appearance(Some(window), cx);
        let palette = PaletteState::new(Theme::global(cx).mode);
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

                    this.refresh(window, cx);
                },
            ));
            editors.push((role, editor));
        }

        let theme_mode: Entity<ThemeModeSelector> =
            cx.new(|cx: &mut Context<'_, ThemeModeSelector>| {
                ThemeModeSelector::new(super::toolbar::ThemePreference::System, cx)
            });

        subscriptions.push(cx.observe_window_appearance(window, |this, window, cx| {
            if this.theme_mode.read(cx).preference() == ThemePreference::System {
                Theme::sync_system_appearance(Some(window), cx);
                this.sync_palette_mode(window, cx);
            }
        }));

        subscriptions.push(
            cx.subscribe_in(&theme_mode, window, |this, _, event, window, cx| {
                match event.0 {
                    ThemePreference::System => {
                        Theme::sync_system_appearance(Some(window), cx);
                    }
                    ThemePreference::Light => {
                        Theme::change(ThemeMode::Light, Some(window), cx);
                    }
                    ThemePreference::Dark => {
                        Theme::change(ThemeMode::Dark, Some(window), cx);
                    }
                }

                this.sync_palette_mode(window, cx);
            }),
        );

        let showcase = cx.new(|cx| Showcase::new(window, cx));

        Self {
            palette,
            showcase,
            editors,
            _subscriptions: subscriptions,
            theme_mode,
        }
    }

    /// Pushes the palette to the UI: editors, live theme, and a re-render.
    ///
    /// Editors must be updated by hand, because programmatic `set_value` calls
    /// do not emit change events.
    fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for (role, editor) in &self.editors {
            let value = self.palette.variable(*role).value;
            editor.update(cx, |editor, cx| editor.set_value(value, window, cx));
        }

        theme::apply(&self.palette, cx);
        cx.notify();
    }

    /// Copies the resolved theme mode into the palette, then refreshes the UI.
    ///
    /// Call it right after every `Theme::change` or `Theme::sync_system_appearance`.
    /// `Theme::global(cx).mode` is already System-resolved, so this works for
    /// the Light, Dark and System preferences alike.
    fn sync_palette_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.palette.set_mode(Theme::global(cx).mode);
        self.refresh(window, cx);
    }
}

impl Render for MainView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = Header::new(toolbar(&self.theme_mode));
        app_layout(
            app_sidebar(&self.palette, &self.editors, cx),
            main_content(&self.showcase, cx),
            header,
            cx,
        )
    }
}

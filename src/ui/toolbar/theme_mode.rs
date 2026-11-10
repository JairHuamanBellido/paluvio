use gpui_kit::{
    base::Side,
    component::{ActiveTheme, Icon, button::Button, menu::DropdownMenu},
    *,
};

actions!(theme_mode, [SelectLight, SelectDark, SelectSystem]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ThemePreference {
    Light,
    Dark,
    System,
}

pub struct ThemeModeSelector {
    preference: ThemePreference,
    focus_handle: FocusHandle,
}

pub struct ThemeModeChanged(pub ThemePreference);

impl EventEmitter<ThemeModeChanged> for ThemeModeSelector {}

impl ThemeModeSelector {
    pub fn new(preference: ThemePreference, cx: &mut Context<Self>) -> Self {
        Self {
            preference,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn select(&mut self, preference: ThemePreference, cx: &mut Context<Self>) {
        if self.preference == preference {
            return;
        }

        self.preference = preference;

        cx.emit(ThemeModeChanged(preference));
        cx.notify();
    }

    pub fn preference(&self) -> ThemePreference {
        self.preference
    }
}

fn theme_icon(preference: ThemePreference) -> Icon {
    let bytes: &[u8] = match preference {
        ThemePreference::Light => {
            include_bytes!("../../../assets/light-mode-icon.svg")
        }
        ThemePreference::Dark => {
            include_bytes!("../../../assets/dark-mode-icon.svg")
        }
        ThemePreference::System => {
            include_bytes!("../../../assets/system-theme-mode-icon.svg")
        }
    };

    Icon::default().data(bytes)
}

fn theme_option(title: &'static str, description: &'static str, icon: Icon, cx: &App) -> Div {
    div()
        .flex()
        .flex_1()
        .items_center()
        .gap_3()
        .py(px(6.))
        .child(icon)
        .child(
            div()
                .flex()
                .flex_col()
                .child(div().text_sm().child(title))
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child(description),
                ),
        )
}

impl Render for ThemeModeSelector {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();

        let preference = self.preference;

        let trigger_mode = if cx.theme().is_dark() {
            ThemePreference::Dark
        } else {
            ThemePreference::Light
        };

        div()
            .track_focus(&focus_handle)
            .on_action(cx.listener(|this, _: &SelectLight, _, cx| {
                this.select(ThemePreference::Light, cx);
            }))
            .on_action(cx.listener(|this, _: &SelectDark, _, cx| {
                this.select(ThemePreference::Dark, cx);
            }))
            .on_action(cx.listener(|this, _: &SelectSystem, _, cx| {
                this.select(ThemePreference::System, cx);
            }))
            .child(
                Button::new("theme-mode-trigger")
                    .icon(theme_icon(trigger_mode))
                    .accessibility_label("Choose theme mode")
                    .tooltip("Choose theme mode")
                    .border_0()
                    .dropdown_caret(false)
                    .dropdown_menu(move |menu, _, _| {
                        menu.action_context(focus_handle.clone())
                            .min_w(px(224.))
                            .check_side(Side::Right)
                            .menu_element_with_check(
                                preference == ThemePreference::System,
                                Box::new(SelectSystem),
                                |_, cx| {
                                    theme_option(
                                        "System",
                                        "Follow device setting",
                                        theme_icon(ThemePreference::System)
                                            .text_color(cx.theme().muted_foreground),
                                        cx,
                                    )
                                },
                            )
                            .menu_element_with_check(
                                preference == ThemePreference::Light,
                                Box::new(SelectLight),
                                |_, cx| {
                                    theme_option(
                                        "Light",
                                        "Use light theme",
                                        theme_icon(ThemePreference::Light)
                                            .text_color(cx.theme().muted_foreground),
                                        cx,
                                    )
                                },
                            )
                            .menu_element_with_check(
                                preference == ThemePreference::Dark,
                                Box::new(SelectDark),
                                |_, cx| {
                                    theme_option(
                                        "Dark",
                                        "Use dark theme",
                                        theme_icon(ThemePreference::Dark)
                                            .text_color(cx.theme().muted_foreground),
                                        cx,
                                    )
                                },
                            )
                    }),
            )
    }
}

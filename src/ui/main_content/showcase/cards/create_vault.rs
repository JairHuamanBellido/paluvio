use gpui_kit::{
    component::{
        ActiveTheme, Sizable,
        button::{Button, ButtonVariants},
        input::{Input, InputState},
        separator::Separator,
    },
    *,
};

use crate::ui::main_content::showcase::card::{CARD_RADIUS, card, card_header};

/// One passphrase requirement: chip label and the check it must pass.
struct PassRule {
    label: &'static str,
    check: fn(&str) -> bool,
}

const RULES: [PassRule; 4] = [
    PassRule {
        label: "10+ characters",
        check: |password| password.chars().count() >= 10,
    },
    PassRule {
        label: "Digit",
        check: |password| password.chars().any(|c| c.is_ascii_digit()),
    },
    PassRule {
        label: "Capital letter",
        check: |password| password.chars().any(char::is_uppercase),
    },
    PassRule {
        label: "Small letter",
        check: |password| password.chars().any(char::is_lowercase),
    },
];

const LEVEL_SEGMENTS: usize = 3;

pub struct CreateVault {
    vault_name: Entity<InputState>,
    password: Entity<InputState>,
    _password_changed: Subscription,
}

impl CreateVault {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let vault_name = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Vault name")
                .default_value("")
        });
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Passphrase")
                .masked(true)
                .default_value("")
        });
        // Re-render on every keystroke so rules and level update live.
        let password_changed = cx.observe(&password, |_, _, cx| cx.notify());
        Self {
            vault_name,
            password,
            _password_changed: password_changed,
        }
    }
}

/// Filled segments and their color for the number of rules met:
/// red (weak), yellow (fair), green (strong).
fn security_level(met: usize, empty: bool, cx: &App) -> (usize, Hsla) {
    let theme = cx.theme();
    if empty {
        return (0, theme.red);
    }
    match met {
        0 | 1 => (1, theme.red),
        2 | 3 => (2, theme.yellow),
        _ => (3, theme.green),
    }
}

fn label(text: &'static str) -> Div {
    div().text_sm().font_weight(FontWeight::MEDIUM).child(text)
}

fn rule_chip(text: &'static str, met: bool, cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    let (icon, color) = if met {
        (icon!("check-circle-icon.svg"), theme.green)
    } else {
        (icon!("x-circle-icon.svg"), theme.muted_foreground)
    };

    div()
        .flex()
        .items_center()
        .gap_1p5()
        .pl_1p5()
        .pr_2p5()
        .py_1()
        .rounded_full()
        .border_1()
        .border_color(theme.border)
        .text_xs()
        .text_color(color)
        .child(icon.small().text_color(color))
        .child(text)
}

impl Render for CreateVault {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let password = self.password.read(cx).value();
        let met = RULES.map(|rule| (rule.check)(&password));
        let (filled, level_color) = security_level(
            met.iter().filter(|met| **met).count(),
            password.is_empty(),
            cx,
        );
        let theme = cx.theme();
        let muted = theme.muted_foreground;

        // Sections own their padding so the divider and footer run edge to edge.
        card(cx)
            .p_0()
            .gap_0()
            .overflow_hidden()
            .child(
                card_header(
                    "Create a vault",
                    "Protect your notes with a master passphrase",
                    cx,
                )
                .p_6()
                .pb_8(),
            )
            .child(Separator::horizontal())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_5()
                    .p_6()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(label("Vault name").text_color(muted))
                            .child(Input::new(&self.vault_name).bg(transparent_black())),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(label("Master passphrase").flex_1().text_color(muted))
                                    .child(div().text_xs().text_color(muted).child("Strength"))
                                    .child(div().flex().gap_0p5().children(
                                        (0..LEVEL_SEGMENTS).map(|segment| {
                                            div().w(px(14.)).h(px(4.)).rounded_full().bg(
                                                if segment < filled {
                                                    level_color
                                                } else {
                                                    theme.muted
                                                },
                                            )
                                        }),
                                    )),
                            )
                            .child(
                                Input::new(&self.password)
                                    .bg(transparent_black())
                                    .prefix(icon!("lock-icon.svg").text_color(muted))
                                    .mask_toggle()
                                    .cleanable(true),
                            ),
                    )
                    .child(
                        div().flex().flex_wrap().gap_2().children(
                            RULES
                                .iter()
                                .zip(met)
                                .map(|(rule, met)| rule_chip(rule.label, met, cx)),
                        ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_6()
                    .py_4()
                    // Overflow clip is rectangular; match the card radius ourselves.
                    .rounded_b(CARD_RADIUS)
                    .bg(theme.muted.opacity(0.5))
                    .child(
                        Button::new("create-vault-cancel")
                            .secondary()
                            .label("Discard"),
                    )
                    .child(Button::new("create-vault-next").primary().label("Continue")),
            )
    }
}

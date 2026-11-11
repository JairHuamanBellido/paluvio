use gpui_kit::{
    component::{ActiveTheme, Icon, Sizable, Theme},
    prelude::FluentBuilder,
    *,
};

use crate::ui::main_content::showcase::card::{card, card_header};

/// The four theme variables of one status color.
#[derive(Clone, Copy)]
struct StatusColors {
    base: Hsla,
    foreground: Hsla,
    hover: Hsla,
    active: Hsla,
}

impl StatusColors {
    fn danger(theme: &Theme) -> Self {
        Self {
            base: theme.danger,
            foreground: theme.danger_foreground,
            hover: theme.danger_hover,
            active: theme.danger_active,
        }
    }

    fn warning(theme: &Theme) -> Self {
        Self {
            base: theme.warning,
            foreground: theme.warning_foreground,
            hover: theme.warning_hover,
            active: theme.warning_active,
        }
    }

    fn success(theme: &Theme) -> Self {
        Self {
            base: theme.success,
            foreground: theme.success_foreground,
            hover: theme.success_hover,
            active: theme.success_active,
        }
    }

    fn info(theme: &Theme) -> Self {
        Self {
            base: theme.info,
            foreground: theme.info_foreground,
            hover: theme.info_hover,
            active: theme.info_active,
        }
    }
}

/// Status chip: tinted base background, base border, foreground text.
/// Hover and press swap the background for the theme's hover and active shades.
///
/// Not a `Button`: its custom variant blends the color 80% toward transparent,
/// so it cannot draw these exact shades.
fn chip(
    id: &'static str,
    label: &'static str,
    icon: Icon,
    colors: StatusColors,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_none()
        .items_center()
        .gap_1p5()
        .pl_2()
        .pr_3()
        .py_1()
        .rounded_full()
        .bg(colors.base)
        .text_color(colors.foreground)
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .hover(move |style| style.bg(colors.hover))
        .active(move |style| style.bg(colors.active))
        .child(icon.small().text_color(colors.foreground))
        .child(label)
}

/// One row of the payments list.
struct Payment {
    title: &'static str,
    detail: &'static str,
    chip_id: &'static str,
    status: &'static str,
    icon: fn() -> Icon,
    colors: fn(&Theme) -> StatusColors,
}

const PAYMENTS: [Payment; 4] = [
    Payment {
        title: "Invoice #1042",
        detail: "$1,250.00 · Acme Inc.",
        chip_id: "payment-completed",
        status: "Completed",
        icon: || icon!("check-circle-icon.svg"),
        colors: StatusColors::success,
    },
    Payment {
        title: "Payout to bank",
        detail: "$3,480.00 · Arrives in 2 days",
        chip_id: "payment-processing",
        status: "Processing",
        icon: || icon!("info-icon.svg"),
        colors: StatusColors::info,
    },
    Payment {
        title: "Subscription renewal",
        detail: "$29.00 · Awaiting approval",
        chip_id: "payment-pending",
        status: "Pending",
        icon: || icon!("warning-icon.svg"),
        colors: StatusColors::warning,
    },
    Payment {
        title: "Card payment",
        detail: "$412.60 · Card declined",
        chip_id: "payment-failed",
        status: "Failed",
        icon: || icon!("x-circle-icon.svg"),
        colors: StatusColors::danger,
    },
];

/// One payment: description on the left, status chip on the right.
fn payment_row(
    title: &'static str,
    detail: &'static str,
    chip: impl IntoElement,
    first: bool,
    cx: &App,
) -> impl IntoElement {
    let theme = cx.theme();

    div()
        .flex()
        .items_center()
        .gap_3()
        .py_3()
        .when(!first, |this| this.border_t_1().border_color(theme.border))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(title))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(detail),
                ),
        )
        .child(chip)
}

pub struct StatusChips;

impl Render for StatusChips {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cx: &App = cx;
        let theme = cx.theme();

        card(cx)
            .gap_2()
            .child(card_header(
                "Payments",
                "Latest activity on your account",
                cx,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(PAYMENTS.iter().enumerate().map(|(index, payment)| {
                        payment_row(
                            payment.title,
                            payment.detail,
                            chip(
                                payment.chip_id,
                                payment.status,
                                (payment.icon)(),
                                (payment.colors)(theme),
                            ),
                            index == 0,
                            cx,
                        )
                    })),
            )
    }
}

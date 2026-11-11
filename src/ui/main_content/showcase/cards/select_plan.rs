use gpui_kit::{
    component::{
        ActiveTheme, Sizable,
        button::{Button, ButtonVariants},
        radio::Radio,
        separator::Separator,
    },
    prelude::FluentBuilder,
    *,
};

use crate::ui::main_content::showcase::card::{card, card_header};

struct Plan {
    name: &'static str,
    audience: &'static str,
    /// Price per user per month, in cents.
    price: u32,
    min_seats: u32,
    features: [&'static str; 3],
}

const PLANS: [Plan; 3] = [
    Plan {
        name: "Plus",
        audience: "For individuals",
        price: 899,
        min_seats: 1,
        features: [
            "100 GB of space",
            "7 days of file recovery",
            "Email support",
        ],
    },
    Plan {
        name: "Standard",
        audience: "For teams",
        price: 1299,
        min_seats: 3,
        features: [
            "1 TB of space",
            "30 days of file recovery",
            "256-bit AES and SSL/TLS",
        ],
    },
    Plan {
        name: "Advanced",
        audience: "For organizations",
        price: 2499,
        min_seats: 5,
        features: [
            "Unlimited space",
            "1 year of file recovery",
            "SSO and audit logs",
        ],
    },
];

const MAX_SEATS: u32 = 99;

fn format_cents(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

/// Check-marked feature list of the selected plan.
fn features(plan: &Plan, cx: &App) -> impl IntoElement {
    let primary = cx.theme().primary;
    div()
        .flex()
        .flex_col()
        .gap_2()
        .children(plan.features.iter().map(|feature| {
            div()
                .flex()
                .items_center()
                .gap_3()
                .text_sm()
                .child(icon!("check-icon.svg").text_color(primary))
                .child(*feature)
        }))
}

pub struct SelectPlan {
    selected: usize,
    seats: u32,
}

impl SelectPlan {
    pub fn new() -> Self {
        Self {
            selected: 1,
            seats: PLANS[1].min_seats,
        }
    }

    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.selected != index {
            self.selected = index;
            self.seats = PLANS[index].min_seats;
            cx.notify();
        }
    }

    fn set_seats(&mut self, seats: u32, cx: &mut Context<Self>) {
        self.seats = seats.clamp(PLANS[self.selected].min_seats, MAX_SEATS);
        cx.notify();
    }

    fn plan_row(&self, index: usize, cx: &mut Context<Self>) -> AnyElement {
        let plan = &PLANS[index];
        let selected = self.selected == index;
        let theme = cx.theme();
        let muted = theme.muted_foreground;
        let border = if selected {
            theme.primary
        } else {
            theme.border
        };

        div()
            .id(("plan", index))
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded(px(16.))
            .border_1()
            .border_color(border)
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| this.select(index, cx)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        Radio::new(("plan-radio", index))
                            .checked(selected)
                            .on_click(cx.listener(move |this, _, _, cx| this.select(index, cx))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(plan.name),
                            )
                            .child(div().text_xs().text_color(muted).child(plan.audience)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_end()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(format_cents(plan.price)),
                            )
                            .child(div().text_xs().text_color(muted).child("user / month")),
                    ),
            )
            .when(selected, |this| {
                this.child(features(plan, cx))
                    .child(Separator::horizontal())
                    .child(self.seats_row(plan, cx))
            })
            .into_any_element()
    }

    fn seats_row(&self, plan: &Plan, cx: &mut Context<Self>) -> AnyElement {
        let seats = self.seats;
        let theme = cx.theme();
        let muted = theme.muted_foreground;

        div()
            .flex()
            .items_center()
            .gap_3()
            .child(icon!("users-icon.svg").text_color(muted))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(div().text_sm().child("Users"))
                    .child(
                        div()
                            .text_xs()
                            .text_color(muted)
                            .child(format!("Starting at {} users", plan.min_seats)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .p_1()
                    .rounded(px(10.))
                    .border_1()
                    .border_color(theme.border)
                    .child(
                        Button::new("seats-minus")
                            .ghost()
                            .xsmall()
                            .icon(icon!("minus-icon.svg"))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.set_seats(seats.saturating_sub(1), cx)
                            })),
                    )
                    .child(
                        div()
                            .w(px(28.))
                            .text_center()
                            .text_sm()
                            .child(seats.to_string()),
                    )
                    .child(
                        Button::new("seats-plus")
                            .ghost()
                            .xsmall()
                            .icon(icon!("plus-icon.svg"))
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.set_seats(seats + 1, cx)),
                            ),
                    ),
            )
            .into_any_element()
    }
}

impl Render for SelectPlan {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let total = PLANS[self.selected].price * self.seats;
        let muted = cx.theme().muted_foreground;
        let plans: Vec<AnyElement> = (0..PLANS.len())
            .map(|index| self.plan_row(index, cx))
            .collect();

        card(cx)
            .gap_5()
            .child(card_header(
                "Select a plan",
                "Billed monthly, cancel anytime",
                cx,
            ))
            .child(div().flex().flex_col().gap_2().children(plans))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .child(div().text_xs().text_color(muted).child("Total per month"))
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(format_cents(total)),
                            ),
                    )
                    .child(
                        Button::new("select-plan-continue")
                            .primary()
                            .label("Continue"),
                    ),
            )
    }
}

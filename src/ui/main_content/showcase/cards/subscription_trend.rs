use gpui_kit::{
    component::{
        ActiveTheme, Sizable,
        button::{Button, ButtonVariants},
        chart::AreaChart,
        separator::Separator,
    },
    *,
};

use crate::ui::main_content::showcase::{card::card, noise};

/// Days charted: 2 Sep to 2 Oct.
const DAYS: usize = 31;
/// Days from 2 Sep to 30 Sep, inclusive.
const SEPTEMBER_DAYS: usize = 29;

/// `Clone` is cheap: `SharedString` is reference-counted.
#[derive(Clone)]
struct Day {
    label: SharedString,
    current: f64,
    previous: f64,
}

/// Deterministic noise in `0.0..1.0`, so the lines are stable across renders.
fn unit_noise(seed: u32, index: usize) -> f64 {
    f64::from(noise::hash(seed, index) % 1000) / 1000.
}

/// MRR in dollars at the end of each day, for this period and the one before it.
fn days() -> Vec<Day> {
    (0..DAYS)
        .map(|index| {
            let label = if index < SEPTEMBER_DAYS {
                format!("{} Sep", index + 2).into()
            } else {
                format!("{} Oct", index - SEPTEMBER_DAYS + 1).into()
            };
            let wave = (index as f64 / 6.).sin();
            Day {
                label,
                current: (41_000.
                    + index as f64 * 220.
                    + wave * 500.
                    + unit_noise(7, index) * 400.)
                    .round(),
                previous: (37_000.
                    + index as f64 * 140.
                    + wave * 350.
                    + unit_noise(23, index) * 600.)
                    .round(),
            }
        })
        .collect()
}

/// Whole dollars with thousands separators: `$153,200`.
fn format_dollars(value: f64) -> String {
    let digits = (value.abs().round() as i64).to_string();
    let mut grouped = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    format!("${grouped}")
}

fn stat(label: &'static str, value: String, cx: &App) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(label),
        )
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .child(value),
        )
}

pub struct SubscriptionTrend {
    /// Built once: the data is fixed, so rendering only clones it.
    days: Vec<Day>,
}

impl SubscriptionTrend {
    pub fn new() -> Self {
        Self { days: days() }
    }
}

impl Render for SubscriptionTrend {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cx: &App = cx;
        let theme = cx.theme();
        let primary = theme.primary;
        let muted = theme.muted_foreground;
        let current_color = theme.chart_1;
        let previous_color = theme.chart_2;

        // MRR is a level, not a sum: compare where each period ended.
        let current_mrr = self.days.last().map_or(0., |day| day.current);
        let previous_mrr = self.days.last().map_or(0., |day| day.previous);

        let chart = AreaChart::new(self.days.iter().cloned())
            .x(|day: &Day| day.label.clone())
            // Previous period: chart_2 line, no fill.
            .y(|day: &Day| day.previous)
            .stroke(previous_color)
            .fill(transparent_black())
            .name("Previous period")
            // This period: chart_1 line over a fading chart_1 fill.
            .y(|day: &Day| day.current)
            .stroke(current_color)
            .fill(linear_gradient(
                180.,
                linear_color_stop(current_color.opacity(0.2), 0.),
                linear_color_stop(current_color.opacity(0.), 1.),
            ))
            .name("This period")
            .linear()
            .y_domain(30_000., 50_000.)
            .y_axis(true)
            .y_tick_count(3)
            .y_tick_format(|value| format!("${:.0}k", value / 1000.))
            .x_tick_count(3)
            .grid_columns(12)
            .tooltip_value(|_, _, value| format_dollars(value).into());

        card(cx)
            .gap_5()
            .child(
                div()
                    .flex()
                    .items_start()
                    .gap_3()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(icon!("chart-bar-icon.svg").text_color(primary))
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Subscriptions"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(muted)
                                    .child("Monthly recurring revenue, 2 Sep to 2 Oct"),
                            ),
                    )
                    .child(
                        Button::new("subscription-trend-more")
                            .ghost()
                            .small()
                            .icon(icon!("dots-three-vertical-icon.svg")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_5()
                    .child(stat("Current MRR", format_dollars(current_mrr), cx))
                    .child(Separator::vertical())
                    .child(stat(
                        "Vs last period",
                        format!(
                            "{}{}",
                            if current_mrr >= previous_mrr {
                                "+"
                            } else {
                                "-"
                            },
                            format_dollars(current_mrr - previous_mrr)
                        ),
                        cx,
                    )),
            )
            .child(div().w_full().h(px(180.)).child(chart))
    }
}

use gpui_kit::{
    component::{
        ActiveTheme, Sizable,
        button::{Button, ButtonVariants},
    },
    *,
};

use crate::ui::main_content::showcase::{card::card, noise};

const ROWS: usize = 4;
const COLUMNS: usize = 24;

/// Opacity of the primary color per activity level, quiet to busy.
const LEVEL_OPACITY: [f32; 4] = [0.1, 0.3, 0.6, 1.];

struct Period {
    month: &'static str,
    branch: &'static str,
    commits: &'static str,
    /// Seed for the activity pattern, so the grid is stable across renders.
    seed: u32,
}

const PERIODS: [Period; 2] = [
    Period {
        month: "August",
        branch: "release/v0.8",
        commits: "1,243 commits",
        seed: 7,
    },
    Period {
        month: "September",
        branch: "release/v0.9",
        commits: "1,891 commits",
        seed: 23,
    },
];

/// Deterministic activity level (index into [`LEVEL_OPACITY`]) for a cell.
/// Most cells are quiet; a few are busy.
fn activity_level(seed: u32, cell: usize) -> usize {
    match noise::hash(seed, cell) % 10 {
        0..=5 => 0,
        6 => 1,
        7 => 2,
        _ => 3,
    }
}

fn stat(
    label: &'static str,
    value: &'static str,
    trend: &'static str,
    cx: &App,
) -> impl IntoElement {
    let theme = cx.theme();
    let primary = theme.primary;

    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child(label),
        )
        .child(
            div()
                .flex()
                .items_end()
                .gap_2()
                .child(
                    div()
                        .text_3xl()
                        .line_height(relative(1.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(value),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_0p5()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(primary)
                        .child(icon!("arrow-up-icon.svg").xsmall().text_color(primary))
                        .child(trend),
                ),
        )
}

fn period(period: &Period, cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    let primary = theme.primary;
    let muted = theme.muted_foreground;

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_xs()
                .child(div().font_weight(FontWeight::SEMIBOLD).child(period.month))
                .child(div().text_color(muted).child("/"))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_color(muted)
                        .child(period.branch),
                )
                .child(div().font_weight(FontWeight::MEDIUM).child(period.commits)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .children((0..ROWS).map(|row| {
                    div()
                        .flex()
                        .gap_1()
                        .children((0..COLUMNS).map(move |column| {
                            let level = activity_level(period.seed, row * COLUMNS + column);
                            div()
                                .flex_1()
                                .h(px(10.))
                                .rounded(px(2.))
                                .bg(primary.opacity(LEVEL_OPACITY[level]))
                        }))
                })),
        )
}

pub struct CommitActivity;

impl Render for CommitActivity {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cx: &App = cx;

        card(cx)
            .gap_5()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Commit activity"),
                    )
                    .child(
                        Button::new("commit-activity-export")
                            .ghost()
                            .small()
                            .icon(icon!("export-icon.svg")),
                    )
                    .child(
                        Button::new("commit-activity-more")
                            .ghost()
                            .small()
                            .icon(icon!("dots-three-vertical-icon.svg")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(stat("Average review", "1.8h", "14.6%", cx))
                    .child(stat("Commits merged", "3,134", "11.5%", cx)),
            )
            .children(PERIODS.iter().map(|item| period(item, cx)))
    }
}

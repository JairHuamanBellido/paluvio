use gpui_kit::{
    component::{
        ActiveTheme, Sizable, progress::Progress, separator::Separator, switch::Switch, tag::Tag,
    },
    *,
};

use crate::ui::main_content::showcase::card::card;

const CAPACITY_GB: f32 = 100.;

/// Label, used GB, opacity of the primary color for its legend dot.
const BREAKDOWN: [(&str, f32, f32); 3] = [
    ("Media", 31.1, 1.),
    ("Documents", 28.4, 0.6),
    ("Other", 4.7, 0.3),
];

pub struct StorageUsage {
    auto_upgrade: bool,
}

impl StorageUsage {
    pub fn new() -> Self {
        Self { auto_upgrade: true }
    }
}

fn breakdown_row(label: &'static str, used: f32, dot: Hsla, cx: &App) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_3()
        .text_sm()
        .child(div().size(px(8.)).rounded_full().bg(dot))
        .child(
            div()
                .flex_1()
                .text_color(cx.theme().muted_foreground)
                .child(label),
        )
        .child(
            div()
                .font_weight(FontWeight::MEDIUM)
                .child(format!("{used:.1} GB")),
        )
}

impl Render for StorageUsage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let primary = theme.primary;
        let muted = theme.muted_foreground;
        let used: f32 = BREAKDOWN.iter().map(|(_, used, _)| used).sum();

        card(cx)
            .gap_5()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(px(36.))
                            .rounded(px(10.))
                            .bg(primary.opacity(0.12))
                            .child(icon!("hard-drives-icon.svg").text_color(primary)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Storage"),
                            )
                            .child(div().text_xs().text_color(muted).child("Team workspace")),
                    )
                    .child(Tag::secondary().small().rounded_full().child("PRO")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
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
                                    .child(format!("{used:.1} GB")),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(muted)
                                    .child(format!("of {CAPACITY_GB:.0} GB")),
                            )
                            .child(
                                Tag::primary()
                                    .small()
                                    .rounded_full()
                                    .gap_1()
                                    .child(icon!("trend-up-icon.svg").xsmall())
                                    .child("12%"),
                            ),
                    )
                    .child(
                        Progress::new("storage-usage-progress")
                            .value(used / CAPACITY_GB * 100.)
                            .color(primary),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(BREAKDOWN.iter().map(|(label, used, opacity)| {
                        breakdown_row(label, *used, primary.opacity(*opacity), cx)
                    })),
            )
            .child(Separator::horizontal())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(div().text_sm().child("Auto-upgrade"))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(muted)
                                    .child("Add 50 GB when storage is full"),
                            ),
                    )
                    .child(
                        Switch::new("storage-auto-upgrade")
                            .checked(self.auto_upgrade)
                            .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                this.auto_upgrade = *checked;
                                cx.notify();
                            })),
                    ),
            )
    }
}

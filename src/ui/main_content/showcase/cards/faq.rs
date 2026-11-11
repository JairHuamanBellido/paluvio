use gpui_kit::{
    component::{ActiveTheme, accordion::Accordion},
    *,
};

use crate::ui::main_content::showcase::card::{card, card_header};

/// Question and answer.
const QUESTIONS: [(&str, &str); 4] = [
    (
        "What is Paluvio?",
        "A theme palette generator for GPUI-kit. Tweak a color and watch it land on real \
         components, like the cards around this one.",
    ),
    (
        "Does it work in light and dark mode?",
        "Yes, both. Switch modes from the toolbar whenever you like and see how your palette \
         holds up in each.",
    ),
    (
        "How much does it cost?",
        "Nothing. Paluvio is free, and no credit card is required. Not now, not later.",
    ),
    (
        "What is it built with?",
        "Rust, GPUI-kit and a healthy dose of procrastination. Sorry, tactical pauses.",
    ),
];

pub struct Faq {
    /// The one question currently expanded, if any.
    open: Option<usize>,
}

impl Faq {
    pub fn new() -> Self {
        Self { open: Some(0) }
    }
}

impl Render for Faq {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let muted = theme.muted_foreground;
        let open = self.open;

        let accordion = QUESTIONS.iter().enumerate().fold(
            Accordion::new("faq").bordered(false),
            |accordion, (index, (question, answer))| {
                accordion.item(|item| {
                    item.open(open == Some(index))
                        .title_style(StyleRefinement::default().px_0().py_3())
                        .content_style(StyleRefinement::default().px_0().pb_3())
                        .title(*question)
                        .child(div().text_sm().text_color(muted).child(*answer))
                })
            },
        );

        card(cx)
            .gap_2()
            .child(card_header(
                "Questions, answered",
                "The short version of Paluvio",
                cx,
            ))
            .child(
                accordion.on_toggle_click(cx.listener(|this, open: &[usize], _, cx| {
                    this.open = open.first().copied();
                    cx.notify();
                })),
            )
    }
}

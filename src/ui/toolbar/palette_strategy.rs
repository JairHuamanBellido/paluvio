use gpui_kit::{
    base::Side,
    component::{ActiveTheme, button::Button, menu::DropdownMenu},
    *,
};

use crate::theme::PaletteStrategy;

actions!(palette_strategy, [SelectDefault, SelectPure]);

/// Toolbar dropdown that picks the [`PaletteStrategy`] used to generate the
/// palette. Emits [`PaletteStrategyChanged`]; the owner regenerates.
pub struct PaletteStrategySelector {
    strategy: PaletteStrategy,
    focus_handle: FocusHandle,
}

pub struct PaletteStrategyChanged(pub PaletteStrategy);

impl EventEmitter<PaletteStrategyChanged> for PaletteStrategySelector {}

impl PaletteStrategySelector {
    pub fn new(strategy: PaletteStrategy, cx: &mut Context<Self>) -> Self {
        Self {
            strategy,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn select(&mut self, strategy: PaletteStrategy, cx: &mut Context<Self>) {
        if self.strategy == strategy {
            return;
        }
        self.strategy = strategy;
        cx.emit(PaletteStrategyChanged(strategy));
        cx.notify();
    }
}

fn select_action(strategy: PaletteStrategy) -> Box<dyn Action> {
    match strategy {
        PaletteStrategy::Default => Box::new(SelectDefault),
        PaletteStrategy::Pure => Box::new(SelectPure),
    }
}

fn strategy_option(strategy: PaletteStrategy, cx: &App) -> Div {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .py(px(6.))
        .child(div().text_sm().child(strategy.title()))
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(strategy.description()),
        )
}

impl Render for PaletteStrategySelector {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let current = self.strategy;

        div()
            .track_focus(&focus_handle)
            .on_action(cx.listener(|this, _: &SelectDefault, _, cx| {
                this.select(PaletteStrategy::Default, cx);
            }))
            .on_action(cx.listener(|this, _: &SelectPure, _, cx| {
                this.select(PaletteStrategy::Pure, cx);
            }))
            .child(
                Button::new("palette-strategy-trigger")
                    .icon(icon!("sliders-horizontal-icon.svg"))
                    .accessibility_label("Choose generator method")
                    .tooltip("Generator method")
                    .border_0()
                    .dropdown_caret(false)
                    .dropdown_menu(move |menu, _, _| {
                        PaletteStrategy::ALL.into_iter().fold(
                            menu.action_context(focus_handle.clone())
                                .min_w(px(260.))
                                .check_side(Side::Right)
                                .label("Generator method")
                                .separator(),
                            |menu, strategy| {
                                menu.menu_element_with_check(
                                    strategy == current,
                                    select_action(strategy),
                                    move |_, cx| strategy_option(strategy, cx),
                                )
                            },
                        )
                    }),
            )
    }
}

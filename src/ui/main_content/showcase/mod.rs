//! Card showcase: a 3-column grid that previews the theme on real components.
//!
//! Cards that hold state (inputs, toggles...) are their own `Entity` views,
//! created once in [`Showcase::new`] so state survives re-renders.
//!
//! # Adding a card
//! 1. Create `cards/<name>.rs`: a view struct (or a plain fn for stateless
//!    cards) built on top of [`card::card`].
//! 2. Declare it in `cards/mod.rs` (`mod <name>;` + `pub use`).
//! 3. Stateful card: add an `Entity` field + build it in `Showcase::new`.
//! 4. Add it to a `column_*` method below. List order = top to bottom.

/// Loads a Phosphor icon embedded from `assets/`. Tint with `.text_color(..)`.
macro_rules! icon {
    ($file:literal) => {
        gpui_kit::component::Icon::default().data(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/",
            $file
        )))
    };
}

mod card;
mod cards;
mod noise;

use gpui_kit::*;

use cards::{
    CommitActivity, CreateVault, Faq, SelectPlan, StatusChips, StorageUsage, SubscriptionTrend,
    TaskSettings,
};

pub struct Showcase {
    create_vault: Entity<CreateVault>,
    commit_activity: Entity<CommitActivity>,
    select_plan: Entity<SelectPlan>,
    storage_usage: Entity<StorageUsage>,
    subscription_trend: Entity<SubscriptionTrend>,
    task_settings: Entity<TaskSettings>,
    status_chips: Entity<StatusChips>,
    faq: Entity<Faq>,
}

impl Showcase {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            create_vault: cx.new(|cx| CreateVault::new(window, cx)),
            commit_activity: cx.new(|_| CommitActivity),
            select_plan: cx.new(|_| SelectPlan::new()),
            storage_usage: cx.new(|_| StorageUsage::new()),
            subscription_trend: cx.new(|_| SubscriptionTrend::new()),
            task_settings: cx.new(|_| TaskSettings::new()),
            status_chips: cx.new(|_| StatusChips),
            faq: cx.new(|_| Faq::new()),
        }
    }

    /// Column 1. Add new cards at the end of the list.
    fn column_one(&self) -> Vec<AnyElement> {
        vec![
            self.commit_activity.clone().into_any_element(),
            self.create_vault.clone().into_any_element(),
            self.status_chips.clone().into_any_element(),
        ]
    }

    /// Column 2. Add new cards at the end of the list.
    fn column_two(&self) -> Vec<AnyElement> {
        vec![
            self.select_plan.clone().into_any_element(),
            self.task_settings.clone().into_any_element(),
        ]
    }

    /// Column 3. Add new cards at the end of the list.
    fn column_three(&self) -> Vec<AnyElement> {
        vec![
            self.storage_usage.clone().into_any_element(),
            self.subscription_trend.clone().into_any_element(),
            self.faq.clone().into_any_element(),
        ]
    }
}

fn column(cards: Vec<AnyElement>) -> impl IntoElement {
    div().flex().flex_col().gap_4().min_w_0().children(cards)
}

impl Render for Showcase {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .grid()
            .grid_cols(3)
            .gap_4()
            .items_start()
            .child(column(self.column_one()))
            .child(column(self.column_two()))
            .child(column(self.column_three()))
    }
}

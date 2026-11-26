//! Main-area containers: one per toolbar tab.
//!
//! The toolbar tab bar lists [`ContentTab::ALL`]; the selected tab decides
//! which container fills the main area. Containers are `Entity` views built
//! once in [`Containers::new`], so their state survives tab switches.
//!
//! # Adding a tab
//! 1. Create `containers/<name>.rs` with a view struct that implements `Render`.
//! 2. Declare it below (`mod <name>;` + `use`).
//! 3. Add a [`ContentTab`] variant, list it in [`ContentTab::ALL`] (order =
//!    left to right in the toolbar) and give it a [`ContentTab::label`].
//! 4. Add an `Entity` field to [`Containers`], build it in [`Containers::new`]
//!    and return it from [`Containers::view`]. The compiler flags every
//!    `match` that misses the new variant.

mod auth;

use gpui_kit::*;

use super::showcase::Showcase;
use auth::AuthContainer;

/// A toolbar tab. Each one shows its own container in the main area.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ContentTab {
    #[default]
    Cards,
    Auth,
}

impl ContentTab {
    /// Every tab, in toolbar order.
    pub const ALL: [ContentTab; 2] = [ContentTab::Cards, ContentTab::Auth];

    pub fn label(self) -> &'static str {
        match self {
            ContentTab::Cards => "Cards",
            ContentTab::Auth => "Auth",
        }
    }

    /// Position in [`ContentTab::ALL`].
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|tab| *tab == self)
            .expect("every tab is listed in ContentTab::ALL")
    }
}

/// Holds one container view per tab.
pub struct Containers {
    cards: Entity<Showcase>,
    auth: Entity<AuthContainer>,
}

impl Containers {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        Self {
            cards: cx.new(|cx| Showcase::new(window, cx)),
            auth: cx.new(|_| AuthContainer),
        }
    }

    /// The container to show for `tab`.
    pub fn view(&self, tab: ContentTab) -> AnyView {
        match tab {
            ContentTab::Cards => self.cards.clone().into(),
            ContentTab::Auth => self.auth.clone().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContentTab;

    #[test]
    fn index_matches_position_in_all() {
        for (position, tab) in ContentTab::ALL.iter().enumerate() {
            assert_eq!(tab.index(), position);
        }
    }

    #[test]
    fn cards_is_the_default_tab() {
        assert_eq!(ContentTab::default(), ContentTab::Cards);
    }
}

//! Pure dark surfaces, layered on top of `default::dark`.

use crate::theme::{
    ColorRole,
    palette::{Rule, RuleTable},
};

pub const RULES: RuleTable = &[
    (
        ColorRole::Background,
        Rule::Anchor {
            saturation: 0.0,
            brightness: 0.0,
        },
    ),
    (
        ColorRole::Popover,
        Rule::Anchor {
            saturation: 0.04,
            brightness: 0.06,
        },
    ),
    (
        ColorRole::Sidebar,
        Rule::Anchor {
            saturation: 0.04,
            brightness: 0.04,
        },
    ),
];

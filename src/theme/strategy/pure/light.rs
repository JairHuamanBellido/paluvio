//! Pure light surfaces, layered on top of `default::light`.

use crate::theme::{
    ColorRole,
    palette::{Rule, RuleTable},
};

pub const RULES: RuleTable = &[
    (
        ColorRole::Background,
        Rule::Anchor {
            saturation: 0.0,
            brightness: 1.0,
        },
    ),
    (
        ColorRole::Popover,
        Rule::Anchor {
            saturation: 0.0,
            brightness: 1.0,
        },
    ),
    (
        ColorRole::Sidebar,
        Rule::Anchor {
            saturation: 0.015,
            brightness: 0.99,
        },
    ),
];

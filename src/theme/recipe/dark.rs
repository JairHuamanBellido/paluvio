use crate::theme::{ColorRole, palette::Rule};

pub const DARK: &[(ColorRole, Rule)] = &[
    (
        ColorRole::Background,
        Rule::Anchor {
            saturation: 0.30,
            brightness: 0.08,
        },
    ),
    (
        ColorRole::Foreground,
        Rule::Anchor {
            saturation: 0.03,
            brightness: 0.97,
        },
    ),
    (
        ColorRole::ButtonPrimary,
        Rule::Shift {
            saturation: 0.0,
            brightness: -0.0,
        },
    ),
];

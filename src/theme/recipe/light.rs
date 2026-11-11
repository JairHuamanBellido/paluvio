use crate::theme::{
    ColorRole,
    palette::{Rule, Tone},
};

pub const LIGHT: &[(ColorRole, Rule)] = &[
    (
        ColorRole::Background,
        Rule::Anchor {
            saturation: 0.018,
            brightness: 0.984,
        },
    ),
    (
        ColorRole::Foreground,
        Rule::Anchor {
            saturation: 1.0,
            brightness: 0.055,
        },
    ),
    (
        ColorRole::ButtonPrimary,
        Rule::Shift {
            saturation: 0.0,
            brightness: 0.0,
        },
    ),
    (
        ColorRole::ButtonPrimaryHover,
        Rule::Shift {
            saturation: 0.0,
            brightness: -0.075,
        },
    ),
    (
        ColorRole::ButtonPrimaryActive,
        Rule::Shift {
            saturation: 0.0,
            brightness: 0.0,
        },
    ),
    (
        ColorRole::Ring,
        Rule::Shift {
            saturation: 0.0,
            brightness: 0.0,
        },
    ),
    (
        ColorRole::ButtonPrimaryForeground,
        Rule::Contrast {
            base: ColorRole::ButtonPrimary,
            dark: Tone {
                saturation: 0.9,
                brightness: 0.12,
            }, // tinted near-black
            light: Tone {
                saturation: 0.0,
                brightness: 1.0,
            }, // white
        },
    ),
    (
        ColorRole::Chart1,
        Rule::Shift {
            saturation: 0.0,
            brightness: 0.0,
        },
    ),
    (
        ColorRole::Chart2,
        Rule::Shift {
            saturation: 0.0,
            brightness: -0.15,
        },
    ),
    (
        ColorRole::Chart3,
        Rule::Shift {
            saturation: 0.0,
            brightness: -0.30,
        },
    ),
    (
        ColorRole::Chart4,
        Rule::Shift {
            saturation: 0.0,
            brightness: -0.45,
        },
    ),
    (
        ColorRole::Chart5,
        Rule::Shift {
            saturation: 0.0,
            brightness: -0.6,
        },
    ),
    (
        ColorRole::Selection,
        Rule::Shift {
            saturation: -0.15,
            brightness: 1.0,
        },
    ),
];

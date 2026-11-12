use crate::theme::{
    ColorRole,
    palette::{Rule, Tone},
};

pub const DARK: &[(ColorRole, Rule)] = &[
    (
        ColorRole::Background,
        Rule::Anchor {
            saturation: 0.0725,
            brightness: 0.035,
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
        ColorRole::Button,
        Rule::Contrast {
            base: ColorRole::Primary,
            dark: Tone {
                saturation: 0.0025,
                brightness: 0.025,
            },
            light: Tone {
                saturation: 0.0025,
                brightness: 0.025,
            },
        },
    ),
    (
        ColorRole::ButtonPrimaryForeground,
        Rule::Contrast {
            base: ColorRole::ButtonPrimary,
            dark: Tone {
                saturation: 0.9,
                brightness: 0.12,
            },
            light: Tone {
                saturation: 0.0,
                brightness: 1.0,
            },
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
    (
        ColorRole::DangerForeground,
        Rule::Contrast {
            base: ColorRole::Danger,
            dark: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
            light: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
        },
    ),
    (
        ColorRole::WarningForeground,
        Rule::Contrast {
            base: ColorRole::Warning,
            dark: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
            light: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
        },
    ),
    (
        ColorRole::InfoForeground,
        Rule::Contrast {
            base: ColorRole::Info,
            dark: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
            light: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
        },
    ),
    (
        ColorRole::SuccessForeground,
        Rule::Contrast {
            base: ColorRole::Success,
            dark: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
            light: Tone {
                saturation: 0.90,
                brightness: 0.25,
            },
        },
    ),
    (
        ColorRole::Popover,
        Rule::Contrast {
            base: ColorRole::Primary,
            dark: Tone {
                saturation: 0.1075,
                brightness: 0.0775,
            },
            light: Tone {
                saturation: 0.1075,
                brightness: 0.0775,
            },
        },
    ),
];

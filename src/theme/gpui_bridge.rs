//! Bridge between the theme domain and gpui-component's `Theme`.
//!
//! Reads pristine library colors per role, and writes generated colors back
//! into the live theme.

use std::sync::Arc;

use gpui_kit::{
    App, Hsla,
    component::{Theme, ThemeColor, ThemeMode},
};

use super::{ColorRole, PaletteState};

/// Single role-to-field list. Generates both directions, so a new `ColorRole`
/// is added in exactly one place and both matches stay exhaustive.
macro_rules! color_roles {
    ($($role:ident => $field:ident,)*) => {
        /// Library default for `role`.
        ///
        /// Used as the fallback for roles the recipe does not drive.
        pub(super) fn default_color(role: ColorRole, defaults: &ThemeColor) -> Hsla {
            match role {
                $(ColorRole::$role => defaults.$field,)*
            }
        }

        /// Writes `value` into the live theme field of `role`.
        fn set_color(colors: &mut ThemeColor, role: ColorRole, value: Hsla) {
            match role {
                $(ColorRole::$role => colors.$field = value,)*
            }
        }
    };
}

color_roles! {
    Primary => primary,
    Background => background,
    Foreground => foreground,
    Border => border,
    PrimaryForeground => primary_foreground,
    PrimaryHover => primary_hover,
    PrimaryActive => primary_active,
    Secondary => secondary,
    SecondaryForeground => secondary_foreground,
    SecondaryHover => secondary_hover,
    SecondaryActive => secondary_active,
    Accent => accent,
    AccentForeground => accent_foreground,
    Muted => muted,
    MutedForeground => muted_foreground,
    Ring => ring,
    Button => button,
    ButtonForeground => button_foreground,
    ButtonHover => button_hover,
    ButtonActive => button_active,
    ButtonPrimary => button_primary,
    ButtonPrimaryForeground => button_primary_foreground,
    ButtonPrimaryHover => button_primary_hover,
    ButtonPrimaryActive => button_primary_active,
    ButtonSecondary => button_secondary,
    ButtonSecondaryForeground => button_secondary_foreground,
    ButtonSecondaryHover => button_secondary_hover,
    ButtonSecondaryActive => button_secondary_active,
    ButtonDanger => button_danger,
    ButtonDangerForeground => button_danger_foreground,
    ButtonDangerHover => button_danger_hover,
    ButtonDangerActive => button_danger_active,
    ButtonWarning => button_warning,
    ButtonWarningForeground => button_warning_foreground,
    ButtonWarningHover => button_warning_hover,
    ButtonWarningActive => button_warning_active,
    ButtonSuccess => button_success,
    ButtonSuccessForeground => button_success_foreground,
    ButtonSuccessHover => button_success_hover,
    ButtonSuccessActive => button_success_active,
    ButtonInfo => button_info,
    ButtonInfoForeground => button_info_foreground,
    ButtonInfoHover => button_info_hover,
    ButtonInfoActive => button_info_active,
    Danger => danger,
    DangerForeground => danger_foreground,
    DangerHover => danger_hover,
    DangerActive => danger_active,
    Warning => warning,
    WarningForeground => warning_foreground,
    WarningHover => warning_hover,
    WarningActive => warning_active,
    Success => success,
    SuccessForeground => success_foreground,
    SuccessHover => success_hover,
    SuccessActive => success_active,
    Info => info,
    InfoForeground => info_foreground,
    InfoHover => info_hover,
    InfoActive => info_active,
    Accordion => accordion,
    GroupBox => group_box,
    GroupBoxForeground => group_box_foreground,
    Popover => popover,
    PopoverForeground => popover_foreground,
    Overlay => overlay,
    Sidebar => sidebar,
    SidebarForeground => sidebar_foreground,
    SidebarBorder => sidebar_border,
    SidebarPrimary => sidebar_primary,
    SidebarPrimaryForeground => sidebar_primary_foreground,
    SidebarAccent => sidebar_accent,
    SidebarAccentForeground => sidebar_accent_foreground,
    TabBar => tab_bar,
    TabBarSegmented => tab_bar_segmented,
    Tab => tab,
    TabForeground => tab_foreground,
    TabActive => tab_active,
    TabActiveForeground => tab_active_foreground,
    Link => link,
    LinkHover => link_hover,
    LinkActive => link_active,
    Input => input,
    Caret => caret,
    Selection => selection,
    Switch => switch,
    SwitchThumb => switch_thumb,
    SliderBar => slider_bar,
    SliderThumb => slider_thumb,
    ProgressBar => progress_bar,
    Skeleton => skeleton,
    List => list,
    ListHead => list_head,
    ListHover => list_hover,
    ListActive => list_active,
    ListActiveBorder => list_active_border,
    ListEven => list_even,
    DescriptionListLabel => description_list_label,
    DescriptionListLabelForeground => description_list_label_foreground,
    Table => table,
    TableHead => table_head,
    TableHeadForeground => table_head_foreground,
    TableFoot => table_foot,
    TableFootForeground => table_foot_foreground,
    TableHover => table_hover,
    TableActive => table_active,
    TableActiveBorder => table_active_border,
    TableEven => table_even,
    TableRowBorder => table_row_border,
    TitleBar => title_bar,
    TitleBarBorder => title_bar_border,
    StatusBar => status_bar,
    StatusBarBorder => status_bar_border,
    WindowBorder => window_border,
    Scrollbar => scrollbar,
    ScrollbarThumb => scrollbar_thumb,
    ScrollbarThumbHover => scrollbar_thumb_hover,
    DragBorder => drag_border,
    DropTarget => drop_target,
    Chart1 => chart_1,
    Chart2 => chart_2,
    Chart3 => chart_3,
    Chart4 => chart_4,
    Chart5 => chart_5,
    ChartBullish => chart_bullish,
    ChartBearish => chart_bearish,
    ChartGrid => chart_grid,
    Red => red,
    RedLight => red_light,
    Green => green,
    GreenLight => green_light,
    Blue => blue,
    BlueLight => blue_light,
    Yellow => yellow,
    YellowLight => yellow_light,
    Magenta => magenta,
    MagentaLight => magenta_light,
    Cyan => cyan,
    CyanLight => cyan_light,
}

/// Writes every palette variable into the live gpui-component `Theme`.
///
/// Covers all roles, generated or manually edited. `Theme::update` reconciles
/// the semantic tokens from `colors` afterwards.
pub fn apply(palette: &PaletteState, cx: &mut App) {
    Theme::update(cx, |theme| {
        theme.focus_ring = false;
        for role in ColorRole::ALL {
            set_color(
                &mut theme.colors,
                role,
                palette.variable(role).value.to_color(),
            );
        }
    });
}

/// Pristine library colors for `mode`.
///
/// Read from `ThemeColor`, not from the live `Theme`, because `theme::apply`
/// overwrites the live values with generated ones.
pub(super) fn default_colors(mode: ThemeMode) -> Arc<ThemeColor> {
    if mode.is_dark() {
        ThemeColor::dark()
    } else {
        ThemeColor::light()
    }
}

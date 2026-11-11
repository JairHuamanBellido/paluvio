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

/// Writes generated colors into the live gpui-component `Theme`.
///
/// Only Background and Foreground are mapped so far. Map more roles here as
/// the recipe starts driving them.
pub fn apply(palette: &PaletteState, cx: &mut App) {
    Theme::update(cx, |theme| {
        theme.colors.background = palette.variable(ColorRole::Background).value.to_color();
        theme.colors.foreground = palette.variable(ColorRole::Foreground).value.to_color();
    });
}

/// Library default for `role`. Exhaustive on purpose: adding a
/// `ColorRole` fails to compile until it gets a default here.
///
/// Used as the fallback for roles the recipe does not drive yet.
pub(super) fn default_color(role: ColorRole, defaults: &ThemeColor) -> Hsla {
    match role {
        ColorRole::Primary => defaults.primary,
        ColorRole::Background => defaults.background,
        ColorRole::Foreground => defaults.foreground,
        ColorRole::Border => defaults.border,
        ColorRole::PrimaryForeground => defaults.primary_foreground,
        ColorRole::PrimaryHover => defaults.primary_hover,
        ColorRole::PrimaryActive => defaults.primary_active,
        ColorRole::Secondary => defaults.secondary,
        ColorRole::SecondaryForeground => defaults.secondary_foreground,
        ColorRole::SecondaryHover => defaults.secondary_hover,
        ColorRole::SecondaryActive => defaults.secondary_active,
        ColorRole::Accent => defaults.accent,
        ColorRole::AccentForeground => defaults.accent_foreground,
        ColorRole::Muted => defaults.muted,
        ColorRole::MutedForeground => defaults.muted_foreground,
        ColorRole::Ring => defaults.ring,
        ColorRole::Button => defaults.button,
        ColorRole::ButtonForeground => defaults.button_foreground,
        ColorRole::ButtonHover => defaults.button_hover,
        ColorRole::ButtonActive => defaults.button_active,
        ColorRole::ButtonPrimary => defaults.button_primary,
        ColorRole::ButtonPrimaryForeground => defaults.button_primary_foreground,
        ColorRole::ButtonPrimaryHover => defaults.button_primary_hover,
        ColorRole::ButtonPrimaryActive => defaults.button_primary_active,
        ColorRole::ButtonSecondary => defaults.button_secondary,
        ColorRole::ButtonSecondaryForeground => defaults.button_secondary_foreground,
        ColorRole::ButtonSecondaryHover => defaults.button_secondary_hover,
        ColorRole::ButtonSecondaryActive => defaults.button_secondary_active,
        ColorRole::ButtonDanger => defaults.button_danger,
        ColorRole::ButtonDangerForeground => defaults.button_danger_foreground,
        ColorRole::ButtonDangerHover => defaults.button_danger_hover,
        ColorRole::ButtonDangerActive => defaults.button_danger_active,
        ColorRole::ButtonWarning => defaults.button_warning,
        ColorRole::ButtonWarningForeground => defaults.button_warning_foreground,
        ColorRole::ButtonWarningHover => defaults.button_warning_hover,
        ColorRole::ButtonWarningActive => defaults.button_warning_active,
        ColorRole::ButtonSuccess => defaults.button_success,
        ColorRole::ButtonSuccessForeground => defaults.button_success_foreground,
        ColorRole::ButtonSuccessHover => defaults.button_success_hover,
        ColorRole::ButtonSuccessActive => defaults.button_success_active,
        ColorRole::ButtonInfo => defaults.button_info,
        ColorRole::ButtonInfoForeground => defaults.button_info_foreground,
        ColorRole::ButtonInfoHover => defaults.button_info_hover,
        ColorRole::ButtonInfoActive => defaults.button_info_active,
        ColorRole::Danger => defaults.danger,
        ColorRole::DangerForeground => defaults.danger_foreground,
        ColorRole::DangerHover => defaults.danger_hover,
        ColorRole::DangerActive => defaults.danger_active,
        ColorRole::Warning => defaults.warning,
        ColorRole::WarningForeground => defaults.warning_foreground,
        ColorRole::WarningHover => defaults.warning_hover,
        ColorRole::WarningActive => defaults.warning_active,
        ColorRole::Success => defaults.success,
        ColorRole::SuccessForeground => defaults.success_foreground,
        ColorRole::SuccessHover => defaults.success_hover,
        ColorRole::SuccessActive => defaults.success_active,
        ColorRole::Info => defaults.info,
        ColorRole::InfoForeground => defaults.info_foreground,
        ColorRole::InfoHover => defaults.info_hover,
        ColorRole::InfoActive => defaults.info_active,
        ColorRole::Accordion => defaults.accordion,
        ColorRole::GroupBox => defaults.group_box,
        ColorRole::GroupBoxForeground => defaults.group_box_foreground,
        ColorRole::Popover => defaults.popover,
        ColorRole::PopoverForeground => defaults.popover_foreground,
        ColorRole::Overlay => defaults.overlay,
        ColorRole::Sidebar => defaults.sidebar,
        ColorRole::SidebarForeground => defaults.sidebar_foreground,
        ColorRole::SidebarBorder => defaults.sidebar_border,
        ColorRole::SidebarPrimary => defaults.sidebar_primary,
        ColorRole::SidebarPrimaryForeground => defaults.sidebar_primary_foreground,
        ColorRole::SidebarAccent => defaults.sidebar_accent,
        ColorRole::SidebarAccentForeground => defaults.sidebar_accent_foreground,
        ColorRole::TabBar => defaults.tab_bar,
        ColorRole::TabBarSegmented => defaults.tab_bar_segmented,
        ColorRole::Tab => defaults.tab,
        ColorRole::TabForeground => defaults.tab_foreground,
        ColorRole::TabActive => defaults.tab_active,
        ColorRole::TabActiveForeground => defaults.tab_active_foreground,
        ColorRole::Link => defaults.link,
        ColorRole::LinkHover => defaults.link_hover,
        ColorRole::LinkActive => defaults.link_active,
        ColorRole::Input => defaults.input,
        ColorRole::Caret => defaults.caret,
        ColorRole::Selection => defaults.selection,
        ColorRole::Switch => defaults.switch,
        ColorRole::SwitchThumb => defaults.switch_thumb,
        ColorRole::SliderBar => defaults.slider_bar,
        ColorRole::SliderThumb => defaults.slider_thumb,
        ColorRole::ProgressBar => defaults.progress_bar,
        ColorRole::Skeleton => defaults.skeleton,
        ColorRole::List => defaults.list,
        ColorRole::ListHead => defaults.list_head,
        ColorRole::ListHover => defaults.list_hover,
        ColorRole::ListActive => defaults.list_active,
        ColorRole::ListActiveBorder => defaults.list_active_border,
        ColorRole::ListEven => defaults.list_even,
        ColorRole::DescriptionListLabel => defaults.description_list_label,
        ColorRole::DescriptionListLabelForeground => defaults.description_list_label_foreground,
        ColorRole::Table => defaults.table,
        ColorRole::TableHead => defaults.table_head,
        ColorRole::TableHeadForeground => defaults.table_head_foreground,
        ColorRole::TableFoot => defaults.table_foot,
        ColorRole::TableFootForeground => defaults.table_foot_foreground,
        ColorRole::TableHover => defaults.table_hover,
        ColorRole::TableActive => defaults.table_active,
        ColorRole::TableActiveBorder => defaults.table_active_border,
        ColorRole::TableEven => defaults.table_even,
        ColorRole::TableRowBorder => defaults.table_row_border,
        ColorRole::TitleBar => defaults.title_bar,
        ColorRole::TitleBarBorder => defaults.title_bar_border,
        ColorRole::StatusBar => defaults.status_bar,
        ColorRole::StatusBarBorder => defaults.status_bar_border,
        ColorRole::WindowBorder => defaults.window_border,
        ColorRole::Scrollbar => defaults.scrollbar,
        ColorRole::ScrollbarThumb => defaults.scrollbar_thumb,
        ColorRole::ScrollbarThumbHover => defaults.scrollbar_thumb_hover,
        ColorRole::DragBorder => defaults.drag_border,
        ColorRole::DropTarget => defaults.drop_target,
        ColorRole::Chart1 => defaults.chart_1,
        ColorRole::Chart2 => defaults.chart_2,
        ColorRole::Chart3 => defaults.chart_3,
        ColorRole::Chart4 => defaults.chart_4,
        ColorRole::Chart5 => defaults.chart_5,
        ColorRole::ChartBullish => defaults.chart_bullish,
        ColorRole::ChartBearish => defaults.chart_bearish,
        ColorRole::ChartGrid => defaults.chart_grid,
        ColorRole::Red => defaults.red,
        ColorRole::RedLight => defaults.red_light,
        ColorRole::Green => defaults.green,
        ColorRole::GreenLight => defaults.green_light,
        ColorRole::Blue => defaults.blue,
        ColorRole::BlueLight => defaults.blue_light,
        ColorRole::Yellow => defaults.yellow,
        ColorRole::YellowLight => defaults.yellow_light,
        ColorRole::Magenta => defaults.magenta,
        ColorRole::MagentaLight => defaults.magenta_light,
        ColorRole::Cyan => defaults.cyan,
        ColorRole::CyanLight => defaults.cyan_light,
    }
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

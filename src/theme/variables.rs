use gpui_kit::{component::ThemeColor, rgb};

use crate::theme::role::ColorRole;

use super::{Hsv, PaletteRecipe};

pub struct ColorVariable {
    pub role: ColorRole,
    pub value: Hsv,
    pub customized: bool,
}

pub struct PaletteState {
    recipe: PaletteRecipe,
    variables: Vec<ColorVariable>,
}

impl PaletteState {
    pub fn new() -> Self {
        let recipe = PaletteRecipe::default();
        let primary = Hsv::from_color(rgb(0x3b82f6).into());
        let generated = recipe.generate(primary.to_color());
        let defaults = ThemeColor::light();
        let variables = ColorRole::ALL
            .into_iter()
            .map(|role| {
                let color = match role {
                    ColorRole::Primary => primary.to_color(),
                    ColorRole::Background => generated.background,
                    ColorRole::Foreground => generated.foreground,
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
                    ColorRole::DescriptionListLabelForeground => {
                        defaults.description_list_label_foreground
                    }
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
                };
                ColorVariable {
                    role,
                    value: Hsv::from_color(color),
                    customized: false,
                }
            })
            .collect();
        Self { recipe, variables }
    }

    pub fn variable(&self, role: ColorRole) -> &ColorVariable {
        self.variables
            .iter()
            .find(|variable| variable.role == role)
            .expect("registered color role")
    }

    pub fn set(&mut self, role: ColorRole, value: Hsv) {
        let variable = self
            .variables
            .iter_mut()
            .find(|variable| variable.role == role)
            .expect("registered color role");
        variable.value = Hsv::new(value.hue, value.saturation, value.brightness);
        variable.customized = true;
        if role == ColorRole::Primary {
            self.regenerate_from_primary();
        }
    }

    pub fn reset(&mut self, role: ColorRole) {
        let variable = self
            .variables
            .iter_mut()
            .find(|variable| variable.role == role)
            .expect("registered color role");
        variable.customized = false;
        if role == ColorRole::Primary {
            variable.value = Hsv::from_color(rgb(0x3b82f6).into());
            self.regenerate_from_primary();
        } else {
            self.regenerate();
        }
    }

    fn regenerate_from_primary(&mut self) {
        for variable in &mut self.variables {
            if variable.role != ColorRole::Primary {
                variable.customized = false;
            }
        }
        self.regenerate();
    }

    fn regenerate(&mut self) {
        let generated = self
            .recipe
            .generate(self.variable(ColorRole::Primary).value.to_color());
        for variable in &mut self.variables {
            if variable.customized {
                continue;
            }
            let color = match variable.role {
                ColorRole::Primary => continue,
                ColorRole::Background => generated.background,
                ColorRole::Foreground => generated.foreground,
                _ => continue,
            };
            variable.value = Hsv::from_color(color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_role_has_an_initial_value() {
        let palette = PaletteState::new();
        assert_eq!(palette.variables.len(), ColorRole::ALL.len());
        for role in ColorRole::ALL {
            let variable = palette.variable(role);
            assert_eq!(variable.role, role);
            assert!(!variable.customized);
            assert!(variable.value.hue.is_finite());
            assert!(variable.value.saturation.is_finite());
            assert!(variable.value.brightness.is_finite());
        }
    }
}

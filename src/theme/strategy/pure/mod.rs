//! Pure strategy: plain white (light) and black (dark) backgrounds, with a
//! barely visible seed tint on raised surfaces (popover, sidebar). Keeps
//! every other rule of the default strategy.

mod dark;
mod light;

use super::default;
use crate::theme::palette::RuleTable;

pub const LIGHT: &[RuleTable] = &[default::light::RULES, light::RULES];
pub const DARK: &[RuleTable] = &[default::dark::RULES, dark::RULES];

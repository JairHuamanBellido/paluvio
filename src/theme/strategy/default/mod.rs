//! Default strategy: seed-tinted surfaces. Base rules every strategy builds on.

pub mod dark;
pub mod light;

use crate::theme::palette::RuleTable;

pub const LIGHT: &[RuleTable] = &[light::RULES];
pub const DARK: &[RuleTable] = &[dark::RULES];

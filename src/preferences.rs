use std::{fs, io, path::PathBuf};

use crate::{theme::PaletteStrategy, ui::ThemePreference};

const APP_DIR: &str = "paluvio";
const THEME_FILE: &str = "theme-preference";
const STRATEGY_FILE: &str = "palette-strategy";

/// `<config dir>/paluvio/<file>`, or `None` when the OS has no config
/// directory.
fn file_path(file: &str) -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join(APP_DIR).join(file))
}

fn load<T: std::str::FromStr>(file: &str) -> Option<T> {
    file_path(file)
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|text| text.trim().parse().ok())
}

fn save(file: &str, value: &str) -> io::Result<()> {
    let path = file_path(file).ok_or_else(|| io::Error::other("no config directory"))?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, value)
}

/// Reads the saved preference. Falls back to `System` when the file is
/// missing, unreadable, or holds an unknown value.
pub fn load_theme_preference() -> ThemePreference {
    load(THEME_FILE).unwrap_or(ThemePreference::System)
}

pub fn save_theme_preference(preference: ThemePreference) -> io::Result<()> {
    save(THEME_FILE, preference.as_str())
}

/// Reads the saved palette strategy. Falls back to the default strategy when
/// the file is missing, unreadable, or holds an unknown value.
pub fn load_palette_strategy() -> PaletteStrategy {
    load(STRATEGY_FILE).unwrap_or_default()
}

pub fn save_palette_strategy(strategy: PaletteStrategy) -> io::Result<()> {
    save(STRATEGY_FILE, strategy.as_str())
}

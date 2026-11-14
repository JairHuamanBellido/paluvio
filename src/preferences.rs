use std::{fs, io, path::PathBuf};

use crate::ui::ThemePreference;

const APP_DIR: &str = "paluvio";
const THEME_FILE: &str = "theme-preference";

/// `<config dir>/paluvio/theme-preference`, or `None` when the OS has no
/// config directory.
fn theme_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join(APP_DIR).join(THEME_FILE))
}

/// Reads the saved preference. Falls back to `System` when the file is
/// missing, unreadable, or holds an unknown value.
pub fn load_theme_preference() -> ThemePreference {
    theme_file_path()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|text| text.trim().parse().ok())
        .unwrap_or(ThemePreference::System)
}

pub fn save_theme_preference(preference: ThemePreference) -> io::Result<()> {
    let path = theme_file_path().ok_or_else(|| io::Error::other("no config directory"))?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, preference.as_str())
}

/// Loads a Phosphor icon embedded from `assets/`. Tint with `.text_color(..)`.
macro_rules! icon {
    ($file:literal) => {
        gpui_kit::component::Icon::default().data(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/",
            $file
        )))
    };
}

mod color_editor;
mod color_field;
mod header;
mod layout;
mod main_content;
mod main_view;
mod sidebar;
mod toolbar;
mod variable_swatch;

pub use main_view::MainView;
pub use toolbar::ThemePreference;

use gpui_kit::{component::TitleBar, *};

mod preferences;
mod theme;
mod ui;

use ui::MainView;

fn main() {
    application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

            let options = WindowOptions {
                window_min_size: Some(size(px(800.), px(500.))),
                titlebar: Some(TitlebarOptions {
                    // Center native traffic lights inside the 48px header.
                    traffic_light_position: Some(point(px(9.), px(17.))),
                    ..TitleBar::title_bar_options()
                }),
                ..TitleBar::window_options()
            };
            cx.text_system()
                // Register separate faces: this backend does not select variable weights.
                .add_fonts(
                    [
                        include_bytes!("../assets/fonts/manrope/static/Manrope-ExtraLight.ttf")
                            .as_slice(),
                        include_bytes!("../assets/fonts/manrope/static/Manrope-Light.ttf")
                            .as_slice(),
                        include_bytes!("../assets/fonts/manrope/static/Manrope-Regular.ttf")
                            .as_slice(),
                        include_bytes!("../assets/fonts/manrope/static/Manrope-Medium.ttf")
                            .as_slice(),
                        include_bytes!("../assets/fonts/manrope/static/Manrope-SemiBold.ttf")
                            .as_slice(),
                        include_bytes!("../assets/fonts/manrope/static/Manrope-Bold.ttf")
                            .as_slice(),
                        include_bytes!("../assets/fonts/manrope/static/Manrope-ExtraBold.ttf")
                            .as_slice(),
                    ]
                    .into_iter()
                    .map(std::borrow::Cow::Borrowed)
                    .collect(),
                )
                .expect("failed to load Manrope");
            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| MainView::new(window, cx))
            })
            .expect("failed to open Paluvio");

            cx.activate(true);
        });
}

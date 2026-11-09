use gpui_kit::*;

mod theme;
mod ui;

use ui::MainView;

fn main() {
    application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

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
            gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
                cx.new(|cx| MainView::new(window, cx))
            })
            .expect("failed to open Paluvio");

            cx.activate(true);
        });
}

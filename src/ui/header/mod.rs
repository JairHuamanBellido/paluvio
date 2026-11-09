use std::sync::{Arc, LazyLock};

use gpui_kit::{component::TitleBar, *};

static LOGO: LazyLock<Arc<Image>> = LazyLock::new(|| {
    Arc::new(Image::from_bytes(
        ImageFormat::Svg,
        include_bytes!("../../../assets/logo.svg").to_vec(),
    ))
});

pub fn app_header() -> impl IntoElement {
    TitleBar::new()
        .h(px(48.))
        .bg(rgb(0xffffff))
        .border_b_0()
        .child(
            div()
                .h_full()
                .flex()
                .items_center()
                .gap_2()
                .pl_2()
                .border_0()
                .child(img(Arc::clone(&LOGO)).size(px(24.)).flex_shrink_0())
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_base()
                        .text_color(rgb(0x40574D))
                        .child("Paluvio"),
                ),
        )
}

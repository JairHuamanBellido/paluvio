use gpui_kit::{
    component::{ActiveTheme, TitleBar},
    *,
};
use std::sync::{Arc, LazyLock};

static LOGO: LazyLock<Arc<Image>> = LazyLock::new(|| {
    Arc::new(Image::from_bytes(
        ImageFormat::Svg,
        include_bytes!("../../../assets/logo.svg").to_vec(),
    ))
});

#[derive(IntoElement)]
pub struct Header {
    toolbar: AnyElement,
}

impl Header {
    pub fn new(toolbar: impl IntoElement) -> Self {
        Self {
            toolbar: toolbar.into_any_element(),
        }
    }
}

impl RenderOnce for Header {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark_mode = cx.theme().is_dark();
        let background_color = if is_dark_mode {
            rgb(0x000000)
        } else {
            rgb(0xffffff)
        };

        TitleBar::new()
            .h(px(48.))
            .border_b_0()
            .bg(background_color)
            .flex()
            .justify_between()
            .items_center()
            .pr_10()
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
            .child(self.toolbar)
    }
}

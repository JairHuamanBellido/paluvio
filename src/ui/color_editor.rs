use gpui_kit::{
    component::{
        ActiveTheme, Colorize,
        button::Button,
        input::{Input, InputEvent, InputState},
    },
    *,
};

use super::color_field::{ColorField, ColorFieldEvent};
use crate::theme::{ColorRole, Hsv};

pub enum ColorEditorEvent {
    Change(Hsv),
    Reset,
}

/// Reusable variable editor, independent of theme mapping and palette generation.
pub struct ColorEditor {
    role: ColorRole,
    value: Hsv,
    field: Entity<ColorField>,
    hex_input: Entity<InputState>,
    copied: bool,
    invalid_hex: bool,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<ColorEditorEvent> for ColorEditor {}

impl ColorEditor {
    pub fn new(role: ColorRole, value: Hsv, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let field = cx.new(|cx| ColorField::new(value, cx));
        let hex_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(value.to_hex())
                .placeholder("#RRGGBB")
        });
        let field_subscription = cx.subscribe_in(
            &field,
            window,
            |this, _, event: &ColorFieldEvent, window, cx| {
                this.value = event.0;
                this.copied = false;
                this.invalid_hex = false;
                this.hex_input.update(cx, |input, cx| {
                    input.set_value(event.0.to_hex(), window, cx)
                });
                cx.emit(ColorEditorEvent::Change(event.0));
                cx.notify();
            },
        );
        let hex_subscription =
            cx.subscribe_in(&hex_input, window, |this, _, event, _, cx| match event {
                InputEvent::PressEnter { .. } => this.apply_hex(cx),
                InputEvent::Change => {
                    this.invalid_hex = false;
                    cx.notify();
                }
                _ => {}
            });
        Self {
            role,
            value,
            field,
            hex_input,
            copied: false,
            invalid_hex: false,
            _subscriptions: vec![field_subscription, hex_subscription],
        }
    }

    pub fn set_value(&mut self, value: Hsv, window: &mut Window, cx: &mut Context<Self>) {
        if self.value == value {
            return;
        }
        self.value = value;
        self.copied = false;
        self.field
            .update(cx, |field, cx| field.set_value(value, cx));
        self.invalid_hex = false;
        self.hex_input
            .update(cx, |input, cx| input.set_value(value.to_hex(), window, cx));
        cx.notify();
    }

    fn apply_hex(&mut self, cx: &mut Context<Self>) {
        let text = self.hex_input.read(cx).value();
        let Some(color) = parse_hex(&text) else {
            self.invalid_hex = true;
            cx.notify();
            return;
        };
        let mut value = Hsv::from_color(color);
        if value.saturation == 0. {
            value.hue = self.value.hue;
        }
        // Preserve raw input and focus while applying; no popup or dismissal event.
        self.value = value;
        self.copied = false;
        self.invalid_hex = false;
        self.field
            .update(cx, |field, cx| field.set_value(value, cx));
        cx.emit(ColorEditorEvent::Change(value));
        cx.notify();
    }
}

impl Render for ColorEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let color = Rgba::from(self.value.to_color());
        div()
            .id("color-editor-scroll")
            .w(px(260.))
            .max_h((window.viewport_size().height - px(40.)).max(px(160.)))
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .gap_3()
            .text_color(cx.theme().popover_foreground)
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.role.name()),
            )
            .child(self.field.clone())
            .child(
                div()
                    .flex()
                    .gap_2()
                    .items_center()
                    .child(Input::new(&self.hex_input).flex_1())
                    .child(
                        Button::new("apply-hex")
                            .label("Apply")
                            .on_click(cx.listener(|this, _, _, cx| this.apply_hex(cx))),
                    ),
            )
            .children(
                self.invalid_hex
                    .then_some("Enter six hex digits, such as #3B82F6."),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!(
                        "RGB  {}, {}, {}",
                        (color.r * 255.).round() as u8,
                        (color.g * 255.).round() as u8,
                        (color.b * 255.).round() as u8
                    )),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("copy-values")
                            .label(if self.copied { "Copied" } else { "Copy values" })
                            .on_click(cx.listener(|this, _, _, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(
                                    this.value.rust_value(this.role.key()),
                                ));
                                this.copied = true;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("reset-color")
                            .label("Reset")
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(ColorEditorEvent::Reset))),
                    ),
            )
    }
}

fn parse_hex(text: &str) -> Option<Hsla> {
    let text = text.trim();
    let digits = text.strip_prefix('#').unwrap_or(text);
    if digits.len() != 6 || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    Hsla::parse_hex(&format!("#{digits}")).ok()
}

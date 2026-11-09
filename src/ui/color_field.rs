use std::{cell::Cell, rc::Rc};

use gpui_kit::*;

use crate::theme::Hsv;

pub struct ColorField {
    value: Hsv,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    dragging: bool,
    hue_bounds: Rc<Cell<Bounds<Pixels>>>,
    hue_dragging: bool,
    hue_focus: FocusHandle,
}

pub struct ColorFieldEvent(pub Hsv);
impl EventEmitter<ColorFieldEvent> for ColorField {}

impl ColorField {
    pub fn new(value: Hsv, cx: &mut Context<Self>) -> Self {
        Self {
            value,
            bounds: Rc::new(Cell::new(Bounds::default())),
            dragging: false,
            hue_bounds: Rc::new(Cell::new(Bounds::default())),
            hue_dragging: false,
            hue_focus: cx.focus_handle(),
        }
    }

    pub fn set_value(&mut self, value: Hsv, cx: &mut Context<Self>) {
        self.value = value;
        cx.notify();
    }

    fn emit(&mut self, cx: &mut Context<Self>) {
        cx.emit(ColorFieldEvent(self.value));
        cx.notify();
    }

    fn pick_hue(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let bounds = self.hue_bounds.get();
        if bounds.size.width > px(0.) {
            self.value.hue = ((position.x - bounds.origin.x) / bounds.size.width).clamp(0., 1.);
            self.emit(cx);
        }
    }

    fn pick(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let bounds = self.bounds.get();
        if bounds.size.width <= px(0.) || bounds.size.height <= px(0.) {
            return;
        }
        self.value.saturation = ((position.x - bounds.origin.x) / bounds.size.width).clamp(0., 1.);
        self.value.brightness =
            1. - ((position.y - bounds.origin.y) / bounds.size.height).clamp(0., 1.);
        self.emit(cx);
    }
}

impl Render for ColorField {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bounds = self.bounds.clone();
        let hue = Hsv::new(self.value.hue, 1., 1.).to_color();
        let hue_bounds = self.hue_bounds.clone();
        div()
            .w(px(260.))
            .flex_shrink_0()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .id("saturation-brightness")
                    .relative()
                    .w_full()
                    .h(px(200.))
                    .flex_shrink_0()
                    .rounded_lg()
                    .cursor_crosshair()
                    .bg(linear_gradient(
                        90.,
                        linear_color_stop(rgb(0xffffff), 0.),
                        linear_color_stop(hue, 1.),
                    ))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, _, cx| {
                            this.dragging = true;
                            this.pick(event.position, cx);
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                        if this.dragging && event.dragging() {
                            this.pick(event.position, cx);
                        }
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| this.dragging = false),
                    )
                    .on_mouse_up_out(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| this.dragging = false),
                    )
                    .child(
                        div()
                            .absolute()
                            .size_full()
                            .rounded_lg()
                            .bg(linear_gradient(
                                180.,
                                linear_color_stop(hsla(0., 0., 0., 0.), 0.),
                                linear_color_stop(rgb(0x000000), 1.),
                            )),
                    )
                    .child(
                        canvas(move |area, _, _| bounds.set(area), |_, _, _, _| {})
                            .absolute()
                            .size_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(relative(self.value.saturation))
                            .top(relative(1. - self.value.brightness))
                            .ml(px(-7.))
                            .mt(px(-7.))
                            .size(px(14.))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(0xffffff))
                            .shadow_sm()
                            .bg(self.value.to_color()),
                    ),
            )
            .child(
                div()
                    .id("hue-strip")
                    .relative()
                    .h(px(24.))
                    .w_full()
                    .cursor_pointer()
                    .track_focus(&self.hue_focus)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, window, cx| {
                            this.hue_dragging = true;
                            window.focus(&this.hue_focus, cx);
                            this.pick_hue(event.position, cx);
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                        if this.hue_dragging && event.dragging() {
                            this.pick_hue(event.position, cx);
                        }
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| this.hue_dragging = false),
                    )
                    .on_mouse_up_out(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| this.hue_dragging = false),
                    )
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                        let hue = match event.keystroke.key.as_str() {
                            "left" | "down" => this.value.hue - 1. / 360.,
                            "right" | "up" => this.value.hue + 1. / 360.,
                            "home" => 0.,
                            "end" => 1.,
                            _ => return,
                        };
                        this.value.hue = hue.clamp(0., 1.);
                        this.emit(cx);
                        cx.stop_propagation();
                    }))
                    .child(
                        div()
                            .absolute()
                            .top(px(6.))
                            .w_full()
                            .h(px(12.))
                            .flex()
                            .rounded_full()
                            .overflow_hidden()
                            .children((0..6).map(|index| {
                                div().flex_1().h_full().bg(linear_gradient(
                                    90.,
                                    linear_color_stop(
                                        Hsv::new(index as f32 / 6., 1., 1.).to_color(),
                                        0.,
                                    ),
                                    linear_color_stop(
                                        Hsv::new((index + 1) as f32 / 6., 1., 1.).to_color(),
                                        1.,
                                    ),
                                ))
                            })),
                    )
                    .child(
                        canvas(move |area, _, _| hue_bounds.set(area), |_, _, _, _| {})
                            .absolute()
                            .size_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(relative(self.value.hue))
                            .top(px(3.))
                            .ml(px(-9.))
                            .size(px(18.))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(0xffffff))
                            .shadow_sm()
                            .bg(hue),
                    ),
            )
    }
}

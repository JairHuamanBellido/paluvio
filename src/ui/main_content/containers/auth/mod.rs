//! Auth container: a sign-in screen for a sample app ("Colony").
//!
//! The whole container fades left to right from a faint `primary` tint into
//! the plain `background` (see [`GRADIENT_TINT`]).
//!
//! Left [`LIFE_SHARE`] of the width: brand, Conway's Game of Life and a
//! tagline. Cells are drawn in `primary`; a cell's opacity grows with its age
//! (40% when born, 100% once settled). Dead cells are not drawn. Every live
//! cell glows as strongly as it is opaque, and starts square, rounding its
//! corners as it settles.
//!
//! Right side: sign-up link, the sign-in form (email, password, button) and
//! a footer.

mod life;

use std::{cell::Cell, rc::Rc, time::Duration};

use gpui_kit::{
    component::{
        ActiveTheme,
        button::{Button, ButtonVariants},
        input::{Input, InputState},
    },
    *,
};

use life::Life;

use crate::ui::main_content::MAIN_RADIUS;

/// Time between generations.
const TICK: Duration = Duration::from_millis(140);
/// Distance from one cell to the next.
const CELL_PITCH: Pixels = px(12.);
/// Gap between two cells, as a share of [`CELL_PITCH`].
const CELL_GAP: f32 = 0.2;
/// Share of the width for the Life panel; the form takes the rest.
const LIFE_SHARE: f32 = 0.45;
/// Glow strength of a fully opaque cell; younger cells glow less.
const GLOW_OPACITY: f32 = 0.55;
/// Corner radius, as a share of the cell side, for a newborn cell and a
/// settled one.
const CELL_ROUNDNESS: (f32, f32) = (0.06, 0.28);
/// Strength of the `primary` tint where the gradient starts.
const GRADIENT_TINT: f32 = 0.1;
/// Gradient stops, as shares of the width: solid tint up to the first, plain
/// `background` from the second.
const GRADIENT_STOPS: (f32, f32) = (0.3, 0.95);
/// Height of the inputs and the Sign in button.
const CONTROL_HEIGHT: Pixels = px(44.);
/// Corner radius of the inputs and the Sign in button.
const CONTROL_RADIUS: Pixels = px(8.);
/// Input fill: a thin layer of `foreground`, so it reads a step lighter than
/// the page in dark mode and a step darker in light mode.
const INPUT_FILL: f32 = 0.05;

pub struct AuthContainer {
    email: Entity<InputState>,
    password: Entity<InputState>,
    life: Life,
    /// Seed for the next board; bumped every time the board is rebuilt.
    seed: u32,
    /// Columns and rows that fit the canvas, written at paint time. The next
    /// tick rebuilds the board when it no longer matches.
    grid: Rc<Cell<(usize, usize)>>,
    /// A tick loop is running.
    ticking: bool,
    /// The view rendered since the last tick. When it did not (tab hidden),
    /// the loop stops and the next render starts it again.
    rendered: bool,
}

impl AuthContainer {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let email = cx.new(|cx| InputState::new(window, cx).placeholder("name@example.com"));
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter password")
                .masked(true)
        });
        Self {
            email,
            password,
            // Empty until the first paint tells us how many cells fit.
            life: Life::new(0, 0, 0),
            seed: 7,
            grid: Rc::default(),
            ticking: false,
            rendered: false,
        }
    }

    /// Steps the simulation every [`TICK`] while the container is on screen.
    fn start_ticking(&mut self, cx: &mut Context<Self>) {
        self.ticking = true;
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(TICK).await;
                let keep_going = this.update(cx, |this, cx| {
                    if !this.rendered {
                        this.ticking = false;
                        return false;
                    }
                    this.rendered = false;
                    this.tick();
                    cx.notify();
                    true
                });
                if !matches!(keep_going, Ok(true)) {
                    break;
                }
            }
        })
        .detach();
    }

    /// Rebuilds the board when the canvas changed size, else steps it.
    fn tick(&mut self) {
        let (cols, rows) = self.grid.get();
        if (cols, rows) == (self.life.cols(), self.life.rows()) {
            self.life.step();
        } else {
            self.seed = self.seed.wrapping_add(1);
            self.life = Life::new(cols, rows, self.seed);
        }
    }

    fn life_panel(&self, cx: &App) -> impl IntoElement {
        let theme = cx.theme();
        let primary = theme.primary;
        let cells = self.life.cells().to_vec();
        let cols = self.life.cols();
        let grid = self.grid.clone();

        div()
            .w(relative(LIFE_SHARE))
            .flex()
            .flex_col()
            .p_8()
            .child(brand(cx))
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, (), window, _| {
                        grid.set(fit_grid(bounds));
                        paint_cells(bounds, &cells, cols, primary, window);
                    },
                )
                .flex_1()
                .min_h_0()
                .w_full()
                .my_6(),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .max_w(px(260.))
                            .text_3xl()
                            .line_height(rems(2.1))
                            .font_weight(FontWeight::BOLD)
                            .child("Small habits. A living map."),
                    )
                    .child(
                        div()
                            .max_w(px(280.))
                            .text_sm()
                            .text_color(theme.muted_foreground)
                            .child(
                                "Each day you show up, a cell comes alive. Stay consistent \
                                 and your colony spreads.",
                            ),
                    ),
            )
    }

    fn form_panel(&self, cx: &App) -> impl IntoElement {
        let theme = cx.theme();
        let muted = theme.muted_foreground;

        div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .p_8()
            .child(
                div()
                    .flex()
                    .justify_end()
                    .items_center()
                    .gap_1()
                    .text_sm()
                    .text_color(muted)
                    .child("No account?")
                    .child(
                        Button::new("auth-sign-up")
                            .link()
                            .text_color(theme.primary)
                            .font_weight(FontWeight::SEMIBOLD)
                            .label("Sign up"),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py_10()
                    .child(
                        div()
                            .w_full()
                            .max_w(px(400.))
                            .flex()
                            .flex_col()
                            .gap_6()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.primary)
                                            .child("SIGN IN"),
                                    )
                                    .child(
                                        div()
                                            .text_3xl()
                                            .font_weight(FontWeight::BOLD)
                                            .child("Welcome back"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(muted)
                                            .child("Enter your email and password to continue."),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(field("Email", Input::new(&self.email), cx))
                                    .child(field(
                                        "Password",
                                        Input::new(&self.password).mask_toggle(),
                                        cx,
                                    )),
                            )
                            .child(
                                Button::new("auth-sign-in")
                                    .primary()
                                    .w_full()
                                    .h(CONTROL_HEIGHT)
                                    .rounded(CONTROL_RADIUS)
                                    .label("Sign in"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_end()
                    .gap_5()
                    .text_xs()
                    .text_color(muted)
                    .child("© 2026 Colony Labs")
                    .child("Privacy")
                    .child("Terms"),
            )
    }
}

/// Columns and rows of whole cells that fit `bounds`.
fn fit_grid(bounds: Bounds<Pixels>) -> (usize, usize) {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "cell counts are small and non-negative"
    )]
    let fit = |length: Pixels| (length / CELL_PITCH).floor().max(0.) as usize;
    (fit(bounds.size.width), fit(bounds.size.height))
}

/// Draws the live cells of a `cols`-wide board, centered in `bounds`.
fn paint_cells(
    bounds: Bounds<Pixels>,
    cells: &[u8],
    cols: usize,
    primary: Hsla,
    window: &mut Window,
) {
    if cols == 0 {
        return;
    }
    let rows = cells.len() / cols;
    let left = (bounds.size.width - CELL_PITCH * cols as f32) / 2.;
    let top = (bounds.size.height - CELL_PITCH * rows as f32) / 2.;
    let gap = CELL_PITCH * CELL_GAP;
    let side = CELL_PITCH - gap;
    let cell_bounds = |index: usize| {
        let (row, col) = (index / cols, index % cols);
        let origin = point(
            bounds.origin.x + left + CELL_PITCH * col as f32 + gap / 2.,
            bounds.origin.y + top + CELL_PITCH * row as f32 + gap / 2.,
        );
        Bounds::new(origin, size(side, side))
    };

    let live = || cells.iter().enumerate().filter(|(_, age)| **age > 0);
    // A cell's opacity, 40% when born up to 100% settled, also sets how
    // strong its glow is and how round its corners are.
    let look = |age: u8| {
        let opacity = life::opacity(age);
        let settled = (opacity - life::opacity(1)) / (1. - life::opacity(1));
        let (square, round) = CELL_ROUNDNESS;
        (opacity, side * (square + (round - square) * settled))
    };

    // Glow first so it sits under the cells and spills into the gaps.
    for (index, age) in live() {
        let (opacity, radius) = look(*age);
        window.paint_drop_shadows(
            cell_bounds(index),
            Corners::all(radius),
            &[BoxShadow {
                color: primary.opacity(GLOW_OPACITY * opacity),
                offset: point(px(0.), px(0.)),
                blur_radius: side * 0.9,
                spread_radius: gap,
                inset: false,
            }],
        );
    }

    // Dead cells are skipped so the gradient shows through.
    for (index, age) in live() {
        let (opacity, radius) = look(*age);
        window.paint_quad(fill(cell_bounds(index), primary.opacity(opacity)).corner_radii(radius));
    }
}

/// Label above a filled, bordered input.
fn field(label: &'static str, input: Input, cx: &App) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(label))
        .child(
            // `Input::h` only sizes multi-line inputs; the `Styled` one
            // refines the frame, so it sets the single-line height.
            Styled::h(input, CONTROL_HEIGHT)
                .rounded(CONTROL_RADIUS)
                .bg(cx.theme().foreground.opacity(INPUT_FILL))
                .px(px(14.))
                .py(px(4.)),
        )
}

/// App mark: a glider drawn with the same cells as the panel.
fn brand(cx: &App) -> impl IntoElement {
    const GLIDER: [[bool; 3]; 3] = [
        [false, true, false],
        [false, false, true],
        [true, true, true],
    ];
    let primary = cx.theme().primary;

    div()
        .flex()
        .items_center()
        .gap_2p5()
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.))
                .p(px(5.))
                .rounded(px(8.))
                .bg(primary.opacity(0.12))
                .children(GLIDER.iter().map(|row| {
                    div().flex().gap(px(2.)).children(row.iter().map(|alive| {
                        div().size(px(5.)).rounded(px(1.5)).bg(if *alive {
                            primary
                        } else {
                            primary.opacity(0.)
                        })
                    }))
                })),
        )
        .child(
            div()
                .text_base()
                .font_weight(FontWeight::SEMIBOLD)
                .child("Colony"),
        )
}

impl Render for AuthContainer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.rendered = true;
        if !self.ticking {
            self.start_ticking(cx);
        }
        let theme = cx.theme();

        div()
            .size_full()
            .min_h(px(620.))
            .flex()
            // The main area only clips to a rectangle, so the gradient
            // rounds its own corners to match it.
            .rounded_l(MAIN_RADIUS)
            // Left to right: `primary` tint, blended over the background so
            // it stays opaque, fading into the plain background.
            .bg(linear_gradient(
                90.,
                linear_color_stop(
                    theme.background.blend(theme.primary.opacity(GRADIENT_TINT)),
                    GRADIENT_STOPS.0,
                ),
                linear_color_stop(theme.background, GRADIENT_STOPS.1),
            ))
            .text_color(theme.foreground)
            .child(self.life_panel(cx))
            .child(self.form_panel(cx))
    }
}

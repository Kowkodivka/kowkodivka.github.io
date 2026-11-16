use dioxus::prelude::*;
use fastrand::Rng;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CELL: f64 = 34.0;
const POINTER_RADIUS: f64 = 190.0;
const POINTER_PUSH: f64 = 46.0;
const ACCENT: &str = "#d97757";

#[component]
pub fn GridCanvas(pointer: Signal<Option<(f64, f64)>>) -> Element {
    rsx! {
        canvas {
            class: "pointer-events-none absolute inset-0 block size-full text-fg",
            aria_hidden: "true",
            onmounted: move |event| {
                if let Some(element) = event.data().downcast::<web_sys::Element>() {
                    let canvas: HtmlCanvasElement = element.clone().unchecked_into();
                    spawn(animate(canvas, pointer));
                }
            },
        }
    }
}

async fn animate(canvas: HtmlCanvasElement, pointer: Signal<Option<(f64, f64)>>) {
    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .expect("canvas 2d")
        .unchecked_into();

    let still = prefers_reduced_motion();
    let start = js_sys::Date::now();
    let mut grid = Grid::new(0.0, 0.0);
    let mut cursor: Option<(f64, f64)> = None;

    loop {
        let width = canvas.client_width() as f64;
        let height = canvas.client_height() as f64;

        if width != grid.width || height != grid.height {
            grid = Grid::new(width, height);
            fit_to_screen(&canvas, &ctx, width, height);
        }

        let time = if still {
            0.0
        } else {
            (js_sys::Date::now() - start) / 1000.0
        };
        cursor = follow(cursor, to_canvas_coords(&canvas, *pointer.peek()));

        grid.draw(&ctx, time, cursor, &text_color(&canvas));
        next_frame().await;
    }
}

struct Node {
    x: f64,
    y: f64,
    chaos: f64,
    shift_x: f64,
    shift_y: f64,
    phase: f64,
    broken_right: bool,
    broken_down: bool,
    square: f64,
    dust: u32,
    dust_x: f64,
    dust_y: f64,
}

struct Grid {
    width: f64,
    height: f64,
    cols: usize,
    rows: usize,
    nodes: Vec<Node>,
    accent: Option<usize>,
}

impl Grid {
    fn new(width: f64, height: f64) -> Self {
        let cols = (width / CELL).ceil() as usize + 2;
        let rows = (height / CELL).ceil() as usize + 2;
        let mut random = Rng::with_seed(67);
        let mut nodes = Vec::with_capacity(cols * rows);

        for row in 0..rows {
            for col in 0..cols {
                let x = (col as f64 - 1.0) * CELL;
                let y = (row as f64 - 1.0) * CELL;
                let chaos = ((x - width * 0.38) / (width * 0.62)).clamp(0.0, 1.0);
                let break_chance = chaos.powf(1.5) * 0.5;

                nodes.push(Node {
                    x,
                    y,
                    chaos,
                    shift_x: between(&mut random, -1.0, 1.0),
                    shift_y: between(&mut random, -1.0, 1.0),
                    phase: between(&mut random, 0.0, std::f64::consts::TAU),
                    broken_right: random.f64() < break_chance,
                    broken_down: random.f64() < break_chance,
                    square: if chaos > 0.5 && random.f64() < 0.03 * chaos {
                        between(&mut random, 4.0, 16.0)
                    } else {
                        0.0
                    },
                    dust: if chaos > 0.55 {
                        (random.f64() * 3.0 * chaos) as u32
                    } else {
                        0
                    },
                    dust_x: between(&mut random, -1.0, 1.0),
                    dust_y: between(&mut random, -1.0, 1.0),
                });
            }
        }

        let accent = nodes.iter().position(|n| {
            (0.62..0.72).contains(&n.chaos) && n.y > height * 0.18 && n.y < height * 0.3
        });

        Grid {
            width,
            height,
            cols,
            rows,
            nodes,
            accent,
        }
    }

    fn position(&self, node: &Node, time: f64, cursor: Option<(f64, f64)>) -> (f64, f64) {
        let c = node.chaos;
        let c2 = c * c;

        let mut x = node.x + node.shift_x * c2 * 26.0 + (time * 0.5 + node.phase).sin() * c2 * 6.0;
        let mut y = node.y
            + node.shift_y * c2 * 26.0
            + (node.x * 0.012 + time * 0.35).sin() * c * 14.0
            + (time * 0.4 + node.phase).cos() * c2 * 5.0;

        if let Some((px, py)) = cursor {
            let (dx, dy) = (x - px, y - py);
            let distance = dx.hypot(dy);

            if distance > 0.01 && distance < POINTER_RADIUS {
                let force = (1.0 - distance / POINTER_RADIUS).powi(2) * POINTER_PUSH;
                x += dx / distance * force;
                y += dy / distance * force;
            }
        }

        (x, y)
    }

    fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        time: f64,
        cursor: Option<(f64, f64)>,
        color: &str,
    ) {
        ctx.clear_rect(0.0, 0.0, self.width, self.height);

        let points: Vec<(f64, f64)> = self
            .nodes
            .iter()
            .map(|n| self.position(n, time, cursor))
            .collect();

        ctx.set_stroke_style_str(color);
        ctx.set_line_width(0.7);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let i = row * self.cols + col;
                let node = &self.nodes[i];

                ctx.set_global_alpha(0.13 + node.chaos * 0.16);

                if col + 1 < self.cols && !node.broken_right {
                    line(ctx, points[i], points[i + 1]);
                }

                if row + 1 < self.rows && !node.broken_down {
                    line(ctx, points[i], points[i + self.cols]);
                }
            }
        }

        ctx.set_global_alpha(0.92);
        ctx.set_fill_style_str(color);

        for (node, &(x, y)) in self.nodes.iter().zip(&points) {
            if node.square > 0.0 {
                ctx.fill_rect(
                    x - node.square / 2.0,
                    y - node.square / 2.0,
                    node.square,
                    node.square,
                );
            }
        }

        ctx.set_global_alpha(0.45);
        for (node, &(x, y)) in self.nodes.iter().zip(&points) {
            for step in 1..=node.dust {
                let step = step as f64;
                let dx = node.dust_x * 14.0 * step + (time + step + node.phase).sin() * 2.0;
                let dy = node.dust_y * 10.0 * step;
                ctx.fill_rect(x + dx, y + dy, 1.6, 1.6);
            }
        }

        if let Some(i) = self.accent {
            let (x, y) = points[i];

            ctx.set_global_alpha(1.0);
            ctx.set_fill_style_str(ACCENT);
            ctx.fill_rect(x - 11.0, y - 11.0, 22.0, 22.0);
        }

        if let Some(point) = cursor {
            draw_crosshair(ctx, point, color);
        }

        ctx.set_global_alpha(1.0);
    }
}

fn draw_crosshair(ctx: &CanvasRenderingContext2d, (x, y): (f64, f64), color: &str) {
    let r = 14.0;

    ctx.set_global_alpha(0.9);
    ctx.set_stroke_style_str(ACCENT);
    ctx.set_line_width(1.0);
    ctx.begin_path();
    let _ = ctx.arc(x, y, r, 0.0, std::f64::consts::TAU);
    ctx.stroke();
    line(ctx, (x - r - 10.0, y), (x - r + 4.0, y));
    line(ctx, (x + r - 4.0, y), (x + r + 10.0, y));
    line(ctx, (x, y - r - 10.0), (x, y - r + 4.0));
    line(ctx, (x, y + r - 4.0), (x, y + r + 10.0));

    ctx.set_global_alpha(0.7);
    ctx.set_fill_style_str(color);
    ctx.set_font("500 10px \"IBM Plex Mono\", monospace");
    let _ = ctx.fill_text(&format!("X {x:.0}  Y {y:.0}"), x + r + 14.0, y - r - 4.0);
}

fn line(ctx: &CanvasRenderingContext2d, from: (f64, f64), to: (f64, f64)) {
    ctx.begin_path();
    ctx.move_to(from.0, from.1);
    ctx.line_to(to.0, to.1);
    ctx.stroke();
}

fn follow(current: Option<(f64, f64)>, target: Option<(f64, f64)>) -> Option<(f64, f64)> {
    match (current, target) {
        (Some((x, y)), Some((tx, ty))) => Some((x + (tx - x) * 0.12, y + (ty - y) * 0.12)),
        (_, target) => target,
    }
}

fn to_canvas_coords(canvas: &HtmlCanvasElement, pointer: Option<(f64, f64)>) -> Option<(f64, f64)> {
    let (x, y) = pointer?;
    let rect = canvas.get_bounding_client_rect();

    Some((x - rect.left(), y - rect.top()))
}

fn fit_to_screen(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
) {
    let ratio = window().device_pixel_ratio().min(2.0);

    canvas.set_width((width * ratio) as u32);
    canvas.set_height((height * ratio) as u32);

    let _ = ctx.set_transform(ratio, 0.0, 0.0, ratio, 0.0, 0.0);
}

fn text_color(canvas: &HtmlCanvasElement) -> String {
    window()
        .get_computed_style(canvas)
        .ok()
        .flatten()
        .and_then(|style| style.get_property_value("color").ok())
        .unwrap_or_else(|| "#141413".into())
}

fn prefers_reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches())
}

async fn next_frame() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let _ = window().request_animation_frame(&resolve);
    });

    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn window() -> web_sys::Window {
    web_sys::window().expect("window")
}

fn between(random: &mut Rng, min: f64, max: f64) -> f64 {
    min + random.f64() * (max - min)
}

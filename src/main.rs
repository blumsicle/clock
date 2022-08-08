use chrono::{Local, Timelike};
use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const HOURS_ON_CLOCK: f32 = 12.0;
const MINUTES_ON_CLOCK: f32 = 60.0;
const SECONDS_ON_CLOCK: f32 = 60.0;

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.5, 0.4, 0.4));

    let now = Local::now();
    let bounds = app.window_rect().pad(25.0);
    let radius = f32::min(bounds.right(), bounds.top());

    // NOTE: Draw inner and outer circle
    draw.ellipse()
        .no_fill()
        .stroke(WHITE)
        .stroke_weight(3.0)
        .radius(radius);

    draw.ellipse()
        .no_fill()
        .stroke(WHITE)
        .stroke_weight(3.0)
        .radius(radius / 30.0);

    // NOTE: Draw tick marks
    let step = TAU / (MINUTES_ON_CLOCK);
    let mut longer = 0;
    let mut angle = 0.0;
    while angle < (TAU - step) {
        let vector = vec2(angle.sin(), angle.cos());
        let start = vector * (radius - 1.0);
        let end = if longer % 5 == 0 {
            vector * (radius - 20.0)
        } else {
            vector * (radius - 10.0)
        };

        draw.line().points(start, end).color(RED).weight(5.0);

        angle += step;
        longer += 1;
    }

    // NOTE: Draw seconds hand
    let seconds = now.second() as f32;
    let subseconds = now.nanosecond() as f32 / 1_000_000_000.0;
    let angle = (seconds / SECONDS_ON_CLOCK) * TAU + (subseconds * (TAU / SECONDS_ON_CLOCK));
    let end = vec2(angle.sin(), angle.cos()) * (radius - 1.0);

    draw.line().points(Vec2::ZERO, end).color(BLUE).weight(3.0);

    // NOTE: Draw minutes hand
    let minutes = now.minute() as f32;
    let subminutes = seconds / SECONDS_ON_CLOCK;
    let angle = (minutes / MINUTES_ON_CLOCK) * TAU + (subminutes * (TAU / MINUTES_ON_CLOCK));
    let end = vec2(angle.sin(), angle.cos()) * radius * 0.75;

    draw.line().points(Vec2::ZERO, end).color(RED).weight(3.0);

    // NOTE: Draw hours hand
    let hours = now.hour() as f32;
    let subhours = minutes / MINUTES_ON_CLOCK;
    let angle = (hours / HOURS_ON_CLOCK) * TAU + (subhours * (TAU / HOURS_ON_CLOCK));
    let end = vec2(angle.sin(), angle.cos()) * radius * 0.5;

    draw.line().points(Vec2::ZERO, end).color(GREEN).weight(3.0);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).run();
}

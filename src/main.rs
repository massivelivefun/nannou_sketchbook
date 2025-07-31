use nannou::prelude::*;
use nannou::color::rgb::Srgb;
use nannou::rand::{rand, Rng};
use nannou::rand::rngs::ThreadRng;
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    colors: Vec<Srgb<u8>>,
    sides: usize,
    steps: usize,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut rng = rand::thread_rng();
    let sides = 6;
    let pool: Vec<Srgb<u8>> = vec![STEELBLUE, PINK, RED, GREEN, ORANGE, PURPLE, WHITE];
    let mut colors: Vec<Srgb<u8>> = Vec::with_capacity(sides);
    for _ in 0..sides {
        let index = rng.gen_range(0..pool.len());
        colors.push(pool[index]);
    }
    let steps = 360 / sides;
    Model { _window, colors, sides, steps }
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    let time = app.time;
    draw.background().color(PLUM);
    
    let radius = 150.0;
    let points = (0..360).step_by(model.steps).enumerate().map(|(idx, i)| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        (pt2(x,y), model.colors[idx])
    });
    let rate = 25.0 * time;
    draw.polygon()
        .z_degrees(rate)
        .points_colored(points);
    draw.to_frame(app, &frame).unwrap();
}

use nannou::prelude::*;
use nannou::rand::{rand, Rng};

pub fn polygon_main() {
    nannou::app(polygon_model).update(polygon_update).run();
}

pub struct PolygonModel {
    _window: window::Id,
    colors: Vec<Srgb<u8>>,
    sides: usize,
    steps: usize,
}

pub fn polygon_model(app: &App) -> PolygonModel {
    let _window = app.new_window().view(polygon_view).build().unwrap();
    let mut rng = rand::thread_rng();
    let sides = 6;
    let pool: Vec<Srgb<u8>> = vec![STEELBLUE, PINK, RED, GREEN, ORANGE, PURPLE, WHITE];
    let mut colors: Vec<Srgb<u8>> = Vec::with_capacity(sides);
    for _ in 0..sides {
        let index = rng.gen_range(0..pool.len());
        colors.push(pool[index]);
    }
    let steps = 360 / sides;
    PolygonModel { _window, colors, sides, steps }
}

pub fn polygon_view(app: &App, model: &PolygonModel, frame: Frame) {
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

pub fn polygon_update(_app: &App, _model: &mut PolygonModel, _update: Update) {}

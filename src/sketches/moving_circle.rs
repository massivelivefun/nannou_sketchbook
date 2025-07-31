use nannou::prelude::*;

pub fn moving_circle_main() {
    nannou::app(moving_circle_model).update(moving_circle_update).run();
}

struct MovingCircleModel {
    pub _window: window::Id,
}

fn moving_circle_model(app: &App) -> MovingCircleModel {
    let _window = app.new_window().view(moving_circle_view).build().unwrap();
    MovingCircleModel { _window }
}

fn moving_circle_view(app: &App, _model: &MovingCircleModel, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let sine = app.time.sin();
    let slowersine = (app.time/ 2.0).sin();
    let boundary = app.window_rect();

    let x = map_range(slowersine, -1.25, 1.25, boundary.left(), boundary.right());
    let y = map_range(sine, -1.25, 1.25, boundary.bottom(), boundary.top());
    
    draw.ellipse().color(STEELBLUE).x_y(0.0, 0.0).x_y(x, y).z_degrees(90.0);

    draw.to_frame(app, &frame).unwrap();
}

fn moving_circle_update(_app: &App, _model: &mut MovingCircleModel, _update: Update) {}

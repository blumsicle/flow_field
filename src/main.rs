use chrono::Utc;
use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

struct Model {
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title("nannou_template")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let seed = Utc::now().timestamp() as u32;
    let noise = Perlin::new().set_seed(seed);

    Model { noise }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.5, 0.4, 0.4));

    let bounds = app.window_rect();

    let points = (bounds.left() as isize..bounds.right() as isize).map(|i| {
        let x = i as f64;
        let scale = 0.01;
        let time = app.elapsed_frames() as f64 * scale;
        let xoff = x * scale + time;

        let nf = 50.0;
        let n = map_range(model.noise.get([xoff, 0.0]), -1.0, 1.0, -nf, nf);
        let s = map_range(
            xoff.sin(),
            -1.0,
            1.0,
            bounds.bottom() + nf,
            bounds.top() - nf,
        );

        let y = n + s;
        pt2(x as f32, y as f32)
    });

    draw.polyline()
        .weight(2.0)
        .color(hsv(0.0, 0.9, 0.2))
        .points(points);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}

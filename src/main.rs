use chrono::Utc;
use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

mod particle;
use particle::Particle;

struct Model {
    noise: Perlin,
    particles: Vec<Particle>,
    flowfield: Vec<Vec2>,
    tilesize: f32,
    rows: usize,
    columns: usize,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title("nannou_template")
        .size(1280, 720)
        .view(view)
        .build()
        .unwrap();

    let seed = Utc::now().timestamp() as u32;
    let noise = Perlin::new().set_seed(seed);

    let bounds = app.window_rect();
    let mut particles = Vec::<Particle>::new();

    for _ in 0..10000 {
        let mut position = random::<Vec2>() - 0.5;
        position.x *= bounds.w();
        position.y *= bounds.h();

        particles.push(Particle {
            position,
            prev_position: position,
            topspeed: 4.0,
            ..Default::default()
        });
    }

    let tilesize = 20.0;
    let rows = (bounds.h() / tilesize) as usize;
    let columns = (bounds.w() / tilesize) as usize;

    Model {
        noise,
        particles,
        flowfield: Vec::new(),
        tilesize,
        rows,
        columns,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let scale = 0.01;
    let zoff = app.elapsed_frames() as f64 * 0.005;
    model.flowfield.clear();
    for y in 0..=model.rows {
        for x in 0..model.columns {
            let value = model.noise.get([x as f64 * scale, y as f64 * scale, zoff]);
            // NOTE: Multiplying PI by 4 simply because the noise results tend to center around 0.0.
            // This just makes the flowfield angles go in all 360 degrees.
            let angle = map_range(value, -1.0, 1.0, -PI * 4.0, PI * 4.0);
            model.flowfield.push(vec2(angle.cos(), angle.sin()));
        }
    }

    let bounds = app.window_rect();
    for particle in &mut model.particles {
        let mut x = ((bounds.right() + particle.position.x) / model.tilesize).trunc() as usize;
        let mut y = ((bounds.top() + particle.position.y) / model.tilesize).trunc() as usize;

        // NOTE: This clamp is necessary because of float rounding errors I believe
        x = x.clamp(0, model.columns - 1);
        y = y.clamp(0, model.rows - 1);

        let index = y * model.columns + x;

        let force = model.flowfield[index] * 2.0;
        particle.apply_force(force);
        particle.update();
        particle.check_edges(bounds);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(hsv(0.0, 0.0, 0.01));
    } else {
        let bounds = app.window_rect();
        draw.rect()
            .wh(bounds.wh())
            .color(hsva(0.0, 0.0, 0.01, 0.05));
    }

    // let bounds = app.window_rect();
    // for y in 0..model.rows {
    //     for x in 0..model.columns {
    //         let index = y * model.columns + x;
    //         let direction = model.flowfield[index];

    //         let x = bounds.left() + (x as f32 * model.tilesize) + (model.tilesize * 0.5);
    //         let y = bounds.bottom() + (y as f32 * model.tilesize) + (model.tilesize * 0.5);
    //         draw.line()
    //             .points(Vec2::ZERO, direction * model.tilesize * 0.75)
    //             .color(rgba(0.0, 0.0, 0.0, 0.3))
    //             .weight(2.0)
    //             .x_y(x, y);
    //     }
    // }

    for particle in &model.particles {
        particle.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

use chrono::Utc;
use nannou::{
    image::{DynamicImage, Rgba},
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
    wgpu::Texture,
};

struct Model {
    noise: Perlin,
    image: DynamicImage,
    texture: Option<Texture>,
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
    let image = DynamicImage::new_rgba8(400, 400);

    Model {
        noise,
        image,
        texture: None,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let pixels = model.image.as_mut_rgba8();
    let scale = 0.015;
    let zoff = (app.elapsed_frames() as f64 * scale).sin();

    if let Some(pixels) = pixels {
        for y in 0..pixels.height() {
            for x in 0..pixels.width() {
                let value = model.noise.get([x as f64 * scale, y as f64 * scale, zoff]);
                let value = map_range(value, -1.0, 1.0, 0.0, 255.0) as u8;
                pixels.put_pixel(x, y, Rgba([value, value, value, 255]));
            }
        }
    }

    // TODO: Is there a way to just update the texture with the updated pixels as opposed to create
    // a new texture every update?
    model.texture = Some(Texture::from_image(app, &model.image));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.5, 0.4, 0.4));

    let bounds = app.window_rect();

    if let Some(texture) = model.texture.as_ref() {
        draw.texture(texture).wh(bounds.wh());
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

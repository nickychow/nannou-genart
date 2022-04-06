use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    // We just return an empty struct here
    Model {}
}

// Updates the model (note the mutable reference)
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Draws on the screen
fn view(app: &App, _model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    draw.background().color(BLACK);

    let noise = Perlin::new();
    let w = app.window_rect();
    let time = app.elapsed_frames() as f64 / 100.;

    let l1 = noise.get([time, 1.23]) as f32;
    let o1 = noise.get([1.0, time]) as f32;
    let l2 = noise.get([time, -0.23]) as f32;
    let o2 = noise.get([0.45, time * 0.7]) as f32;

    for x in (w.left() * 10.0) as i32..(w.right() * 10.0) as i32 {
        let t1 = PI * (x - 1) as f32 / 100.;
        let t2 = PI * x as f32 / 100.;
        let p1 = ((t1 * 0.038 + l1).sin() + o1) * 100.;
        let p2 = ((t1 * 0.074 + l2 + time as f32).sin() + o2) * 100.;
        let p3 = (t1 * 5.0).sin() * (p2 - p1) / 2.0 + (p1 + p2) / 2.;

        let f1 = ((t2 * 0.038 + l1).sin() + o1) * 100.;
        let f2 = ((t2 * 0.074 + l2 + time as f32).sin() + o2) * 100.;
        let f3 = (t2 * 5.0).sin() * (f2 - f1) / 2.0 + (f1 + f2) / 2.;

        draw.line()
            .start(vec2((x - 1) as f32 / 10.0, p3))
            .end(vec2(x as f32 / 10.0, f3))
            .color(hsla(
                x as f32 / 5000.0 + time as f32,
                1.,
                0.5,
                ((p3 - f3) / 20.0).abs(),
            ))
            .stroke_weight(2.0);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

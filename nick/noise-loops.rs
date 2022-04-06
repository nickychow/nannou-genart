use nannou::noise::{NoiseFn, OpenSimplex};
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

    // app.set_loop_mode(LoopMode::Wait);

    let time = (app.elapsed_frames() as f32) / 30.0;

    // Let's first color the background
    draw.background().color(BLACK);

    let noise = OpenSimplex::new();

    let noise_strength = 1.5;

    let points = (0..=360).step_by(10).map(|i| {
        // Convert each degree to radians.
        let radian = deg_to_rad(i as f32);

        let radang = radian + time;

        let xoff = map_range(radang.cos(), -1.0, 1.0, 0.0, noise_strength) as f64;
        let yoff = map_range(radang.sin(), -1.0, 1.0, 0.0, noise_strength) as f64;
        let noise_value = noise.get([xoff, yoff, time as f64]) as f32;

        let radius = map_range(noise_value, 0.0, 1.0, 100.0, 200.0);
        // Get the sine of the radian to find the x co-ordinate of this point of the circle
        // and multiply it by the radius.
        let x = radian.cos() * radius;
        // Do the same with cosine to find the y co-ordinate.
        let y = radian.sin() * radius;

        // yoff += 0.1;

        // Construct and return a point object with a color.
        pt2(x, y)
    });

    // draw.polyline().weight(3.0).points_colored(points);
    draw.polygon()
        .stroke(BLUE)
        .stroke_weight(2.0)
        .no_fill()
        .points(points);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

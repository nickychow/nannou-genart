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
    let win = app.window_rect();

    // Let's first color the background
    draw.background().color(WHITE);

    let noise = OpenSimplex::new();

    let noise_x_range = map_range(app.mouse.x, win.left(), win.right(), 0.0, win.w()) / 10.0;

    let range = win.w() as usize / 10;
    let vertices = (0..=range).map(|x| {
        let noise_x = map_range(x as f32, 0.0, win.w(), 0.0, noise_x_range) as f64;
        let y = noise.get([noise_x, noise_x]) * win.h() as f64 / 2.0;
        pt2(win.left() + (x as f32 * 10.0), y as f32)
    });

    draw.polyline()
        .weight(1.0)
        .points(vertices)
        .rgb(0.0, 0.5, 0.64);

    // for x in (0..win.w() as usize).step_by(10) {
    //     let noise_x = map_range(x as f32 / 10.0, 0.0, win.w(), 0.0, noise_x_range) as f64;
    //     let y = noise.get([noise_x, noise_x]) * win.h() as f64 / 2.0;
    //     draw.ellipse()
    //         .x_y(win.left() + x as f32, y as f32)
    //         .w_h(5.0, 5.0)
    //         .color(BLACK);
    // }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

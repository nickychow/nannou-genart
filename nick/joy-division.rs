use nannou::prelude::*;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(1024, 1024).build().unwrap();

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

    // let y_spacing = 1024.0 / (SPEC_BANDS as f32 + 3.0);
    // let mut y_ofs = 512.0 - (y_spacing * 2.0); // Some headroom at the top...
    // let x_start = -512.0 * 0.75;
    // let x_end = -x_start;

    // for band in (1..SPEC_BANDS as usize).rev() {
    //     y_ofs -= y_spacing;
    //     let mut points = Vec::new();
    //     // let start = model.third_octave_lookups[band].0.trunc() as i32;
    //     // let end = model.third_octave_lookups[band].1.trunc() as i32;
    //     let num_samples = end - start;
    //     let num_points = num_samples + 2;
    //     let x_spacing = (x_end - x_start) / num_points as f32;
    //     points.push((pt2(x_start + 0.0, y_ofs + 0.0), WHITE));
    // }

    let points = (0..50).map(|i| {
        let x = i as f32 * 10.0 - 250.0;
        let y = x.sin() * 20.0;
        (pt2(x, y), STEELBLUE)
    });

    draw.polyline().weight(2.0).points_colored(points);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

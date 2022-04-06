use nannou::prelude::*;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    y_positions: Vec<f32>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    // We just return an empty struct here
    Model {
        y_positions: Vec::new(),
    }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    model.y_positions.push(app.mouse.y);
    if model.y_positions.len() > 60 {
        model.y_positions.remove(0);
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    let window = app.window_rect();
    app.main_window().set_title("`LoopMode` Demonstration");

    // Start in `Wait` mode. In other words, don't keep looping, just wait for events.
    app.set_loop_mode(LoopMode::rate_fps(60.0));

    // Let's first color the background
    draw.background().color(BLUE);

    if model.y_positions.len() > 1 {
        let half_thickness = 1.5;

        let vertices = (0..model.y_positions.len())
            .map(|i| pt2(((i as f32) * 5.0) - window.w() / 2.0, model.y_positions[i]));

        draw.polyline()
            .stroke_weight(half_thickness)
            .color(rgba(1.0, 0.0, 0.0, 1.0))
            .points(vertices);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

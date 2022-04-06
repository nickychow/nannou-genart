use nannou::prelude::*;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    p: Vec2,
    s: Vec2,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    // We just return an empty struct here
    Model {
        p: vec2(0.0, 0.0),
        s: vec2(0.0, 0.0),
    }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let t = (app.elapsed_frames() as f32) * 0.03;
    let w = (t * 0.832).sin() * 90.0 + 100.0;
    let h = (t * 0.734).cos() * 90.0 + 100.0;
    let x = (t * 0.132).cos() * 200.0;
    let y = (t * 0.176).sin() * 200.0;

    model.p = vec2(x, y);
    model.s = vec2(w, h);
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    // draw.background().color(BLACK);
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(720.0, 720.0)
        .color(hsla(0.0, 0.0, 0.0, 0.005));

    draw.ellipse()
        .xy(model.p)
        .wh(model.s)
        .color(hsla(0.5, 1.0, 0.2, 0.01))
        .stroke(hsla(0.3, 1.0, 0.5, 0.1))
        .stroke_weight(1.0);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

use nannou::prelude::*;

const X: f32 = 0.0;
const Y: f32 = 0.0;
const X_SPEED: f32 = 2.5;
const Y_SPEED: f32 = 5.0;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    location: Vec2,
    velocity: Vec2,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    // We just return an empty struct here
    Model {
        location: pt2(X, Y),
        velocity: vec2(X_SPEED, Y_SPEED),
    }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.location += model.velocity;

    if model.location.x > 350.0 || model.location.x < -350.0 {
        model.velocity.x *= -1.0;
    }
    if model.location.y > 350.0 || model.location.y < -350.0 {
        model.velocity.y *= -1.0;
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    draw.background().color(WHITE);
    // then draw a 42 x 42 (pixels) orange rectangle
    // at the center of the screen (0, 0)
    draw.ellipse()
        .xy(model.location)
        .w_h(20.0, 20.0)
        .color(ORANGE);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

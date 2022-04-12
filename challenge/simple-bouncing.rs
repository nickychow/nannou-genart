use nannou::prelude::*;

const X: f32 = 0.0;
const Y: f32 = 0.0;
const X_SPEED: f32 = 1.0;
const Y_SPEED: f32 = 3.3;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    // We just return an empty struct here
    Model {
        x: X,
        y: Y,
        x_speed: X_SPEED,
        y_speed: Y_SPEED,
    }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.x += model.x_speed;
    model.y += model.y_speed;

    if model.x > 350.0 || model.x < -350.0 {
        model.x_speed *= -1.0;
    }
    if model.y > 350.0 || model.y < -350.0 {
        model.y_speed *= -1.0;
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
        .x_y(model.x, model.y)
        .w_h(20.0, 20.0)
        .color(ORANGE);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

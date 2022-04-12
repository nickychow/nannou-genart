use nannou::prelude::*;

const X: f32 = 100.0;
const Y: f32 = 200.0;
const X_SPEED: f32 = 0.05;
const Y_SPEED: f32 = 0.01;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
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
        acceleration: vec2(0.0, 0.0),
    }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = app.mouse.position();
    let dir = mouse - model.location;
    let dir = dir.normalize() * 0.5;
    model.acceleration = dir;
    // let
    // model.acceleration = pt2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));

    model.velocity += model.acceleration;
    model.velocity.clamp_length_max(0.01);
    // println!("{:?}", &model.velocity);

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
    draw.background().color(SNOW);
    // if app.elapsed_frames() == 1 {
    //     draw.background().color(SNOW);
    // }
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

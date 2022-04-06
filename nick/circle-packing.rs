use nannou::prelude::*;

// const LINE_WIDTH: f32 = 2.0;
const MIN_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 50.0;
const N_CIRCLES: usize = 10;
const CREATE_CIRCLE_ATTEMPTS: usize = 10;

struct Circle {
    coordinates: Point2,
    radius: f32,
    color: Rgba,
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    circles: Vec<Circle>,
}

// Builds the model
fn model(app: &App) -> Model {
    let width = 720;
    let height = 720;

    let half_w = width as f32 * 0.5;
    let half_h = height as f32 * 0.5;

    app.new_window().size(width, height).build().unwrap();

    let mut circles = Vec::<Circle>::with_capacity(N_CIRCLES);

    for _ in 0..=N_CIRCLES {
        for _attempt in 0..=CREATE_CIRCLE_ATTEMPTS {
            let x = random_range(-half_w, half_w);
            let y = random_range(-half_h, half_h);
            let radius = random_range(MIN_RADIUS, MAX_RADIUS);
            let coordinates = pt2(x, y);
            let color = rgba(random_f32(), random_f32(), random_f32(), random_f32());

            circles.push(Circle {
                coordinates,
                radius,
                color,
            });
        }
    }

    // We just return an empty struct here
    Model { circles }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    draw.background().color(SNOW);

    for circle in &model.circles {
        draw.ellipse()
            .xy(circle.coordinates)
            .radius(circle.radius)
            .color(circle.color);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

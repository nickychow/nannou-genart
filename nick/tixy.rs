use nannou::prelude::*;

const ROWS: u32 = 16;
const COLS: u32 = 16;
const SIZE: f32 = 16.0;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE as u32 + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE as u32 + 2 * MARGIN;

struct Dot {
    location: Vec2,
    size: f32,
    color: Rgb,
}

impl Dot {
    fn new(location: Vec2) -> Self {
        let size = 16.0;
        let color = Rgb::new(1.0, 1.0, 1.0);
        Dot {
            location,
            size,
            color,
        }
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    dots: Vec<Dot>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let mut dots = Vec::new();

    for y in 0..ROWS {
        for x in 0..COLS {
            let location = pt2(x as f32, y as f32);
            let dot = Dot::new(location);
            dots.push(dot);
        }
    }
    // We just return an empty struct here
    Model { dots }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0, ROWS as f32 / -2.0);

    // Let's first color the background

    // then draw a 42 x 42 (pixels) orange rectangle
    // at the center of the screen (0, 0)

    for dot in &model.dots {
        // let cdraw = gdraw.xy(dot.location);
        // cdraw
        //     .ellipse()
        //     .xy(dot.location)
        //     .radius(0.5)
        //     .color(dot.color);
        gdraw
            .ellipse()
            .xy(dot.location)
            .radius(0.5)
            .color(dot.color);
    }
    draw.ellipse().x_y(0.0, 0.0).w_h(16.0, 16.0).color(ORANGE);

    draw.ellipse().x_y(0.0, 0.0).w_h(16.0, 16.0).color(ORANGE);

    draw.background().color(BLACK);
    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

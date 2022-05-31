use nannou::prelude::*;

const DT: f32 = 0.1;
const STEPS: f32 = 500.0;
const NUMBER_PER_PATH: usize = 40;
const PATHS: usize = 300;
const MAX_PARTICLE_SIZE: f32 = 1.0;
// The total number of particles will be PATHS * NUMBER_PER_PATH

fn ease1(p: f32) -> f32 {
    3.0 * p.powi(2) - 2.0 * p.powi(3)
}

fn ease2(p: f32, g: f32) -> f32 {
    if p < 0.5 {
        0.5 * (2.0 * (1.0 - p).powf(g))
    } else {
        1.0 - 0.5 * (2.0 * (p - 1.0).powf(g))
    }
}

fn mn() -> f32 {
    0.5 * (3.0).sqrt()
}

fn ia() -> f32 {
    0.5.sqrt().atan()
}

fn field(pos: Vec2) -> Vec2 {
    pt2(0.0, 15.0)
}

// struct Particle {
//     position: Vec2,
//     size: f32,
// }

struct Path {
    positions: Vec<Vec2>,
    // particles: Vec<Particle>,
    numbers: usize,
    offset: f32,
}

impl Path {
    fn new(x: f32, y: f32, numbers: usize, offset: f32) -> Self {
        Path {
            position: vec2(x, y),
            particles: Vec::new(),
            numbers,
            offset,
        }
    }

    fn update(&mut self) {
        self.position += direction;
        for p in &mut self.particles {
            p.position += direction;
        }
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    paths: Vec<Path>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let paths = Vec::new();
    // We just return an empty struct here
    Model { paths }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Draws on the screen
fn view(app: &App, _model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    draw.background().color(BLACK);
    // then draw a 42 x 42 (pixels) orange rectangle
    // at the center of the screen (0, 0)
    draw.rect().x_y(0.0, 0.0).w_h(42.0, 42.0).color(ORANGE);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

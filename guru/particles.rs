use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

struct Particle {
    position: Point2,
    velocity: Vec2,
    // acceleration: Vector2,
    // color: Rgb,
    // radius: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        Particle {
            position: pt2(x, y),
            velocity: vec2(0.0, 0.0),
        }
    }

    fn update(&mut self, direction: Vec2) {
        self.position += self.velocity;
        self.velocity += direction;
        self.velocity *= 0.8;
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    particles: Vec<Particle>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let r = app.window_rect().right() as f32;
    let l = app.window_rect().left() as f32;

    let w = l - r;
    let t = app.window_rect().top() as f32;
    let b = app.window_rect().bottom() as f32;

    let h = t - b;

    let mut particles = Vec::new();
    for _i in 0..2000 {
        let x = random_f32() * w + r;
        let y = random_f32() * h + b;
        particles.push(Particle::new(x, y));
    }

    // We just return an empty struct here
    Model { particles }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new();
    let time = app.elapsed_frames() as f64 / 100.;
    for i in 0..model.particles.len() {
        let p = &mut model.particles[i];
        let x = noise.get([
            p.position.x as f64 / 128.,
            p.position.y as f64 / 137.,
            time + i as f64 / 1000.,
        ]);
        let y = noise.get([
            -p.position.y as f64 / 128.,
            p.position.x as f64 / 137.,
            time + i as f64 / 1000.,
        ]);

        let a = pt2(x as f32, y as f32);
        p.update(a);
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    // draw.background().color(WHITE);

    for p in &model.particles {
        draw.ellipse()
            .xy(p.position)
            .w_h(1.0, 1.0)
            .color(hsla(0.1, 1., 5., 0.01));
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

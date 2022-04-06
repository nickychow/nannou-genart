use nannou::noise::*;
use nannou::prelude::*;

const N_THINGS: usize = 2000;

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        let positions = vec![p];
        Thing { positions }
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    things: Vec<Thing>,
    noise: Perlin,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(1024, 1024).build().unwrap();

    let mut things = Vec::new();

    for _ in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0,
        ));

        things.push(thing);
    }

    let noise = Perlin::new();

    // We just return an empty struct here
    Model { things, noise }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 120.0;
    let sn = 0.01 + time.cos() as f64 * 0.005;

    for thing in &mut model.things {
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0,
        ));

        for _ in 0..50 {
            let last = thing.positions[0];
            let new = last
                + vec2(
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 0.0])
                        as f32,
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 1.0])
                        as f32,
                );
            thing.positions.insert(0, new);
        }
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    // draw.background().color(BLACK);
    // then draw a 42 x 42 (pixels) orange rectangle
    // at the center of the screen (0, 0)
    draw.rect()
        .w_h(1024.0, 1024.0)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for thing in model.things.iter() {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(WHITE);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

use nannou::prelude::*;

struct Firefly {
    position: Vec2,
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    fireflies: Vec<Firefly>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let win = app.window_rect();

    let mut fireflies = Vec::new();

    (0..80).for_each(|_| {
        let x = random_range(win.left(), win.right());
        let y = random_range(win.top(), win.bottom());
        fireflies.push(Firefly {
            position: vec2(x, y),
        });
    });

    // We just return an empty struct here
    Model { fireflies }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.fireflies.iter_mut().for_each(|firefly| {
        let r = random_range(0, 4);

        match r {
            0 => firefly.position.x += 1.0,
            1 => firefly.position.y += 1.0,
            2 => firefly.position.x -= 1.0,
            3 => firefly.position.y -= 1.0,
            _ => (),
        }
    });
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // let win = app.window_rect();

    // Let's first color the background
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    // draw.background().color(BLACK);

    model.fireflies.iter().for_each(|firefly| {
        draw.ellipse()
            .x_y(firefly.position.x, firefly.position.y)
            .radius(1.0)
            .color(hsla(0.2, 1.0, 2.0, 0.01));
    });

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

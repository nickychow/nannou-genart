use nannou::prelude::*;

struct Walker {
    position: Point2,
    velocity: Vec2,
}

impl Walker {
    fn new(win: Rect) -> Self {
        Walker {
            position: pt2(
                random_range(win.left(), win.right()),
                random_range(win.top(), win.bottom()),
            ),
            velocity: vec2(0.0, 0.0),
        }
    }

    fn update(&mut self) {
        // self.position.x += random_range(-1.0, 1.0);
        // self.position.y += random_range(-1.0, 1.0);
        self.velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        self.position += self.velocity;
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    walkers: Vec<Walker>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    let width = 720;
    let height = 720;

    // let half_w = width as f32 * 0.5;
    // let half_h = height as f32 * 0.5;
    app.new_window().size(width, height).build().unwrap();

    let win = app.window_rect();

    let walkers = (0..50).map(|_| Walker::new(win)).collect();

    // let walkers = vec![Walker::new()];

    // We just return an empty struct here
    Model { walkers }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, model: &mut Model, _update: Update) {
    for walker in &mut model.walkers {
        // let r = random_range::<i32>(0, 4);
        // match r {
        //     0 => walker.position.x += 1.0,
        //     1 => walker.position.x -= 1.0,
        //     2 => walker.position.y += 1.0,
        //     3 => walker.position.y -= 1.0,
        //     _ => {}
        // }
        walker.update();
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

    for walker in &model.walkers {
        draw.ellipse()
            .x_y(walker.position.x, walker.position.y)
            .w_h(2.0, 2.0)
            .color(hsla(0.2, 1.0, 2.0, 0.01));
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

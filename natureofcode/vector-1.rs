use nannou::prelude::*;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    movers: Vec<Mover>,
}

struct Mover {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
}

impl Mover {
    fn new(rect: Rect<f32>) -> Self {
        let rand_x = random_range(rect.left(), rect.right());
        let rand_y = random_range(rect.top(), rect.bottom());
        let position = pt2(rand_x, rand_y);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let top_speed = 5.0;
        Mover {
            position,
            velocity,
            acceleration,
            top_speed,
        }
    }

    fn update(&mut self, mouse: Point2) {
        self.acceleration = mouse - self.position;
        self.acceleration = self.acceleration.normalize() * 0.2;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.position += self.velocity;
    }

    fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .xy(self.position)
            .w_h(48.0, 48.0)
            .rgba(0.5, 0.5, 0.5, 0.7)
            .stroke(BLACK)
            .stroke_weight(1.0);
    }
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    let rect = Rect::from_w_h(720.0, 720.0);
    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .build()
        .unwrap();

    // We just return an empty struct here
    let movers = (0..20).map(|_| Mover::new(rect)).collect();
    Model { movers }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    for mover in &mut model.movers {
        let mouse = app.mouse.position();
        mover.update(mouse);
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // let win = app.window_rect();

    // let start = pt2(0.0, 0.0);
    // let mouse = app.mouse.position();
    // let end = mouse / 2.0;

    // let top_left = win.top_left();
    // // let l2_start = top_left;
    // let mag_end = pt2(top_left.x + start.distance(mouse), top_left.y);
    // // println!("{:?}", start.distance(mouse));

    // let normal_end = mouse.normalize() * 100.0;

    // Let's first color the background
    draw.background().color(WHITE);

    // draw.line()
    //     .start(start)
    //     .end(normal_end)
    //     .weight(1.0)
    //     .color(BLACK);

    for mover in &model.movers {
        mover.display(&draw);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

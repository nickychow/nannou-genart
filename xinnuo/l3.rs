use nannou::prelude::*;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    movers: Vec<Mover>,
    clicked: bool,
}

struct Mover {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Mover {
    fn new(mass: f32, position: Point2) -> Self {
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        Mover {
            position,
            velocity,
            acceleration,
            mass,
        }
    }

    fn apple_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .xy(self.position)
            .radius(self.mass.sqrt() * 10.0)
            .rgba(0.3, 0.3, 0.3, 0.5)
            .stroke(BLACK)
            .stroke_weight(1.0);
    }

    fn check_edges(&mut self, rect: Rect) {
        let radius = self.mass.sqrt() * 10.0;
        let edge_top = rect.top() - radius;
        let edge_bottom = rect.bottom() + radius;
        let edge_left = rect.left() + radius;
        let edge_right = rect.right() - radius;

        if self.position.x > edge_right {
            self.velocity.x *= -1.0;
            self.position.x = edge_right;
        } else if self.position.x < edge_left {
            self.velocity.x *= -1.0;
            self.position.x = edge_left;
        }

        if self.position.y > edge_top {
            self.velocity.y *= -1.0;
            self.position.y = edge_top;
        } else if self.position.y < edge_bottom {
            self.velocity.y *= -1.0;
            self.position.y = edge_bottom;
        }
    }
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    let rect = Rect::from_w_h(720.0, 720.0);
    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build()
        .unwrap();

    let movers = (0..2)
        .map(|_| Mover::new(random_range(0.01_f32, 4.0), pt2(rect.left(), rect.top())))
        .collect();

    // We just return an empty struct here
    Model {
        movers,
        clicked: false,
    }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let rect = app.window_rect();

    for mover in &mut model.movers {
        let wind = vec2(0.01, 0.0);
        let gravity = vec2(0.0, -0.05 * mover.mass);

        let c = 0.05;
        let mut friction = mover.velocity;
        if friction.length() > 0.0 {
            friction = friction.normalize();
            friction *= -1.0;
            friction *= c;
            mover.apple_force(friction);
        }

        // let mouse = app.mouse.position();
        if model.clicked {
            mover.apple_force(wind);
        }

        mover.apple_force(gravity);
        mover.update();
        mover.check_edges(rect);
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Let's first color the background
    draw.background().color(WHITE);
    // then draw a 42 x 42 (pixels) orange rectangle
    // at the center of the screen (0, 0)
    for mover in &model.movers {
        mover.display(&draw);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = true;
}
fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = false;
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

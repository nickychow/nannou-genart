use nannou::prelude::*;
// http://www.codeplastic.com/2017/09/09/controlled-circle-packing-with-processing/
//https://youtu.be/QHEQuoIKgNE

// const LINE_WIDTH: f32 = 2.0;
const MIN_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 300.0;
const N_CIRCLES: usize = 5000;
const CREATE_CIRCLE_ATTEMPTS: usize = 1500;

struct Circle {
    coordinates: Point2,
    radius: f32,
    color: Rgba,
}

impl Circle {
    fn collides(&self, other: &Circle) -> bool {
        let distance = self.coordinates.distance(other.coordinates);
        distance < self.radius + other.radius
    }

    fn any_collides(&self, others: &[Circle]) -> bool {
        others.iter().any(|other| self.collides(other))
    }

    fn edges(&self, rect: Rect) -> bool {
        self.coordinates.x < rect.right() - self.radius + 1.0
            && self.coordinates.x > rect.left() + self.radius
            && self.coordinates.y < rect.top() - self.radius
            && self.coordinates.y > rect.bottom() + self.radius
    }

    fn grow(&mut self) {
        self.radius += 1.0;
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    circles: Vec<Circle>,
}

// Builds the model
fn model(app: &App) -> Model {
    app.new_window().size(720, 720).build().unwrap();
    let window = app.window_rect();

    let mut circles = Vec::<Circle>::with_capacity(N_CIRCLES);

    for _ in 0..=N_CIRCLES {
        for _attempt in 0..=CREATE_CIRCLE_ATTEMPTS {
            let radius = random_range(MIN_RADIUS, MAX_RADIUS);
            let coordinates = pt2(
                random_range(window.left(), window.right()),
                random_range(window.top(), window.bottom()),
            );
            let color = rgba(random_f32(), random_f32(), random_f32(), random_f32());

            let circle = Circle {
                coordinates,
                radius,
                color,
            };

            if !circle.any_collides(&circles) && circle.edges(window) {
                circles.push(circle);
                break;
            }
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
    draw.background().color(BLACK);

    for circle in &model.circles {
        draw.ellipse()
            .xy(circle.coordinates)
            .radius(circle.radius)
            .color(circle.color)
            .stroke_weight(1.0)
            .stroke_color(SNOW);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

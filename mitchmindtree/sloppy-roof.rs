use nannou::prelude::*;

const STARS_PER_100_PIXELS: usize = 500;
const STARS_PER_PATH: usize = 50;
const CLOSEST: usize = STARS_PER_PATH / 10;

fn random_star_points(win: Rect) -> Vec<Point2> {
    let w = win.w();
    let h = win.h();
    let n_stars = (w * h / (STARS_PER_100_PIXELS as f32).powi(2)) as usize * STARS_PER_100_PIXELS;
    let half_w = w * 0.5;
    let half_h = h * 0.5;
    (0..n_stars)
        .map(|_| {
            let coord = || {
                let v: f32 = random();
                let sign: bool = random();
                if sign {
                    v
                } else {
                    -v
                }
            };
            pt2(coord() * half_w, coord() * half_h)
        })
        .collect()
}

fn distance_order(origin: Point2, a: Point2, b: Point2) -> std::cmp::Ordering {
    origin.distance(a).partial_cmp(&origin.distance(b)).unwrap()
}

fn random_star_path(points: &[Point2]) -> Vec<usize> {
    let mut points_by_distance: Vec<_> = points.iter().cloned().enumerate().collect();
    let o = pt2(0.0, 0.0);
    points_by_distance.sort_by(|&(_, a), &(_, b)| distance_order(o, a, b));
    let mut origin = points_by_distance[0].0;
    let mut path = vec![origin];
    while path.len() < STARS_PER_PATH {
        let o = points_by_distance[origin].1;
        points_by_distance = points_by_distance.iter().cloned().take(CLOSEST).collect();
        points_by_distance.sort_by(|&(_, a), &(_, b)| distance_order(o, a, b));
        origin = random_range(0, CLOSEST);
        path.push(origin);
    }
    path
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    points: Vec<Point2>,
    path: Vec<usize>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let points = random_star_points(app.window_rect());
    let path = random_star_path(&points);

    // We just return an empty struct here
    Model { points, path }
}

// Updates the model (note the mutable reference)
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    let win = app.window_rect();

    let t = app.time;

    // Let's first color the background
    draw.background().color(BLACK);

    // Path.
    let mut iter = model.path.iter().peekable();
    let mut i = 0;
    while let Some(&a) = iter.next() {
        if let Some(&&b) = iter.peek() {
            let start = model.points[a];
            let end = model.points[b];
            let a = (t + i as f32 * 0.2).sin().max(0.0).powi(6);
            draw.line().points(start, end).hsla(1.0, 1.0, 1.0, a);
            i += 1;
        }
    }

    // Stars.
    for (i, &p) in model.points.iter().enumerate() {
        let i_f = i as f32 / model.points.len() as f32;
        let x_f = p.x / win.w();
        let a = (t + i as f32 * 0.01).sin().max(0.0).powi(4);
        let r = i_f % 2.0 % 1.0;
        let h = (x_f + i_f + t * 0.2) % 1.0;
        draw.ellipse()
            .resolution(8.0)
            //.radius(1.0)
            .radius(r)
            .xy(p)
            .hsla(h, 0.5, 0.9, a);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

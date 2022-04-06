use nannou::noise::{NoiseFn, OpenSimplex};
use nannou::prelude::*;

const CAPTURE: bool = false;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    points: Vec<Point3>,
    noise: OpenSimplex,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let mut points = Vec::new();

    for x in -20..20 {
        for y in -20..20 {
            points.push(pt3(x as f32, y as f32, 0.0));
        }
    }

    let noise = OpenSimplex::new();

    // We just return an empty struct here
    Model { points, noise }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let time = (app.elapsed_frames() as f32) * 0.03;
    let noise_scale = 10.0;

    for point in &mut model.points {
        let noise: f32 = model.noise.get([
            point.x as f64 / noise_scale,
            point.y as f64 / noise_scale,
            time as f64,
        ]) as f32;

        point.z = noise;
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    draw.background().color(BLACK);

    for point in &model.points {
        let d = vec2(point.x, point.y).normalize();
        let r = point.z * 6.0 + 6.0;
        let p = vec2(point.x, point.y) * 15.0 + d * point.z * 15.0;

        draw.rect()
            .x_y(p.x, p.y)
            .w_h(r, r)
            .color(BLACK)
            .stroke(hsla(1.0 - point.z as f32 / 2.0 + 0.5, 1.0, 0.5, 1.0))
            .stroke_weight(2.0 - point.z);
    }

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();

    if CAPTURE {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

use nannou::prelude::*;

const NUM_FRAMES: u64 = 20;

fn ease(p: f32, g: f32) -> f32 {
    // p.powf(g)
    if p < 0.5 {
        (2.0 * p).powf(g) * 0.5
    } else {
        1.0 - (2.0 * (1.0 - p)).powf(g) * 0.5
    }
}

fn pixel_color(pos: &Vec2, time: f32) -> Rgb {
    // let y = map_range(pos.y, -310.0, 310.0, 0.0, 720.0);
    // let i = map_range(
    //     (2.0 * PI * (time + scalar_field_offset(pos.x, pos.y))).sin(),
    //     -1.0,
    //     1.0,
    //     0.0,
    //     1.0,
    // ) as f32;
    // let r = ease(i, 3.0);

    let test = (2.0 * PI * (time + 0.05 * pos.y)).sin();

    rgb(test, test, test)
    // rgb(r, r, r)
}

fn scalar_field_offset(x: f32, y: f32) -> f32 {
    0.05 * x + 0.05 * y
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    points: Vec<(Vec2, Rgb)>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let win = app.window_rect();
    let win_p = win.pad(50.0);

    let mut points = Vec::new();

    for x in win_p.left() as i32..=win_p.right() as i32 {
        for y in win_p.bottom() as i32..=win_p.top() as i32 {
            let p = pt2(x as f32, y as f32);
            let color = Rgb::new(0.0, 0.0, 0.0);

            points.push((p, color));
        }
    }

    // We just return an empty struct here
    Model { points }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let time = ((app.elapsed_frames() - 1) % NUM_FRAMES) as f32 / NUM_FRAMES as f32;
    // dbg!(time);
    for (pos, color) in model.points.iter_mut() {
        *color = pixel_color(pos, time);
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    let win = app.window_rect();
    let win_p = win.pad(50.0);

    // Let's first color the background
    draw.background().color(BLACK);

    let points = model.points.to_owned();

    draw.polyline().weight(1.0).points_colored(points);

    draw.rect()
        .xy(win_p.xy())
        .wh(win_p.wh())
        .stroke(WHITE)
        .stroke_weight(1.0)
        .no_fill();

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

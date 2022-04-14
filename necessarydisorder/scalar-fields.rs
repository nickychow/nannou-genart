use nannou::prelude::*;

// const MARGIN: f32 = 50.0;
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
    // let r = ease(i, 3.0);

    let result = ease(
        map_range(
            (TAU * (time + (scalar_field_offset(pos.x, pos.y)))).sin(),
            -1.0,
            1.0,
            0.0,
            1.0,
        ),
        3.0,
    );

    rgb(result, result, result)
}

fn scalar_field_offset(x: f32, y: f32) -> f32 {
    let distance = (pt2(x, y)).distance(pt2(62.0, 62.0));
    0.05 * distance
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Point {
    pos: Vec2,
    color: Rgba,
}

struct Model {
    points: Vec<Point>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    // let win = app.window_rect();
    // let win_p = win.pad(50.0);

    let mut points: Vec<Point> = Vec::new();

    for x in 0..124 {
        for y in 0..=124 {
            let pos = pt2(x as f32, y as f32);
            let color = Rgba::new(0.0, 0.0, 0.0, 1.0);

            points.push(Point { pos, color });
        }
    }

    // We just return an empty struct here
    Model { points }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let time = (app.elapsed_frames() - 1) as f32 / NUM_FRAMES as f32;

    for point in model.points.iter_mut() {
        *point.color = pixel_color(&point.pos, time);
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();
    let win = app.window_rect();
    let win_p = win.pad(50.0);
    let offset = win_p.top_left();
    let gdraw = draw.xy(offset).scale_y(-1.0).scale(5.0);

    // Let's first color the background
    draw.background().color(BLACK);

    // let points = model.points;

    for point in model.points.iter() {
        gdraw
            .ellipse()
            .xy(point.pos)
            .w_h(1.0, 1.0)
            .color(point.color);
    }
    // gdraw
    //     .ellipse()
    //     .xy(pt2(0.0, 10.0))
    //     .w_h(1.0, 1.0)
    //     .color(ORANGE);

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

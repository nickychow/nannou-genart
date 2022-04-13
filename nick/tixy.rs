use nannou::prelude::*;

const ROWS: i32 = 16;
const COLS: i32 = 16;
const SIZE: f32 = 16.0;
const NUM_FRAMES: u64 = 16;

struct Dot {
    location: Vec2,
    size: f32,
    color: Rgb,
}

impl Dot {
    fn new(location: Vec2) -> Self {
        let size = 16.0;
        let color = Rgb::new(1.0, 1.0, 1.0);
        Dot {
            location,
            size,
            color,
        }
    }

    fn time(app: &App) -> f32 {
        (app.elapsed_frames() as f32 / (NUM_FRAMES * 3) as f32)
    }

    fn nromalised(&mut self, t: f32) -> f32 {
        let x = self.location.x / (SIZE + 1.0) - 8.0;
        let y = self.location.y / (SIZE + 1.0) - 8.0;

        // let i = (15.0 - x) + (x * y);

        // 1
        // random_range(-1.0, 1.0)
        // 2
        // (y / 2.0 + t).sin()
        // 3
        // t
        // 4
        x / 16.0

        // (y / 2.0 + t).sin()

        // time.sin()
        // y / 256.0
        // t
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    dots: Vec<Dot>,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    let half_rows = ROWS / 2;
    let half_cols = COLS / 2;

    let mut dots = Vec::new();

    for y in -half_rows..half_rows {
        for x in -half_cols..half_cols {
            let location_offset = pt2(8.0, 8.0);
            // get normalized xy coordinates and multiply by the size plusing the gap
            let location = pt2(x as f32, y as f32) * (SIZE + 1.0) + location_offset;
            let dot = Dot::new(location);
            dots.push(dot);
        }
    }
    // We just return an empty struct here
    Model { dots }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    let time = Dot::time(app);

    for dot in model.dots.iter_mut() {
        let n = dot.nromalised(time);
        dot.size = n.clamp(-1.0, 1.0) * 8.0;

        dot.color = if n > 0.0 {
            Rgb::new(1.0, 1.0, 1.0)
        } else {
            Rgb::new(1.0, 0.133, 0.267)
        };
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    for dot in &model.dots {
        draw.ellipse()
            .xy(dot.location)
            .radius(dot.size)
            .color(dot.color);
    }
    // draw.ellipse().x_y(0.0, 0.0).w_h(16.0, 16.0).color(ORANGE);

    draw.background().color(BLACK);

    // draw miscellaneous stuff here
    draw_grid(&draw, &win, 80.0, 1.0);
    draw_grid(&draw, &win, 16.0, 0.5);

    // Ellipse at mouse.
    draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

    // Mouse position text.
    let mouse = app.mouse.position();
    let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
    draw.text(&pos)
        .xy(mouse + vec2(0.0, 20.0))
        .font_size(14)
        .color(WHITE);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .color(GREY)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .color(GREY)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}

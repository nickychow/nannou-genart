use nannou::prelude::*;

const ROWS: i32 = 16;
const COLS: i32 = 16;
const SIZE: f32 = 16.0;
const GAP: f32 = 1.0;
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
        app.elapsed_frames() as f32 / (NUM_FRAMES * 3) as f32
        // app.elapsed_frames() as f32
    }

    /// sin(t-sqrt((x-7.5)**2+(y-6)**2))
    /// sin(y/8 + t)
    /// y - x
    /// (y > x) && (14-x < y)
    /// i%4 - y%4
    /// x%4 && y%4
    /// x>3 & y>3 & x<12 & y<12
    /// (y-6) * (x-6)
    /// (y-4*t|0) * (x-2-t|0)
    /// 4 * t & i & x & y
    /// (t*10) & (1<<x) && y==8
    /// sin(i ** 2)
    /// cos(t + i + x * y)
    /// sin(x/2) - sin(x-t) - y+6
    /// (x-8)*(y-8) - sin(t)*64
    /// -.4/(hypot(x-t%10,y-t%8)-t%2*9) // ? something wrong
    /// Math.sin(t-Math.sqrt(x*x+y*y))
    /// [5463,2194,2386][y+t*9&7]&1<<x-1
    /// (((x-8)/y+t*5)&1^1/y*8&1)*y/5
    /// y-t*3+9+3*cos(x*3-t)-5*sin(x*7)
    /// d=y*y%5.9+1,!((x+t*50/d)&15)/d // ?
    /// 1/32*tan(t/64*x*tan(i-x))
    /// hypot(x-=t%4*5,y-=8)<6&&x<y|y<-x
    /// sin(2*atan((y-7.5)/(x-7.5))+5*t)
    /// (x-y) - sin(t) * 16
    /// (x-y)/24 - sin(t)
    /// (x-5)**2 + (y-5)**2 - 99*sin(t)
    /// sin(atan2(y-=8,x-=8)+hypot(y,x)+t)

    fn nromalised(&mut self, t: f32) -> f32 {
        let x = self.location.x / (SIZE + 1.0);
        let y = self.location.y / (SIZE + 1.0);
        let i = x + 16.0 * y; // i is the index of the dot

        // random_range(-1.0, 1.0)
        // (y / 2.0 + t).sin()
        // t
        // x / 16.0
        // y / 16.0
        // [1, 0, -1][i as usize % 3] as f32
        // (t - ((x - 7.5).powi(2) + (y - 6.5).powi(2)).sqrt()).sin()
        // y - x
        // (i as i32 % 4 - y as i32 % 4) as f32
        // (y - 4.0 * t) * (x - 2.0 - t)
        // (i.powi(2)).sin()
        // (t + i + x * y).sin()
        // (x / 2.0).sin() - (x - t).sin() - y + 6.0
        // (x - 8.0) * (y - 8.0) - t.sin() * 64.0
        // (t - (x.powi(2) + y.powi(2)).sqrt()).sin()
        // y - t * 3.0 + 9.0 + 3.0 * (x * 3.0 - t).cos() - 5.0 * (x * 7.0).sin()
        // 1.0 / 32.0 * (t / 64.0 * x * (i - x).tan()).tan()
        // (x - t % 4.0 * 5.0).hypot(y - 8.0)

        // t.sin()
        // i / 256.0
        // i - 76.0
        // y - t

        // -0.4 / ((x - t).hypot(y - t) - (t as u32 % 2 * 9) as f32)
        // y - t * 4.0

        // (t - ((x - 7.5).powi(2) + (y - 6.5).powi(2)).sqrt()).sin()
        // y - t * 3.0 + 9.0 + 3.0 * (x * 3.0 - t).cos() - 5.0 * (x * 7.0).sin()
        (x - t % 4.0 * 5.0).hypot(y - 8.0)
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

    let mut dots = Vec::new();

    for y in 0..ROWS {
        for x in 0..COLS {
            // get normalized xy coordinates and multiply by the size plusing the gap
            let location = pt2(x as f32, y as f32) * (SIZE + 1.0);
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
    let offset = pt2(
        (COLS as f32 * (SIZE + GAP) - SIZE) / 2.0,
        (ROWS as f32 * (SIZE + GAP) - SIZE) / 2.0,
    );
    let gdraw = draw.scale_y(-1.0).xy(-offset);

    let window = app.main_window();

    for dot in &model.dots {
        gdraw
            .ellipse()
            .xy(dot.location)
            .radius(dot.size)
            .color(dot.color);
    }

    draw.background().color(BLACK);

    // gdraw.ellipse().x_y(0.0, 0.0).w_h(16.0, 16.0).color(ORANGE);
    // // draw miscellaneous stuff here
    // let win = window.rect();
    // draw_grid(&draw, &win, 80.0, 1.0);
    // draw_grid(&draw, &win, 16.0, 0.5);

    // // Ellipse at mouse.
    // draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

    // // Mouse position text.
    // let mouse = app.mouse.position();
    // let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
    // draw.text(&pos)
    //     .xy(mouse + vec2(0.0, 20.0))
    //     .font_size(14)
    //     .color(WHITE);

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

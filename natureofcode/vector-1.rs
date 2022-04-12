use nannou::prelude::*;

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(720, 720).build().unwrap();

    // We just return an empty struct here
    Model {}
}

// Updates the model (note the mutable reference)
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Draws on the screen
fn view(app: &App, _model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    let win = app.window_rect();

    let start = pt2(0.0, 0.0);
    let mouse = app.mouse.position();
    let end = mouse / 2.0;

    let top_left = win.top_left();
    // let l2_start = top_left;
    let mag_end = pt2(top_left.x + start.distance(mouse), top_left.y);
    // println!("{:?}", start.distance(mouse));

    let normal_end = mouse.normalize() * 100.0;

    // Let's first color the background
    draw.background().color(WHITE);
    // then draw a 42 x 42 (pixels) orange rectangle
    // at the center of the screen (0, 0)
    // draw.rect().x_y(0.0, 0.0).w_h(42.0, 42.0).color(ORANGE);
    // draw.line().start(start).end(mouse).weight(1.0).color(BLACK);
    // draw.line().start(start).end(end).weight(1.0).color(BLACK);
    // draw.line()
    //     .start(top_left)
    //     .end(mag_end)
    //     .weight(10.0)
    //     .color(BLACK);
    draw.line()
        .start(start)
        .end(normal_end)
        .weight(1.0)
        .color(BLACK);

    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

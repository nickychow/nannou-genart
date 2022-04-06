use nannou::prelude::*;

const A: f32 = -1.0251;
const B: f32 = -1.345;
const C: f32 = 2.12;
const D: f32 = 1.32;

// formula dell'attrattore
fn attrc(x: f32, y: f32) -> (f32, f32) {
    (
        (A * y).sin() + C * (A * x).cos(),
        (B * x).sin() + D * (B * y).cos(),
    )
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    // x e y random tra 0 e 1 (forse anche tra -1 e 1? boh)
    let mut x = random_f32();
    let mut y = random_f32();

    for _i in 0..4000 {
        let xy = attrc(x, y);
        x = xy.0;
        y = xy.1;

        draw.ellipse()
            .x_y(x * 100.0, y * 100.0)
            .w_h(1.0, 1.0)
            .color(hsla(0.2, 1.0, 2.0, 0.01));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::sketch(view).size(720, 720).run();
}

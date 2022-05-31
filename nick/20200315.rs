use nannou::geom::range::Range;
use nannou::prelude::*;
use std::{cell::RefCell, rc::Rc};

const MAX_LINE_LENGTH2: f32 = 2500.0;
const MAX_LINES_EVER: usize = 20;

struct LinePoint {
    pos: Point2,
    vel: Vec2,
    sine_i: usize,
    max_lines: usize,
    lines: Vec<Rc<RefCell<LinePoint>>>,
}

impl LinePoint {
    fn new_at(pos: Point2, sine_i: usize) -> Self {
        let mut lines = Vec::new();
        lines.reserve(MAX_LINES_EVER);
        Self {
            pos,
            vel: vec2(0.0, 0.0),
            sine_i,
            max_lines: MAX_LINES_EVER,
            lines,
        }
    }

    fn update(&mut self, app: &App, frication: f32, force_strength: f32) {
        // let dt = app.delta_seconds();
        // self.pos += self.vel * dt;
        // self.vel += vec2(0.0, 0.0.sin() * 0.1);
        let win_rect = app.window_rect();
        // Remove lines if there are too many.
        if self.lines.len() > self.max_lines {
            for _ in 0..self.lines.len() - self.max_lines {
                self.lines.pop();
            }
        }
        // Rpmove lines to points that are too far away.
        let local_pos = self.pos;
        self.lines
            .retain(|l| l.borrow().pos.distance(local_pos) < MAX_LINE_LENGTH2);

        // Move towards connected dots it's far away from and away from close ones.
        self.vel *= frication; // Velocity damping, 0.7 is good.
        let dist_range = Range::new(0.0, MAX_LINE_LENGTH2);
        let vel_range = Range::new(-force_strength, force_strength);
        for lp in &self.lines {
            let lp = lp.borrow();
            let dist = lp.pos.distance(local_pos);
            let force = dist_range.map_value(dist, &vel_range);
            self.vel += (lp.pos - self.pos) * force;
        }

        self.pos += self.vel;

        // let speed_range = Range::new(0.0, 10.0);
        // let freq_range = Range::new(100.0, 1000.0);
        // let freq: f64 = speed_range.map_value(self.vel.distance(pt2(0.0, 0.0)) as f64, &freq_range);

        self.pos.x = self.pos.x.max(win_rect.left()).min(win_rect.right());
        // self.pos.x = self.pos.x.max(win_rect.right()).min(win_rect.left());
        self.pos.y = self.pos.y.max(win_rect.bottom()).min(win_rect.top());
        // self.pos.y = self.pos.y.max(win_rect.top()).min(win_rect.bottom());
    }
}

impl PartialEq for LinePoint {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

// This struct represents our "data state"
// It should contain and model whatever we want to draw on screen
struct Model {
    points: Vec<Rc<RefCell<LinePoint>>>,
    friction: f32,
    max_lines: usize,
    force_strength: f32,
}

// Builds the model
fn model(app: &App) -> Model {
    // The window is created here; we could
    // also create it in the main function
    app.new_window().size(1024, 1024).build().unwrap();

    // We just return an empty struct here
    Model {
        points: Vec::new(),
        friction: 0.2,
        max_lines: 10,
        force_strength: 0.5,
    }
}

// Updates the model (note the mutable reference)
fn update(app: &App, model: &mut Model, _update: Update) {
    if let Some(pushed_point) = app.mouse.buttons.left().if_down() {
        let mouse_pos = app.mouse.position();
        let dist_from_pushed: f32 = mouse_pos.distance(pushed_point);
        if dist_from_pushed > 0.0 && random_f32() > 0.6 {
            let dist_from_pushed = 15.0;
            let new_pod = mouse_pos
                + pt2(
                    random_range(-dist_from_pushed, dist_from_pushed),
                    random_range(-dist_from_pushed, dist_from_pushed),
                );
            let sine_i: usize = random();
            let new_point = Rc::new(RefCell::new(LinePoint::new_at(new_pod, sine_i)));
            model.points.push(new_point);
        }
    }

    // Create a random new point
    if random_f32() > 0.6 {
        let win = app.window_rect();
        let new_pos = pt2(
            random_range(win.left(), win.right()),
            random_range(win.top(), win.bottom()),
        );
        let sine_i: usize = random();
        let new_point = Rc::new(RefCell::new(LinePoint::new_at(new_pos, sine_i)));
        model.points.push(new_point);
    }

    // add lines to points without neighbours
    for p in &model.points {
        let find_new_neighbours = p.borrow().lines.len() < p.borrow().max_lines;
        if find_new_neighbours {
            let min_dist = 1000000.0;
            let mut closest_neighbour: Option<Rc<RefCell<LinePoint>>> = None;
            let pos = p.borrow().pos;
            for np in &model.points {
                if np.borrow().lines.len() < np.borrow().max_lines
                    && !np.borrow().lines.contains(p)
                    && !p.borrow().lines.contains(np)
                {
                    let dist = np.borrow().pos.distance(pos);
                    if dist < min_dist && dist > 0.0 && dist < MAX_LINE_LENGTH2 {
                        closest_neighbour = Some(Rc::clone(np));
                    }
                }
            }
            if let Some(point_rc) = closest_neighbour {
                // add as a line to both points
                point_rc.borrow_mut().lines.push(Rc::clone(p));
                p.borrow_mut().lines.push(point_rc);
            }
        }
    }

    // Update all points.
    for p in &model.points {
        p.borrow_mut().max_lines = model.max_lines;
        p.borrow_mut()
            .update(app, model.friction, model.force_strength);
    }
}

// Draws on the screen
fn view(app: &App, model: &Model, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();

    // Let's first color the background
    draw.background().color(hsl(0.7, 0.3, 0.1));

    for p in &model.points {
        let pos = p.borrow().pos;
        draw.ellipse()
            .xy(pos)
            .radius(30.0)
            .color(hsla(0.7, 0.8, 0.4, 0.2));
        for np in &p.borrow().lines {
            let np = np.borrow();
            draw.line()
                .start(pos)
                .end(np.pos)
                .color(hsla(0.7, 0.5, 0.7, 0.1))
                .weight(2.0);
        }
    }
    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

// Starting point of the app
fn main() {
    nannou::app(model).view(view).update(update).run();
}

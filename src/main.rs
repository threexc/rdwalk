use nannou::prelude::*;
use rand::Rng;
use std::env;

struct RandomWalk {
    positions: Vec<Point2>,
    current_pos: Point2,
}

impl RandomWalk {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            current_pos: pt2(0.0, 0.0),
        }
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Generate random step in x and y directions
        let dx: f32 = rng.gen_range(-1.0..=1.0);
        let dy: f32 = rng.gen_range(-1.0..=1.0);
        
        // Update current position
        self.current_pos.x += dx;
        self.current_pos.y += dy;
        
        // Store the path
        self.positions.push(self.current_pos);
    }
}

fn main() {
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> RandomWalk {
    RandomWalk::new()
}

fn update(_app: &App, walk: &mut RandomWalk, _update: Update) {
    // Take multiple steps per frame for more interesting visualization
    for _ in 0..5 {
        walk.step();
    }
}

fn view(app: &App, walk: &RandomWalk, frame: Frame) {
    // Prepare to draw
    let draw = app.draw();
    draw.background().color(WHITE);

    // Draw the path of the random walk
    if walk.positions.len() > 1 {
        for window in walk.positions.windows(2) {
            draw.line()
                .start(window[0])
                .end(window[1])
                .color(BLUE)
                .weight(2.0);
        }
    }

    // Draw the current position
    draw.ellipse()
        .xy(walk.current_pos)
        .radius(5.0)
        .color(RED);

    // Render to the frame
    draw.to_frame(app, &frame).unwrap();
}

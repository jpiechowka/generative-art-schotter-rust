use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use std::fmt::{self, Display};
use std::time::{SystemTime, UNIX_EPOCH};

const ROWS: u32 = 25;
const COLUMNS: u32 = 15;
const SQUARE_SIZE_PX: u32 = 30;
const MARGIN_TOP_BOTTOM_PX: u32 = 40;
const MARGIN_SIDES_PX: u32 = 40;
const WINDOW_WIDTH: u32 = COLUMNS * SQUARE_SIZE_PX + 2 * MARGIN_SIDES_PX;
const WINDOW_HEIGHT: u32 = ROWS * SQUARE_SIZE_PX + 2 * MARGIN_TOP_BOTTOM_PX;
const SQUARE_LINE_WIDTH_RATIO: f32 = 0.075; // In relation to square size

const MAX_RNG_SEED_VAL: u64 = 100_000_000;

#[derive(Debug)]
struct Model {
    random_seed: u64,
    displacement_adjustment: f32,
    rotation_adjustment: f32,
    squares: Vec<Square>,
}

impl Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(seed: {}, displacement_adj: {}, rotation_adj: {}, len_squares: {})",
            self.random_seed,
            self.displacement_adjustment,
            self.rotation_adjustment,
            self.squares.len()
        )
    }
}

#[derive(Debug)]
struct Square {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
}

impl Square {
    fn new(x: f32, y: f32) -> Self {
        let x_offset = 0.0;
        let y_offset = 0.0;
        let rotation = 0.0;
        Square {
            x,
            y,
            x_offset,
            y_offset,
            rotation,
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let random_seed = random_range(0, MAX_RNG_SEED_VAL);
    let displacement_adjustment = 1.0;
    let rotation_adjustment = 1.0;

    let mut squares = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLUMNS {
            let stone = Square::new(x as f32, y as f32);
            squares.push(stone);
        }
    }

    let initial_model = Model {
        random_seed,
        displacement_adjustment,
        rotation_adjustment,
        squares,
    };

    println!("Initial model: {}", initial_model);
    initial_model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut rng = StdRng::seed_from_u64(model.random_seed);
    for square in &mut model.squares {
        let factor = square.y / ROWS as f32;
        let displacement_factor = factor * model.displacement_adjustment;
        let rotation_factor = factor * model.rotation_adjustment;
        square.x_offset = displacement_factor * rng.gen_range(-0.5..0.5);
        square.y_offset = displacement_factor * rng.gen_range(-0.5..0.5);
        square.rotation = rotation_factor * rng.gen_range(-PI / 4.0..PI / 4.0); // 45 degrees
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw(); // standard Nannou coordinate system
    let grid_draw = draw
        .scale(SQUARE_SIZE_PX as f32) // custom grid coordinate system
        .scale_y(-1.0)
        .x_y(COLUMNS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    for square in &model.squares {
        let cell_draw = grid_draw.x_y(square.x, square.y);
        cell_draw
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(SQUARE_LINE_WIDTH_RATIO)
            .w_h(1.0, 1.0)
            .x_y(square.x_offset, square.y_offset)
            .rotate(square.rotation);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, MAX_RNG_SEED_VAL);
            println!("R key pressed. Regenerating seed. New model: {}", model)
        }
        Key::S => {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let filename = format!("{}_{}.png", app.exe_name().unwrap(), timestamp);
            println!("S key pressed. Saving screenshot as: {filename}");
            app.main_window().capture_frame(filename);
        }
        Key::Up => {
            model.displacement_adjustment += 0.1;
            println!(
                "UP key pressed. Adjusting displacement. New model: {}",
                model
            );
        }
        Key::Down => {
            if model.displacement_adjustment > 0.0 {
                model.displacement_adjustment -= 0.1;
                println!(
                    "DOWN key pressed. Adjusting displacement. New model: {}",
                    model
                );
            }
        }
        Key::Right => {
            model.rotation_adjustment += 0.1;
            println!(
                "RIGHT key pressed. Adjusting rotation. New model: {}",
                model
            );
        }
        Key::Left => {
            if model.rotation_adjustment > 0.0 {
                model.rotation_adjustment -= 0.1;
                println!("LEFT key pressed. Adjusting rotation. New model: {}", model);
            }
        }
        Key::Space | Key::D => {
            model.displacement_adjustment = 1.0;
            model.rotation_adjustment = 1.0;
            println!(
                "SPACE or D key pressed. Setting default values for displacement and rotation. New model: {}",
                model
            );
        }
        _other_key => {}
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run()
}

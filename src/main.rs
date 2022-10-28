use nannou::prelude::*;

const ROWS: u32 = 30;
const COLUMNS: u32 = 15;
const SQUARE_SIZE_PX: u32 = 30;
const MARGIN_TOP_BOTTOM_PX: u32 = 40;
const MARGIN_SIDES_PX: u32 = 40;
const WINDOW_WIDTH: u32 = COLUMNS * SQUARE_SIZE_PX + 2 * MARGIN_SIDES_PX;
const WINDOW_HEIGHT: u32 = ROWS * SQUARE_SIZE_PX + 2 * MARGIN_TOP_BOTTOM_PX;
const SQUARE_LINE_WIDTH_RATIO: f32 = 0.075; // In relation to square size

fn main() {
    nannou::sketch(view)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw(); // standard Nannou coordinate system
    let grid_draw = draw
        .scale(SQUARE_SIZE_PX as f32) // custom grid coordinate system
        .scale_y(-1.0)
        .x_y(COLUMNS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    grid_draw
        .rect()
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(SQUARE_LINE_WIDTH_RATIO)
        .w_h(1.0, 1.0)
        .x_y(0.0, 0.0)
        .rotate(0.0);

    draw.to_frame(app, &frame).unwrap();
}

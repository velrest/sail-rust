use crate::settings::{SQUARE_SIZE, SQUARES};
use macroquad::prelude::*;

pub fn draw_map() {
    let offset_x = -SQUARES * SQUARE_SIZE / 2 as f32;
    let offset_y = -SQUARES * SQUARE_SIZE / 2 as f32;

    for i in 1..SQUARES as i32 {
        draw_line(
            offset_x + i as f32 * SQUARE_SIZE,
            offset_y,
            offset_x + i as f32 * SQUARE_SIZE,
            offset_y + SQUARES * SQUARE_SIZE,
            2.,
            BLACK,
        );
    }

    for i in 1..SQUARES as i32 {
        draw_line(
            offset_x,
            offset_y + i as f32 * SQUARE_SIZE,
            offset_x + SQUARES * SQUARE_SIZE,
            offset_y + i as f32 * SQUARE_SIZE,
            2.,
            BLACK,
        );
    }
}

use macroquad::prelude::*;
use crate::settings::{SQUARE_SIZE, SQUARES};


pub fn draw_map() {
    let offset_x = -SQUARES / 2 as f32;
    let offset_y = -SQUARES / 2 as f32;

    for i in 1..SQUARES as i32 {
        draw_line(
            offset_x,
            offset_y + SQUARE_SIZE * i as f32,
            screen_width() - offset_x,
            offset_y + SQUARE_SIZE * i as f32,
            2.,
            BLACK,
        );
    }

    for i in 1..SQUARES as i32 {
        draw_line(
            offset_x + SQUARE_SIZE * i as f32,
            offset_y,
            offset_x + SQUARE_SIZE * i as f32,
            screen_height() - offset_y,
            2.,
            BLACK,
        );
    }
}

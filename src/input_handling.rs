use crate::settings::{ACCEL_CHANGE, DIR_CHANGE};
use macroquad::prelude::{KeyCode, is_key_down};

struct KeyMap {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    quit: bool,
}

#[derive(Debug)]
pub struct MovementChange {
    pub direction_change: f32,
    pub speed_change: f32,
}

fn build_keymap() -> KeyMap {
    KeyMap {
        right: is_key_down(KeyCode::Right),
        left: is_key_down(KeyCode::Left),
        up: is_key_down(KeyCode::Up),
        down: is_key_down(KeyCode::Down),
        quit: is_key_down(KeyCode::Q),
    }
}

pub fn handle_input() -> Option<MovementChange> {
    let keys = build_keymap();
    if keys.quit {
        None
    } else {
        Some(MovementChange {
            direction_change: match keys {
                KeyMap {
                    left: true,
                    right: true,
                    ..
                } => 0.,
                KeyMap { right: true, .. } => -DIR_CHANGE,
                KeyMap { left: true, .. } => DIR_CHANGE,
                _ => 0.,
            },
            speed_change: match keys {
                KeyMap {
                    down: true,
                    up: true,
                    ..
                } => 0.,
                KeyMap { down: true, .. } => -ACCEL_CHANGE,
                KeyMap { up: true, .. } => ACCEL_CHANGE,
                _ => 0.,
            },
        })
    }
}

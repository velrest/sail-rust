use macroquad::prelude::*;

pub mod settings;
use settings::{ACCEL_CHANGE, DIR_CHANGE};

mod boat;
use boat::{Boat, MovementChange, update_boat};

mod ui;
use ui::draw_ui;

mod map;
use map::draw_map;

struct KeyMap {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn build_keymap() -> KeyMap {
    KeyMap {
        right: is_key_down(KeyCode::Right),
        left: is_key_down(KeyCode::Left),
        up: is_key_down(KeyCode::Up),
        down: is_key_down(KeyCode::Down),
    }
}

#[macroquad::main("Sail")]
async fn main() {
    // request_new_screen_size(1920.0, 1080.0);

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));
    let mut boat = Boat {
        ..Default::default()
    };
    // TODO here
    loop {
        let keys = build_keymap();
        let movement_change = MovementChange {
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
                } => -boat.movement.speed / 100.,
                KeyMap { down: true, .. } => -ACCEL_CHANGE,
                KeyMap { up: true, .. } => ACCEL_CHANGE,
                _ => -boat.movement.speed / 100.,
            },
        };

        camera.target = vec2(boat.position.x, boat.position.y);

        set_camera(&camera);
        clear_background(BLUE);
        draw_map();
        update_boat(&mut boat, movement_change);

        set_default_camera();
        draw_ui(&boat);
        next_frame().await
    }
}

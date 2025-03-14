mod boat;
mod input_handling;
mod map;
pub mod settings;
mod ui;

use boat::Boat;
use input_handling::handle_input;
use macroquad::prelude::*;
use map::draw_map;
use ui::draw_ui;

#[macroquad::main("Sail")]
async fn main() {
    set_fullscreen(true);

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));
    let mut boat = Boat {
        ..Default::default()
    };

    loop {
        camera.target = vec2(boat.position.x, boat.position.y);

        set_camera(&camera);
        clear_background(BLUE);
        draw_map();
        match handle_input() {
            Some(movement_change) => {
                boat.update(movement_change);

                set_default_camera();
                draw_ui(&boat);
                next_frame().await
            }
            None => break,
        }
    }
}

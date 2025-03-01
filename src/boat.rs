use std::f32::consts::PI;
use macroquad::prelude::*;

pub struct Boat {
    pub position: Vec2,
    pub movement: BoatMovement,
}

impl Default for Boat {
    fn default() -> Boat {
        Boat {
            position: vec2(screen_width() / 2., screen_height() / 2.),
            movement: BoatMovement {
                ..Default::default()
            },
        }
    }
}

pub struct BoatMovement {
    pub direction: Vec2,
    pub speed: f32,
}

impl Default for BoatMovement {
    fn default() -> BoatMovement {
        BoatMovement {
            direction: vec2(0., 1.),
            speed: 10.,
        }
    }
}
pub struct MovementChange {
    pub direction_change: f32,
    pub speed_change: f32,
}

pub fn update_boat(boat: &mut Boat, movement_change: MovementChange) {
    calculate_boat_physics(boat, movement_change);
    draw_boat(boat);
}

fn calculate_boat_physics(boat: &mut Boat, movement_change: MovementChange) {
    let turn = movement_change.direction_change * get_frame_time();
    // println!("turn: {}", turn);
    let rotation_matrix = Affine2::from_angle(turn);
    let movement = BoatMovement {
        direction: rotation_matrix.transform_vector2(boat.movement.direction),
        speed: boat.movement.speed + movement_change.speed_change,
    };
    boat.position += movement.direction * movement.speed * get_frame_time();
    boat.movement = movement;
}

fn draw_boat(boat: &Boat) {
    let top_point = &boat.position;

    let angle = Vec2::to_angle(boat.movement.direction) - PI / 2.;

    let boat_texture = Texture2D::from_file_with_format(
        include_bytes!("../assets/PNG/Default size/Ship parts/hullLarge (1).png"),
        None,
    );
    let sail_texture = Texture2D::from_file_with_format(
        include_bytes!("../assets/PNG/Default size/Ship parts/sailLarge (7).png"),
        None,
    );

    draw_texture_ex(
        &boat_texture,
        top_point.x - boat_texture.width() / 2.,
        top_point.y - boat_texture.height() / 2.,
        WHITE,
        DrawTextureParams {
            rotation: angle,
            ..Default::default()
        },
    );
    draw_texture_ex(
        &sail_texture,
        top_point.x - sail_texture.width() / 2.,
        top_point.y - sail_texture.height() / 2.,
        WHITE,
        DrawTextureParams {
            rotation: angle,
            ..Default::default()
        },
    );
}

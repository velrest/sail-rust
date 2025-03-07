use macroquad::prelude::*;
use std::f32::consts::PI;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

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
    let angle = Vec2::to_angle(boat.movement.direction) + PI / 2.;
    let v1 = Vec2::new(
        boat.position.x + angle.sin() * SHIP_HEIGHT / 2.,
        boat.position.y - angle.cos() * SHIP_HEIGHT / 2.,
    );
    let v2 = Vec2::new(
        boat.position.x - angle.cos() * SHIP_BASE / 2. - angle.sin() * SHIP_HEIGHT / 2.,
        boat.position.y - angle.sin() * SHIP_BASE / 2. + angle.cos() * SHIP_HEIGHT / 2.,
    );
    let v3 = Vec2::new(
        boat.position.x + angle.cos() * SHIP_BASE / 2. - angle.sin() * SHIP_HEIGHT / 2.,
        boat.position.y + angle.sin() * SHIP_BASE / 2. + angle.cos() * SHIP_HEIGHT / 2.,
    );
    draw_triangle_lines(v1, v2, v3, 2., BLACK);
}

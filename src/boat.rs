use crate::input_handling::MovementChange;
use crate::settings::{DRAG, MAX_SPEED, MIN_DRAG_SPEED, SHIP_BASE, SHIP_HEIGHT};
use core::panic;
use macroquad::prelude::*;
use std::f32::consts::PI;

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

impl Boat {
    pub fn update(&mut self, movement_change: MovementChange) {
        self.calc_physics(movement_change);
        self.draw();
    }

    fn calc_physics(&mut self, movement_change: MovementChange) {
        if self.position.x.is_nan() || self.position.y.is_nan() {
            panic!("Position is invalid")
        }

        let turn = movement_change.direction_change
            * get_frame_time()
            * steering_force(self.movement.speed);

        let has_direction_change = movement_change.direction_change != 0.;
        let has_speed_change = movement_change.speed_change != 0.;
        let speed_with_drag = add_drag(self.movement.speed, has_direction_change, has_speed_change);

        let rotation_matrix = Affine2::from_angle(turn);
        let new_speed = speed_with_drag + movement_change.speed_change;
        let movement = BoatMovement {
            direction: rotation_matrix.transform_vector2(self.movement.direction),
            speed: if new_speed.is_sign_positive() {
                new_speed.min(MAX_SPEED)
            } else {
                new_speed.max(-MAX_SPEED)
            },
        };
        self.position += movement.direction * movement.speed * get_frame_time();
        self.movement = movement;
    }

    fn draw(&self) {
        let angle = Vec2::to_angle(self.movement.direction) + PI / 2.;
        let v1 = Vec2::new(
            self.position.x + angle.sin() * SHIP_HEIGHT / 2.,
            self.position.y - angle.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.position.x - angle.cos() * SHIP_BASE / 2. - angle.sin() * SHIP_HEIGHT / 2.,
            self.position.y - angle.sin() * SHIP_BASE / 2. + angle.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.position.x + angle.cos() * SHIP_BASE / 2. - angle.sin() * SHIP_HEIGHT / 2.,
            self.position.y + angle.sin() * SHIP_BASE / 2. + angle.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
    }
}

#[derive(Debug)]
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

fn steering_force(speed: f32) -> f32 {
    if speed == 0. {
        speed
    } else {
        // With speed as x and MAX_SPEED as y and log(b, x) the base formula is and:
        // f(x) = log(y^1/y, x) which gives us a log curve where if f(MAX_SPEED) = MAX_SPEED.
        // From this, we divide by MAX_SPEED to get a percentage we can apply.
        to_positive(speed).log(MAX_SPEED.powf(1. / MAX_SPEED)) / MAX_SPEED
    }
}

fn add_drag(speed: f32, has_direction_change: bool, has_speed_change: bool) -> f32 {
    if has_speed_change {
        speed
    } else {
        match speed {
            0. => speed,
            _ => {
                let penalty = match (speed < 0., has_direction_change) {
                    (true, true) => 4.,
                    (false, true) | (true, false) => 2.,
                    _ => 1.,
                };
                // TODO: This curve is too steep. It should not have as much drag on higher speeds.
                let drag = ((100. / to_positive(speed).max(1.)).max(1.)).min(100.) * DRAG / penalty;
                if speed.is_sign_positive() {
                    (speed - (speed / drag).max(MIN_DRAG_SPEED * get_frame_time())).max(0.)
                } else {
                    (speed - (speed / drag).min(-MIN_DRAG_SPEED * get_frame_time())).min(0.)
                }
            }
        }
    }
}

fn to_positive(float: f32) -> f32 {
    if float.is_sign_negative() {
        -float
    } else {
        float
    }
}

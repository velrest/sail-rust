use macroquad::prelude::*;

use crate::boat::{Boat, BoatMovement};

pub fn draw_ui(
    Boat {
        movement: BoatMovement { speed, .. },
        position,
    }: &Boat,
) {
    draw_rectangle(0., 0., 200., 300., LIGHTGRAY);
    draw_text("Sail", 10., 35., 50., BLACK);
    let wind_label = draw_text("Wind speed:", 10., 35. + 50. / 2. + 15., 30., BLACK);
    draw_text(
        "0",
        10. + wind_label.width,
        35. + 50. / 2. + 15.,
        30.,
        BLACK,
    );
    let boat_label = draw_text("Boat speed:", 10., 35. + 50. / 2. + 15. + 25., 30., BLACK);
    draw_text(
        &speed.trunc().to_string(),
        10. + boat_label.width,
        35. + 50. / 2. + 15. + 25.,
        30.,
        BLACK,
    );
    let y = 210.;
    let spacing = 10.;
    draw_text("Boat pos:", spacing, y, 30., BLACK);
    let xl = draw_text("x:", spacing, y + 35., 30., BLACK);
    draw_text(
        &position.x.to_string(),
        xl.width + spacing,
        y + 35.,
        30.,
        BLACK,
    );
    let yl = draw_text("y:", spacing, y + 35. * 2., 30., BLACK);
    draw_text(
        &position.y.to_string(),
        yl.width + spacing,
        y + 35. * 2.,
        30.,
        BLACK,
    );
    draw_line(
        100.,
        35. + 50. / 2. + 15. + 25. + 25.,
        100.,
        35. + 50. / 2. + 15. + 25. + 25. + 50.,
        5.,
        BLACK,
    );
    draw_line(
        100.,
        35. + 50. / 2. + 15. + 25. + 25.,
        90.,
        35. + 50. / 2. + 15. + 25. + 25. + 10.,
        5.,
        BLACK,
    );
}

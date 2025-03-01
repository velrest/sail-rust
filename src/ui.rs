use macroquad::prelude::*;

pub fn draw_ui(boat_speed: f32) {
    draw_rectangle(0., 0., 200., 200., LIGHTGRAY);
    draw_text("Sail", 10., 35., 50., BLACK);
    draw_text("Wind speed:", 10., 35. + 50. / 2. + 15., 30., BLACK);
    let boat_label = draw_text("Boat speed:", 10., 35. + 50. / 2. + 15. + 25., 30., BLACK);
    draw_text(
        &boat_speed.to_string(),
        10. + boat_label.width,
        35. + 50. / 2. + 15. + 25.,
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

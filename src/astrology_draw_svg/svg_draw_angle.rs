/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (https://github.com/stephaneworkspace)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an CC License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to this source in your commercial projects, you must
 * adhere to the GPL license or buy a Swiss Ephemeris commercial license.
 */
extern crate strum;
use svg::node::element::path::{Data, Number};
use svg::node::element::{Group, Path};
use svg::Document;
pub const ANGLE_SIZE: Number = 50.0;

pub fn draw_asc() -> Document {
    let size: (Number, Number) = (ANGLE_SIZE, ANGLE_SIZE);
    let document: Document;
    let data1 = Data::new()
        .move_to((9.2, 28.6)) // M
        .line_by((-2.5, 7.6)) // l
        .horizontal_line_to(3.4) // H
        .line_by((8.2, -24.3)) // l
        .horizontal_line_by(3.8) // h
        .line_by((8.3, 24.3)) // l
        .horizontal_line_by(-3.3) // h
        .line_by((-2.6, -7.6)) // l
        .horizontal_line_to(9.2) // H
        .close() // z
        .move_to((17.1, 26.1)) // M
        .line_by((-2.4, -7.0)) // l
        .cubic_curve_by((-0.5, -1.6, -0.9, -3.0, -1.3, -4.4)) // c
        .horizontal_line_by(-0.1) // h
        .cubic_curve_by((-0.4, 1.4, -0.8, 2.9, -1.2, 4.4)) // c
        .line_by((-2.4, 7.0)) // l
        .horizontal_line_to(17.1) // H
        .close(); // z
    let data2 = Data::new()
        .move_to((43.7, 35.4)) // M
        .cubic_curve_by((-1.2, 0.6, -3.5, 1.2, -6.4, 1.2)) // c
        .cubic_curve_by((-6.8, 0.0, -12.0, -4.3, -12.0, -12.3)) // c
        .cubic_curve_by((0.0, -7.6, 5.1, -12.7, 12.7, -12.7)) // c
        .cubic_curve_by((3.0, 0.0, 4.9, 0.6, 5.8, 1.1)) // c
        .line_to((43.0, 15.2)) // L
        .cubic_curve_by((-1.2, -0.6, -2.9, -1.0, -4.9, -1.0)) // c
        .cubic_curve_by((-5.7, 0.0, -9.5, 3.6, -9.5, 10.0)) // c
        .cubic_curve_by((0.0, 5.9, 3.4, 9.8, 9.3, 9.8)) // c
        .cubic_curve_by((1.9, 0.0, 3.9, -0.4, 5.1, -1.0)) // c
        .line_to((43.7, 35.4)) // L
        .close(); // z
    let path1 = Path::new()
        .set("fill", "black")
        .set("stroke", "black")
        .set("stroke-width", 0)
        .set("d", data1);
    let path2 = Path::new()
        .set("fill", "black")
        .set("stroke", "black")
        .set("stroke-width", 0)
        .set("d", data2);
    let group = Group::new().add(path1).add(path2);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

pub fn draw_fc() -> Document {
    let size: (Number, Number) = (ANGLE_SIZE, ANGLE_SIZE);
    let document: Document;
    let data1 = Data::new()
        .move_to((7.2, 11.9)) // M
        .horizontal_line_by(13.1) // h
        .vertical_line_by(2.6) // v
        .horizontal_line_by(-9.9) // h
        .vertical_line_by(8.1) // v
        .horizontal_line_by(9.2) // h
        .vertical_line_by(2.6) // v
        .horizontal_line_by(-9.2) // h
        .vertical_line_by(11.0) // v
        .horizontal_line_to(7.2) // H
        .vertical_line_to(11.9) // V
        .close(); // z
    let data2 = Data::new()
        .move_to((41.7, 35.4)) // M
        .cubic_curve_by((-1.2, 0.6, -3.5, 1.2, -6.4, 1.2)) // c
        .cubic_curve_by((-6.8, 0.0, -12.0, -4.3, -12.0, -12.3)) // c
        .cubic_curve_by((0.0, -7.6, 5.1, -12.7, 12.7, -12.7)) // c
        .cubic_curve_by((3.0, 0.0, 4.9, 0.6, 5.8, 1.1)) // c
        .line_to((41.0, 15.2)) // L
        .cubic_curve_by((-1.2, -0.6, -2.9, -1.0, -4.9, -1.0)) // c
        .cubic_curve_by((-5.7, 0.0, -9.5, 3.6, -9.5, 10.0)) // c
        .cubic_curve_by((0.0, 5.9, 3.4, 9.8, 9.3, 9.8)) // c
        .cubic_curve_by((1.9, 0.0, 3.9, -0.4, 5.1, -1.0)) // c
        .line_to((41.7, 35.4)) // L
        .close(); // z
    let path1 = Path::new()
        .set("fill", "black")
        .set("stroke", "black")
        .set("stroke-width", 0)
        .set("d", data1);
    let path2 = Path::new()
        .set("fill", "black")
        .set("stroke", "black")
        .set("stroke-width", 0)
        .set("d", data2);
    let group = Group::new().add(path1).add(path2);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

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

pub fn draw_degre(degre: i16) -> Document {
    let size: (Number, Number);
    let document: Document;
    match degre {
        0 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((27.1, 26.4)) // M
                .cubic_curve_by((0.0, 8.5, -3.2, 13.3, -8.7, 13.3)) // c
                .cubic_curve_by((-4.9, 0.0, -8.2, -4.6, -8.3, -12.9)) // c
                .cubic_curve_by((0.0, -8.4, 3.6, -13.1, 8.7, -13.1)) // c
                .cubic_curve_to((24.0, 13.7, 27.1, 18.4, 27.1, 26.4)) // C
                .close() // z
                .move_to((13.4, 26.8)) // M
                .cubic_curve_by((0.0, 6.5, 2.0, 10.2, 5.1, 10.2)) // c
                .cubic_curve_by((3.5, 0.0, 5.1, -4.1, 5.1, -10.5)) // c
                .cubic_curve_by((0.0, -6.2, -1.6, -10.2, -5.1, -10.2)) // c
                .cubic_curve_to((15.6, 16.3, 13.4, 19.9, 13.4, 26.8)) // C
                .close(); // z
            let data2 = Data::new()
                .move_to((39.8, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((37.9, 12.7, 39.8, 15.1, 39.8, 17.8)) // C
                .close() // z
                .move_to((31.7, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((32.7, 14.5, 31.7, 16.3, 31.7, 17.9)) // C
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
        },
        1 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((17.8, 17.3)) // M
                .line_to((17.8, 17.3)) // L
                .line_by((-4.4, 2.4)) // l
                .line_to((12.7, 17.0)) // L
                .line_by((5.5, -2.9)) // l
                .horizontal_line_to(21.0) // H
                .vertical_line_by(25.1) // v
                .horizontal_line_by(-3.3) // h
                .vertical_line_to(17.3) // V
                .close(); // z
            let data2 = Data::new()
                .move_to((39.8, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((37.9, 12.7, 39.8, 15.1, 39.8, 17.8)) // C
                .close() // z
                .move_to((31.7, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((32.7, 14.5, 31.7, 16.3, 31.7, 17.9)) // C
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
        },
        2 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((10.4, 39.2)) // M
                .vertical_line_by(-2.1) // v
                .line_by((2.7, -2.6)) // l
                .cubic_curve_by((6.4, -6.1, 9.3, -9.3, 9.4, -13.1)) // c
                .cubic_curve_by((0.0, -2.5, -1.2, -4.9, -5.0, -4.9)) // c
                .cubic_curve_by((-2.3, 0.0, -4.2, 1.2, -5.3, 2.1)) // c
                .line_to((11.0, 16.2)) // L
                .cubic_curve_by((1.7, -1.5, 4.2, -2.6, 7.1, -2.6)) // c
                .cubic_curve_by((5.4, 0.0, 7.7, 3.7, 7.7, 7.3)) // c
                .cubic_curve_by((0.0, 4.6, -3.4, 8.4, -8.7, 13.5)) // c
                .line_by((-2.0, 1.9)) // l
                .vertical_line_by(0.1) // v
                .horizontal_line_by(11.3) // h
                .vertical_line_by(2.8) // v
                .horizontal_line_to(10.4) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((39.8, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((37.9, 12.7, 39.8, 15.1, 39.8, 17.8)) // C
                .close() // z
                .move_to((31.7, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((32.7, 14.5, 31.7, 16.3, 31.7, 17.9)) // C
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
        },
        3 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((11.2, 35.3)) // M
                .cubic_curve_by((1.0, 0.6, 3.2, 1.6, 5.6, 1.6)) // c
                .cubic_curve_by((4.4, 0.0, 5.7, -2.8, 5.7, -4.9)) // c
                .cubic_curve_by((0.0, -3.5, -3.2, -5.0, -6.5, -5.0)) // c
                .horizontal_line_to(14.0) // H
                .vertical_line_by(-2.5) // v
                .horizontal_line_by(1.9) // h
                .cubic_curve_by((2.5, 0.0, 5.6, -1.3, 5.6, -4.2)) // c
                .cubic_curve_by((0.0, -2.0, -1.3, -3.8, -4.4, -3.8)) // c
                .cubic_curve_by((-2.0, 0.0, -3.9, 0.9, -5.0, 1.7)) // c
                .line_by((-0.9, -2.5)) // l
                .cubic_curve_by((1.3, -1.0, 3.9, -1.9, 6.6, -1.9)) // c
                .cubic_curve_by((4.9, 0.0, 7.2, 2.9, 7.2, 6.0)) // c
                .cubic_curve_by((0.0, 2.6, -1.5, 4.8, -4.6, 5.9)) // c
                .vertical_line_by(0.1) // v
                .cubic_curve_by((3.1, 0.6, 5.6, 2.9, 5.6, 6.5)) // c
                .cubic_curve_by((0.0, 4.0, -3.1, 7.5, -9.2, 7.5)) // c
                .cubic_curve_by((-2.8, 0.0, -5.3, -0.9, -6.5, -1.7)) // c
                .line_to((11.2, 35.3)) // L
                .close(); // z
            let data2 = Data::new()
                .move_to((39.8, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((37.9, 12.7, 39.8, 15.1, 39.8, 17.8)) // c
                .close() // z
                .move_to((31.7, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((32.7, 14.5, 31.7, 16.3, 31.7, 17.9)) // C
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
        },
        4 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((20.9, 39.2)) // M
                .vertical_line_by(-6.8) // v
                .horizontal_line_to(9.2) // H
                .vertical_line_by(-2.2) // v
                .line_by((11.2, -16.0)) // l
                .horizontal_line_by(3.7) // h
                .vertical_line_by(15.6) // v
                .horizontal_line_by(3.5) // h
                .vertical_line_by(2.7) // v
                .horizontal_line_by(-3.5) // h
                .vertical_line_by(6.8) // v
                .horizontal_line_to(20.9) // H
                .close() // z
                .move_to((20.9, 29.7)) // M
                .vertical_line_by(-8.4) // v
                .cubic_curve_by((0.0, -1.3, 0.0, -2.6, 0.1, -3.9)) // c
                .horizontal_line_by(-0.1) // h
                .cubic_curve_by((-0.8, 1.5, -1.4, 2.6, -2.1, 3.7)) // c
                .line_by((-6.1, 8.5)) // l
                .vertical_line_by(0.1) // v
                .horizontal_line_to(20.9) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((39.8, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((37.9, 12.7, 39.8, 15.1, 39.8, 17.8)) // C
                .close() // z
                .move_to((31.7, 17.9))
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((32.7, 14.5, 31.7, 16.3, 31.7, 17.9)) // C
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
        },
        5 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((25.4, 16.9)) // M
                .horizontal_line_by(-9.6) // h
                .line_by((-1.0, 6.5)) // l
                .cubic_curve_by((0.6, -0.1, 1.1, -0.2, 2.0, -0.2)) // c
                .cubic_curve_by((1.9, 0.0, 3.9, 0.4, 5.4, 1.4)) // c
                .cubic_curve_by((2.0, 1.1, 3.6, 3.3, 3.6, 6.5)) // c
                .cubic_curve_by((0.0, 4.9, -3.9, 8.6, -9.4, 8.6)) // c
                .cubic_curve_by((-2.7, 0.0, -5.1, -0.8, -6.3, -1.5)) // c
                .line_by((0.9, -2.6)) // l
                .cubic_curve_by((1.0, 0.6, 3.1, 1.4, 5.4, 1.4)) // c
                .cubic_curve_by((3.2, 0.0, 6.0, -2.1, 6.0, -5.4)) // c
                .cubic_curve_by((0.0, -3.2, -2.2, -5.6, -7.2, -5.6)) // c
                .cubic_curve_by((-1.4, 0.0, -2.5, 0.2, -3.5, 0.3)) // c
                .line_by((1.6, -12.1)) // l
                .horizontal_line_by(12.0) // h
                .vertical_line_to(16.9) // V
                .close(); // z
            let data2 = Data::new()
                .move_to((39.8, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((37.9, 12.7, 39.8, 15.1, 39.8, 17.8)) // C
                .close() // z
                .move_to((31.7, 17.9))
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((32.7, 14.5, 31.7, 16.3, 31.7, 17.9)) // C
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
        },
        _ => {
            size = (50.0, 50.0);
            document = Document::new().set("viewBox", (0, 0, size.0, size.1));
        },
    }
    document
}

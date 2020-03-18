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

pub const HOUSE_SIZE: Number = 50.0;

pub fn draw_house(house: i16) -> Document {
    let size: (Number, Number) = (HOUSE_SIZE, HOUSE_SIZE);
    let document: Document;
    match house {
        1 => {
            let data = Data::new()
                .move_to((22.9, 14.3)) // M
                .line_to((22.9, 14.3)) // L
                .line_by((-4.4, 2.4)) // l
                .line_to((17.8, 14.0)) // L
                .line_by((5.5, -2.9)) // l
                .horizontal_line_by(2.9) // h
                .vertical_line_by(25.1) // v
                .horizontal_line_by(-3.3) // h
                .vertical_line_to(14.3) // V
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        2 => {
            let data = Data::new()
                .move_to((15.5, 36.2)) // M
                .vertical_line_by(-2.1) // v
                .line_by((2.7, -2.6)) // l
                .cubic_curve_by((6.4, -6.1, 9.3, -9.3, 9.3, -13.1)) // c
                .cubic_curve_by((0.0, -2.5, -1.2, -4.9, -5.0, -4.9)) // c
                .cubic_curve_by((-2.3, 0.0, -4.2, 1.2, -5.3, 2.1)) // c
                .line_by((-1.1, -2.4)) // l
                .cubic_curve_by((1.7, -1.5, 4.2, -2.6, 7.1, -2.6)) // c
                .cubic_curve_by((5.4, 0.0, 7.7, 3.7, 7.7, 7.3)) // c
                .cubic_curve_by((0.0, 4.6, -3.4, 8.4, -8.7, 13.5)) // c
                .line_by((-2.0, 1.9)) // l
                .vertical_line_by(0.1) // v
                .horizontal_line_by(11.3) // h
                .vertical_line_by(2.8) // v
                .horizontal_line_to(15.5) // H
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        3 => {
            let data = Data::new()
                .move_to((16.3, 32.3)) // M
                .cubic_curve_by((1.0, 0.6, 3.2, 1.6, 5.6, 1.6)) // c
                .cubic_curve_by((4.4, 0.0, 5.7, -2.8, 5.7, -4.9)) // c
                .cubic_curve_by((0.0, -3.5, -3.2, -5.0, -6.5, -5.0)) // c
                .horizontal_line_by(-1.9) // h
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
                .line_to((16.3, 32.3)) // L
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        4 => {
            let data = Data::new()
                .move_to((26.0, 36.2)) // M
                .vertical_line_by(-6.8) // v
                .horizontal_line_to(14.4) // H
                .vertical_line_by(-2.2) // v
                .line_by((11.2, -16.0)) // l
                .horizontal_line_by(3.7) // h
                .vertical_line_by(15.6) // v
                .horizontal_line_by(3.5) // h
                .vertical_line_by(2.7) // v
                .horizontal_line_by(-3.5) // h
                .vertical_line_by(6.8) // v
                .horizontal_line_to(26.0) // H
                .close() // z
                .move_to((26.0, 26.7)) // M
                .vertical_line_by(-8.4) // v
                .cubic_curve_by((0.0, -1.3, 0.0, -2.6, 0.1, -3.9)) // c
                .horizontal_line_to(26.0) // H
                .cubic_curve_by((-0.8, 1.5, -1.4, 2.6, -2.1, 3.7)) // c
                .line_by((-6.1, 8.5)) // l
                .vertical_line_by(0.1) // v
                .horizontal_line_to(26.0) // H
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        5 => {
            let data = Data::new()
                .move_to((30.5, 13.9)) // M
                .horizontal_line_by(-9.6) // h
                .line_by((-1.0, 6.5)) // l
                .cubic_curve_by((0.6, -0.1, 1.1, -0.2, 2.0, -0.2)) // c
                .cubic_curve_by((1.9, 0.0, 3.9, 0.4, 5.4, 1.4)) // c
                .cubic_curve_by((2.0, 1.1, 3.6, 3.3, 3.6, 6.5)) // c
                .cubic_curve_by((0.0, 4.9, -3.9, 8.6, -9.3, 8.6)) // c
                .cubic_curve_by((-2.7, 0.0, -5.1, -0.8, -6.3, -1.5)) // c
                .line_by((0.9, -2.6)) // l
                .cubic_curve_by((1.0, 0.6, 3.1, 1.4, 5.4, 1.4)) // c
                .cubic_curve_by((3.2, 0.0, 6.0, -2.1, 6.0, -5.4)) // c
                .cubic_curve_by((0.0, -3.2, -2.2, -5.6, -7.2, -5.6)) // c
                .cubic_curve_by((-1.4, 0.0, -2.5, 0.2, -3.5, 0.3)) // c
                .line_by((1.6, -12.1)) // l
                .horizontal_line_by(12.0) // h
                .vertical_line_to(13.9) // V
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        6 => {
            let data = Data::new()
                .move_to((29.8, 13.5)) // M
                .cubic_curve_by((-0.7, 0.0, -1.6, 0.0, -2.5, 0.2)) // c
                .cubic_curve_by((-5.3, 0.9, -8.2, 4.8, -8.7, 8.9)) // c
                .horizontal_line_by(0.1) // h
                .cubic_curve_by((1.2, -1.6, 3.3, -2.9, 6.1, -2.9)) // c
                .cubic_curve_by((4.4, 0.0, 7.6, 3.2, 7.6, 8.1)) // c
                .cubic_curve_by((0.0, 4.6, -3.1, 8.8, -8.3, 8.8)) // c
                .cubic_curve_by((-5.4, 0.0, -8.9, -4.2, -8.9, -10.7)) // c
                .cubic_curve_by((0.0, -4.9, 1.8, -8.8, 4.2, -11.3)) // c
                .cubic_curve_by((2.1, -2.0, 4.9, -3.3, 8.0, -3.7)) // c
                .cubic_curve_by((1.0, -0.2, 1.9, -0.2, 2.5, -0.2)) // c
                .vertical_line_to(13.5) // V
                .close() // z
                .move_to((28.9, 28.0)) // M
                .cubic_curve_by((0.0, -3.6, -2.0, -5.8, -5.2, -5.8)) // c
                .cubic_curve_by((-2.0, 0.0, -3.9, 1.3, -4.9, 3.1)) // c
                .cubic_curve_by((-0.2, 0.4, -0.4, 0.9, -0.4, 1.5)) // c
                .cubic_curve_by((0.1, 4.1, 2.0, 7.2, 5.5, 7.2)) // c
                .cubic_curve_to((26.9, 34.0, 28.9, 31.6, 28.9, 28.0)) // C
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        7 => {
            let data = Data::new()
                .move_to((31.8, 11.1)) // M
                .vertical_line_by(2.2) // v
                .line_to((20.9, 36.2)) // L
                .horizontal_line_by(-3.5) // h
                .line_to((28.3, 14.0)) // L
                .vertical_line_by(-0.1) // v
                .horizontal_line_to(16.0) // H
                .vertical_line_by(-2.8) // v
                .horizontal_line_to(31.8) // H
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        8 => {
            let data = Data::new()
                .move_to((15.2, 29.8)) // M
                .cubic_curve_by((0.0, -3.2, 1.9, -5.4, 5.0, -6.7)) // c
                .line_by((0.0, -0.1)) // l
                .cubic_curve_by((-2.8, -1.3, -4.0, -3.5, -4.0, -5.6)) // c
                .cubic_curve_by((0.0, -4.0, 3.4, -6.7, 7.8, -6.7)) // c
                .cubic_curve_by((4.9, 0.0, 7.3, 3.1, 7.3, 6.2)) // c
                .cubic_curve_by((0.0, 2.1, -1.0, 4.4, -4.1, 5.9)) // c
                .vertical_line_by(0.1) // c
                .cubic_curve_by((3.1, 1.2, 5.1, 3.4, 5.1, 6.5)) // c
                .cubic_curve_by((0.0, 4.4, -3.7, 7.3, -8.5, 7.3)) // c
                .cubic_curve_to((18.4, 36.6, 15.2, 33.5, 15.2, 29.8)) // C
                .close() // z
                .move_to((28.7, 29.7)) // M
                .cubic_curve_by((0.0, -3.1, -2.1, -4.5, -5.5, -5.5)) // c
                .cubic_curve_by((-2.9, 0.8, -4.5, 2.8, -4.5, 5.2)) // c
                .cubic_curve_by((-0.1, 2.6, 1.8, 4.8, 5.0, 4.8)) // c
                .cubic_curve_to((26.8, 34.1, 28.7, 32.3, 28.7, 29.7)) // C
                .close() // z
                .move_to((19.4, 17.1)) // M
                .cubic_curve_by((0.0, 2.5, 1.9, 3.9, 4.8, 4.6)) // c
                .cubic_curve_by((2.2, -0.7, 3.8, -2.3, 3.8, -4.6)) // c
                .cubic_curve_by((0.0, -2.0, -1.2, -4.1, -4.2, -4.1)) // c
                .cubic_curve_to((20.9, 13.1, 19.4, 14.9, 19.4, 17.1)) // C
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        9 => {
            let data = Data::new()
                .move_to((17.5, 33.8)) // M
                .cubic_curve_by((0.7, 0.1, 1.6, 0.0, 2.7, -0.1)) // c
                .cubic_curve_by((2.0, -0.3, 3.8, -1.1, 5.3, -2.4)) // c
                .cubic_curve_by((1.7, -1.5, 2.9, -3.7, 3.3, -6.7)) // c
                .horizontal_line_by(-0.1) // h
                .cubic_curve_by((-1.4, 1.7, -3.4, 2.7, -5.9, 2.7)) // c
                .cubic_curve_by((-4.5, 0.0, -7.4, -3.4, -7.4, -7.7)) // c
                .cubic_curve_by((0.0, -4.8, 3.4, -8.9, 8.6, -8.9)) // c
                .smooth_cubic_curve_by((8.3, 4.2, 8.3, 10.6)) // s
                .cubic_curve_by((0.0, 5.5, -1.9, 9.4, -4.3, 11.8)) // c
                .cubic_curve_by((-1.9, 1.9, -4.6, 3.1, -7.3, 3.4)) // c
                .cubic_curve_by((-1.2, 0.2, -2.3, 0.2, -3.1, 0.2)) // c
                .vertical_line_to(33.8) // V
                .close() // z
                .move_to((18.7, 19.4)) // M
                .cubic_curve_by((0.0, 3.1, 1.9, 5.3, 4.8, 5.3)) // c
                .cubic_curve_by((2.3, 0.0, 4.1, -1.1, 4.9, -2.6)) // c
                .cubic_curve_by((0.2, -0.3, 0.3, -0.7, 0.3, -1.2)) // c
                .cubic_curve_by((0.0, -4.3, -1.6, -7.6, -5.1, -7.6)) // c
                .cubic_curve_to((20.8, 13.2, 18.7, 15.8, 18.7, 19.4)) // C
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        10 => {
            let data1 = Data::new()
                .move_to((13.0, 14.3)) // M
                .line_to((13.0, 14.3)) // L
                .line_by((-4.4, 2.4)) // l
                .line_to((7.9, 14.0)) // L
                .line_by((5.5, -2.9)) // l
                .horizontal_line_by(2.9) // h
                .vertical_line_by(25.1) // v
                .horizontal_line_to(13.0) // H
                .vertical_line_to(14.3) // V
                .close(); // z
            let data2 = Data::new()
                .move_to((42.1, 23.4)) // M
                .cubic_curve_by((0.0, 8.5, -3.2, 13.3, -8.7, 13.3)) // c
                .cubic_curve_by((-4.9, 0.0, -8.2, -4.6, -8.3, -12.9)) // c
                .cubic_curve_by((0.0, -8.4, 3.6, -13.1, 8.7, -13.1)) // c
                .cubic_curve_to((39.1, 10.7, 42.1, 15.4, 42.1, 23.4)) // C
                .close() // z
                .move_to((28.5, 23.8)) // M
                .cubic_curve_by((0.0, 6.5, 2.0, 10.2, 5.1, 10.2)) // c
                .cubic_curve_by((3.5, 0.0, 5.1, -4.1, 5.1, -10.5)) // c
                .cubic_curve_by((0.0, -6.2, -1.6, -10.2, -5.1, -10.2)) // c
                .cubic_curve_to((30.6, 13.3, 28.5, 16.9, 28.5, 23.8)) // C
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
        11 => {
            let data1 = Data::new()
                .move_to((7.8, 17.3)) // M
                .line_to((7.8, 17.3)) // L
                .line_by((-4.4, 2.4)) // l
                .line_to((2.7, 17.0)) // L
                .line_by((5.5, -2.9)) // l
                .horizontal_line_by(2.9) // h
                .vertical_line_by(25.1) // v
                .horizontal_line_to(7.8) // H
                .vertical_line_to(17.3) // V
                .close(); // z
            let data2 = Data::new()
                .move_to((27.7, 17.3)) // M
                .line_to((27.7, 17.3)) // L
                .line_by((-4.4, 2.4)) // l
                .line_to((22.6, 17.0)) // L
                .line_by((5.5, -2.9)) // l
                .horizontal_line_by(2.9) // h
                .vertical_line_by(25.1) // v
                .horizontal_line_by(-3.3) // h
                .vertical_line_to(17.3)
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
        12 => {
            let data1 = Data::new()
                .move_to((7.8, 17.3)) // M
                .line_to((7.8, 17.3)) // L
                .line_by((-4.4, 2.4)) // l
                .line_to((2.7, 17.0)) // L
                .line_by((5.5, -2.9)) // l
                .horizontal_line_by(2.9) // h
                .vertical_line_by(25.1) // v
                .horizontal_line_to(7.8) // H
                .vertical_line_to(17.3) // V
                .close(); // z
            let data2 = Data::new()
                .move_to((20.3, 39.2)) // M
                .vertical_line_by(-2.1) // v
                .line_by((2.7, -2.6)) // l
                .cubic_curve_by((6.4, -6.1, 9.3, -9.3, 9.4, -13.1)) // c
                .cubic_curve_by((0.0, -2.5, -1.2, -4.9, -5.0, -4.9)) // c
                .cubic_curve_by((-2.3, 0.0, -4.2, 1.2, -5.3, 2.1)) // c
                .line_by((-1.1, -2.4)) // l
                .cubic_curve_by((1.7, -1.5, 4.2, -2.6, 7.1, -2.6)) // c
                .cubic_curve_by((5.4, 0.0, 7.7, 3.7, 7.7, 7.3)) // c
                .cubic_curve_by((0.0, 4.6, -3.4, 8.4, -8.7, 13.5)) // c
                .line_by((-2.0, 1.9)) // l
                .vertical_line_by(0.1) // v
                .horizontal_line_by(11.3) // h
                .vertical_line_by(2.8) // v
                .horizontal_line_to(20.3) // H
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
        _ => document = Document::new(),
    }
    document
}

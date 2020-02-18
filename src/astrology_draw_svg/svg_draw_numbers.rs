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
        6 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((24.7, 16.5)) // M
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
                .vertical_line_to(16.5) // V
                .close() // z
                .move_to((23.7, 31.0)) // M
                .cubic_curve_by((0.0, -3.6, -2.0, -5.8, -5.2, -5.8)) // c
                .cubic_curve_by((-2.0, 0.0, -3.9, 1.3, -4.9, 3.1)) // c
                .cubic_curve_by((-0.2, 0.4, -0.4, 0.9, -0.4, 1.5)) // c
                .cubic_curve_by((0.1, 4.1, 2.0, 7.2, 5.5, 7.2)) // c
                .cubic_curve_to((21.8, 37.0, 23.7, 34.6, 23.7, 31.0)) // C
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
        7 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((26.7, 14.1)) // M
                .vertical_line_by(2.2) // v
                .line_to((15.7, 39.2)) // L
                .horizontal_line_by(-3.5) // h
                .line_to((23.1, 17.0)) // L
                .vertical_line_by(-0.1) // v
                .horizontal_line_to(10.8) // H
                .vertical_line_by(-2.8) // v
                .horizontal_line_to(26.7) // H
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
        8 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((10.1, 32.8)) // M
                .cubic_curve_by((0.0, -3.2, 1.9, -5.4, 5.0, -6.7)) // c
                .line_by((0.0, -0.1)) // L
                .cubic_curve_by((-2.8, -1.3, -4.0, -3.5, -4.0, -5.6)) // c
                .cubic_curve_by((0.0, -4.0, 3.4, -6.7, 7.8, -6.7)) // c
                .cubic_curve_by((4.9, 0.0, 7.3, 3.1, 7.3, 6.2)) // c
                .cubic_curve_by((0.0, 2.1, -1.0, 4.4, -4.1, 5.9)) // c
                .vertical_line_by(0.1) // v
                .cubic_curve_by((3.1, 1.2, 5.1, 3.4, 5.1, 6.5)) // c
                .cubic_curve_by((0.0, 4.4, -3.7, 7.3, -8.5, 7.3)) // c
                .cubic_curve_to((13.2, 39.6, 10.1, 36.5, 10.1, 32.8)) // C
                .close() // z
                .move_to((23.6, 32.7)) // M
                .cubic_curve_by((0.0, -3.1, -2.1, -4.5, -5.5, -5.5)) // c
                .cubic_curve_by((-2.9, 0.8, -4.5, 2.8, -4.5, 5.2)) // c
                .cubic_curve_by((-0.1, 2.6, 1.8, 4.8, 5.0, 4.8)) // c
                .cubic_curve_to((21.6, 37.1, 23.6, 35.3, 23.6, 32.7)) // C
                .close() // z
                .move_to((14.2, 20.1)) // M
                .cubic_curve_by((0.0, 2.5, 1.9, 3.9, 4.8, 4.6)) // c
                .cubic_curve_by((2.2, -0.7, 3.8, -2.3, 3.8, -4.6)) // c
                .cubic_curve_by((0.0, -2.0, -1.2, -4.1, -4.2, -4.1)) // c
                .cubic_curve_to((15.8, 16.1, 14.2, 17.9, 14.2, 20.1)) // C
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
        9 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((12.3, 36.8)) // M
                .cubic_curve_by((0.7, 0.1, 1.6, 0.0, 2.7, -0.1)) // c
                .cubic_curve_by((2.0, -0.3, 3.8, -1.1, 5.3, -2.4)) // c
                .cubic_curve_by((1.7, -1.5, 2.9, -3.7, 3.3, -6.7)) // c
                .horizontal_line_by(-0.1) // h
                .cubic_curve_by((-1.4, 1.7, -3.4, 2.7, -5.9, 2.7)) // c
                .cubic_curve_by((-4.5, 0.0, -7.4, -3.4, -7.4, -7.7)) // c
                .cubic_curve_by((0.0, -4.8, 3.4, -8.9, 8.6, -8.9)) // c
                .cubic_curve_by((5.1, 0.0, 8.3, 4.2, 8.3, 10.6)) // c
                .cubic_curve_by((0.0, 5.5, -1.9, 9.4, -4.3, 11.8)) // c
                .cubic_curve_by((-1.9, 1.9, -4.6, 3.1, -7.3, 3.4)) // c
                .cubic_curve_by((-1.2, 0.2, -2.3, 0.2, -3.1, 0.2)) // c
                .vertical_line_to(36.8) // V
                .close() // z
                .move_to((13.6, 22.4)) // M
                .cubic_curve_by((0.0, 3.1, 1.9, 5.3, 4.8, 5.3)) // c
                .cubic_curve_by((2.3, 0.0, 4.1, -1.1, 4.9, -2.6)) // c
                .cubic_curve_by((0.2, -0.3, 0.3, -0.7, 0.3, -1.2)) // c
                .cubic_curve_by((0.0, -4.3, -1.6, -7.6, -5.1, -7.6)) // c
                .cubic_curve_to((15.6, 16.2, 13.6, 18.8, 13.6, 22.4)) // C
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
        10 => {
            size = (50.0, 50.0);
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
                .move_to((37.0, 26.4)) // M
                .cubic_curve_by((0.0, 8.5, -3.2, 13.3, -8.7, 13.3)) // c
                .cubic_curve_by((-4.9, 0.0, -8.2, -4.6, -8.3, -12.9)) // c
                .cubic_curve_by((0.0, -8.4, 3.6, -13.1, 8.7, -13.1)) // c
                .cubic_curve_to((34.0, 13.7, 37.0, 18.4, 37.0, 26.4)) // C
                .close() // z
                .move_to((23.3, 26.8)) // M
                .cubic_curve_by((0.0, 6.5, 2.0, 10.2, 5.1, 10.2)) // c
                .cubic_curve_by((3.5, 0.0, 5.1, -4.1, 5.1, -10.5)) // c
                .cubic_curve_by((0.0, -6.2, -1.6, -10.2, -5.1, -10.2)) // c
                .cubic_curve_to((25.5, 16.3, 23.3, 19.9, 23.3, 26.8)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        11 => {
            size = (50.0, 50.0);
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
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        12 => {
            size = (50.0, 50.0);
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
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        13 => {
            size = (50.0, 50.0);
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
                .move_to((21.1, 35.3)) // M
                .cubic_curve_by((1.0, 0.6, 3.2, 1.6, 5.6, 1.6)) // c
                .cubic_curve_by((4.4, 0.0, 5.7, -2.8, 5.7, -4.9)) // c
                .cubic_curve_by((0.0, -3.5, -3.2, -5.0, -6.5, -5.0)) // c
                .horizontal_line_to(24.0) // H
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
                .line_to((21.1, 35.3)) // L
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        14 => {
            size = (50.0, 50.0);
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
                .move_to((30.8, 39.2)) // M
                .vertical_line_by(-6.8) // v
                .horizontal_line_to(19.1) // H
                .vertical_line_by(-2.2) // v
                .line_by((11.2, -16.0)) // l
                .horizontal_line_to(34.0) // H
                .vertical_line_by(15.6) // v
                .horizontal_line_by(3.5) // h
                .vertical_line_by(2.7) // v
                .horizontal_line_to(34.0) // H
                .vertical_line_by(6.8) // v
                .horizontal_line_to(30.8) // H
                .close() // z
                .move_to((30.8, 29.7)) // M
                .vertical_line_by(-8.4) // v
                .cubic_curve_by((0.0, -1.3, 0.0, -2.6, 0.1, -3.9)) // c
                .horizontal_line_by(-0.1) // h
                .cubic_curve_by((-0.8, 1.5, -1.4, 2.6, -2.1, 3.7)) // c
                .line_by((-6.1, 8.5)) // l
                .vertical_line_by(0.1) // v
                .horizontal_line_to(30.8) // H
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        15 => {
            size = (50.0, 50.0);
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
                .move_to((35.3, 16.9)) // M
                .horizontal_line_by(-9.6) // h
                .line_by((-1.0, 6.5)) // l
                .cubic_curve_by((0.6, -0.1, 1.1, -0.2, 2.0, -0.2)) // c
                .cubic_curve_by((1.9, 0.0, 3.9, 0.4, 5.4, 1.4)) // c
                .cubic_curve_by((2.0, 1.1, 3.6, 3.3, 3.6, 6.5)) // c
                .cubic_curve_by((0.0, 4.9, -3.9, 8.6, -9.3, 8.6)) // c
                .cubic_curve_by((-2.7, 0.0, -5.1, -0.8, -6.3, -1.5)) // c
                .line_by((0.9, -2.6)) // l
                .cubic_curve_by((1.0, 0.6, 3.1, 1.4, 5.4, 1.4)) // c
                .cubic_curve_by((3.2, 0.0, 5.9, -2.1, 5.9, -5.4)) // c
                .cubic_curve_by((0.0, -3.2, -2.2, -5.6, -7.2, -5.6)) // c
                .cubic_curve_by((-1.4, 0.0, -2.5, 0.2, -3.5, 0.3)) // c
                .line_by((1.6, -12.1)) // l
                .horizontal_line_by(12.0) // h
                .vertical_line_to(16.9) // V
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        16 => {
            size = (50.0, 50.0);
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
                .move_to((34.6, 16.5)) // M
                .cubic_curve_by((-0.7, 0.0, -1.6, 0.0, -2.6, 0.2)) // c
                .cubic_curve_by((-5.3, 0.9, -8.2, 4.8, -8.7, 8.9)) // c
                .horizontal_line_by(0.1) // h
                .cubic_curve_by((1.2, -1.6, 3.3, -2.9, 6.1, -2.9)) // c
                .cubic_curve_by((4.4, 0.0, 7.6, 3.2, 7.6, 8.1)) // c
                .cubic_curve_by((0.0, 4.6, -3.1, 8.8, -8.3, 8.8)) // c
                .cubic_curve_by((-5.4, 0.0, -8.9, -4.2, -8.9, -10.7)) // c
                .cubic_curve_by((0.0, -4.9, 1.8, -8.8, 4.2, -11.3)) // c
                .cubic_curve_by((2.1, -2.0, 4.9, -3.3, 8.0, -3.7)) // c
                .cubic_curve_by((1.0, -0.2, 1.9, -0.2, 2.5, -0.2)) // c
                .vertical_line_to(16.5) // V
                .close() // z
                .move_to((33.6, 31.0)) // M
                .cubic_curve_by((0.0, -3.6, -2.0, -5.8, -5.2, -5.8)) // c
                .cubic_curve_by((-2.0, 0.0, -3.9, 1.3, -4.9, 3.1)) // c
                .cubic_curve_by((-0.2, 0.4, -0.4, 0.9, -0.4, 1.5)) // c
                .cubic_curve_by((0.1, 4.1, 2.0, 7.2, 5.5, 7.2)) // c
                .cubic_curve_to((31.7, 37.0, 33.6, 34.6, 33.6, 31.0)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        17 => {
            size = (50.0, 50.0);
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
                .move_to((36.6, 14.1)) // M
                .vertical_line_by(2.2) // v
                .line_to((25.7, 39.2)) // L
                .horizontal_line_by(-3.5) // h
                .line_to((33.0, 17.0)) // L
                .vertical_line_by(-0.1) // v
                .horizontal_line_to(20.7) // H
                .vertical_line_by(-2.8) // v
                .horizontal_line_to(36.6) // H
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        18 => {
            size = (50.0, 50.0);
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
                .move_to((20.0, 32.8)) // M
                .cubic_curve_by((0.0, -3.2, 1.9, -5.4, 5.0, -6.7)) // c
                .line_by((0.0, -0.1)) // l
                .cubic_curve_by((-2.8, -1.3, -4.0, -3.5, -4.0, -5.6)) // c
                .cubic_curve_by((0.0, -4.0, 3.4, -6.7, 7.8, -6.7)) // c
                .cubic_curve_by((4.9, 0.0, 7.3, 3.1, 7.3, 6.2)) // c
                .cubic_curve_by((0.0, 2.1, -1.0, 4.4, -4.1, 5.9)) // c
                .vertical_line_by(0.1) // v
                .cubic_curve_by((3.1, 1.2, 5.1, 3.4, 5.1, 6.5)) // c
                .cubic_curve_by((0.0, 4.4, -3.7, 7.3, -8.5, 7.3)) // c
                .cubic_curve_to((23.1, 39.6, 20.0, 36.5, 20.0, 32.8)) // C
                .close() // z
                .move_to((33.5, 32.7)) // M
                .cubic_curve_by((0.0, -3.1, -2.1, -4.5, -5.5, -5.5)) // c
                .cubic_curve_to((25.0, 28.0, 23.4, 30.0, 23.4, 32.4)) // C
                .cubic_curve_by((-0.1, 2.6, 1.8, 4.8, 5.0, 4.8)) // c
                .cubic_curve_to((31.5, 37.1, 33.5, 35.3, 33.5, 32.7)) // C
                .close() // z
                .move_to((24.1, 20.1)) // M
                .cubic_curve_by((0.0, 2.5, 1.9, 3.9, 4.8, 4.6)) // c
                .cubic_curve_by((2.2, -0.7, 3.8, -2.3, 3.8, -4.6)) // c
                .cubic_curve_by((0.0, -2.0, -1.2, -4.1, -4.2, -4.1)) // c
                .cubic_curve_to((25.7, 16.1, 24.1, 17.9, 24.1, 20.1)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        19 => {
            size = (50.0, 50.0);
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
                .move_to((22.3, 36.8)) // M
                .cubic_curve_by((0.7, 0.1, 1.6, 0.0, 2.7, -0.1)) // c
                .cubic_curve_by((2.0, -0.3, 3.8, -1.1, 5.3, -2.4)) // c
                .cubic_curve_by((1.7, -1.5, 2.9, -3.7, 3.3, -6.7)) // c
                .horizontal_line_by(-0.1) // h
                .cubic_curve_by((-1.4, 1.7, -3.4, 2.7, -5.9, 2.7)) // c
                .cubic_curve_by((-4.5, 0.0, -7.4, -3.4, -7.4, -7.7)) // c
                .cubic_curve_by((0.0, -4.8, 3.4, -8.9, 8.6, -8.9)) // c
                .cubic_curve_by((5.1, 0.0, 8.3, 4.2, 8.3, 10.6)) // c
                .cubic_curve_by((0.0, 5.5, -1.9, 9.4, -4.3, 11.8)) // c
                .cubic_curve_by((-1.9, 1.9, -4.6, 3.1, -7.3, 3.4)) // c
                .cubic_curve_by((-1.2, 0.2, -2.3, 0.2, -3.1, 0.2)) // c
                .vertical_line_to(36.8) // V
                .close() // z
                .move_to((23.5, 22.4)) // M
                .cubic_curve_by((0.0, 3.1, 1.9, 5.3, 4.8, 5.3)) // c
                .cubic_curve_by((2.3, 0.0, 4.1, -1.1, 4.9, -2.6)) // c
                .cubic_curve_by((0.2, -0.3, 0.3, -0.7, 0.3, -1.2)) // c
                .cubic_curve_by((0.0, -4.3, -1.6, -7.6, -5.1, -7.6)) // c
                .cubic_curve_to((25.5, 16.2, 23.5, 18.8, 23.5, 22.4)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        20 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((37.0, 26.4)) // M
                .cubic_curve_by((0.0, 8.5, -3.2, 13.3, -8.7, 13.3)) // c
                .cubic_curve_by((-4.9, 0.0, -8.2, -4.6, -8.3, -12.9)) // c
                .cubic_curve_by((0.0, -8.4, 3.6, -13.1, 8.7, -13.1)) // c
                .cubic_curve_to((34.0, 13.7, 37.0, 18.4, 37.0, 26.4)) // C
                .close() // z
                .move_to((23.3, 26.8)) // M
                .cubic_curve_by((0.0, 6.5, 2.0, 10.2, 5.1, 10.2)) // c
                .cubic_curve_by((3.5, 0.0, 5.1, -4.1, 5.1, -10.5)) // c
                .cubic_curve_by((0.0, -6.2, -1.6, -10.2, -5.1, -10.2)) // c
                .cubic_curve_to((25.5, 16.3, 23.3, 19.9, 23.3, 26.8)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        21 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
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
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        22 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
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
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        23 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((21.1, 35.3)) // M
                .cubic_curve_by((1.0, 0.6, 3.2, 1.6, 5.6, 1.6)) // c
                .cubic_curve_by((4.4, 0.0, 5.7, -2.8, 5.7, -4.9)) // c
                .cubic_curve_by((0.0, -3.5, -3.2, -5.0, -6.5, -5.0)) // c
                .horizontal_line_to(24.0) // H
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
                .line_to((21.1, 35.3)) // L
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        24 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((30.8, 39.2)) // M
                .vertical_line_by(-6.8) // v
                .horizontal_line_to(19.1) // H
                .vertical_line_by(-2.2) // v
                .line_by((11.2, -16.0)) // l
                .horizontal_line_to(34.0) // H
                .vertical_line_by(15.6) // v
                .horizontal_line_by(3.5) // h
                .vertical_line_by(2.7) // v
                .horizontal_line_to(34.0) // H
                .vertical_line_by(6.8) // v
                .horizontal_line_to(30.8) // H
                .close() // z
                .move_to((30.8, 29.7)) // M
                .vertical_line_by(-8.4) // v
                .cubic_curve_by((0.0, -1.3, 0.0, -2.6, 0.1, -3.9)) // c
                .horizontal_line_by(-0.1) // h
                .cubic_curve_by((-0.8, 1.5, -1.4, 2.6, -2.1, 3.7)) // c
                .line_by((-6.1, 8.5)) // l
                .vertical_line_by(0.1) // v
                .horizontal_line_to(30.8) // H
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        25 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((35.3, 16.9)) // M
                .horizontal_line_by(-9.6) // h
                .line_by((-1.0, 6.5)) // l
                .cubic_curve_by((0.6, -0.1, 1.1, -0.2, 2.0, -0.2)) // c
                .cubic_curve_by((1.9, 0.0, 3.9, 0.4, 5.4, 1.4)) // c
                .cubic_curve_by((2.0, 1.1, 3.6, 3.3, 3.6, 6.5)) // c
                .cubic_curve_by((0.0, 4.9, -3.9, 8.6, -9.3, 8.6)) // c
                .cubic_curve_by((-2.7, 0.0, -5.1, -0.8, -6.3, -1.5)) // c
                .line_by((0.9, -2.6)) // l
                .cubic_curve_by((1.0, 0.6, 3.1, 1.4, 5.4, 1.4)) // c
                .cubic_curve_by((3.2, 0.0, 5.9, -2.1, 5.9, -5.4)) // c
                .cubic_curve_by((0.0, -3.2, -2.2, -5.6, -7.2, -5.6)) // c
                .cubic_curve_by((-1.4, 0.0, -2.5, 0.2, -3.5, 0.3)) // c
                .line_by((1.6, -12.1)) // l
                .horizontal_line_by(12.0) // h
                .vertical_line_to(16.9) // V
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        26 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((34.6, 16.5)) // M
                .cubic_curve_by((-0.7, 0.0, -1.6, 0.0, -2.6, 0.2)) // c
                .cubic_curve_by((-5.3, 0.9, -8.2, 4.8, -8.7, 8.9)) // c
                .horizontal_line_by(0.1) // h
                .cubic_curve_by((1.2, -1.6, 3.3, -2.9, 6.1, -2.9)) // c
                .cubic_curve_by((4.4, 0.0, 7.6, 3.2, 7.6, 8.1)) // c
                .cubic_curve_by((0.0, 4.6, -3.1, 8.8, -8.3, 8.8)) // c
                .cubic_curve_by((-5.4, 0.0, -8.9, -4.2, -8.9, -10.7)) // c
                .cubic_curve_by((0.0, -4.9, 1.8, -8.8, 4.2, -11.3)) // c
                .cubic_curve_by((2.1, -2.0, 4.9, -3.3, 8.0, -3.7)) // c
                .cubic_curve_by((1.0, -0.2, 1.9, -0.2, 2.5, -0.2)) // c
                .vertical_line_to(16.5) // V
                .close() // z
                .move_to((33.6, 31.0)) // M
                .cubic_curve_by((0.0, -3.6, -2.0, -5.8, -5.2, -5.8)) // c
                .cubic_curve_by((-2.0, 0.0, -3.9, 1.3, -4.9, 3.1)) // c
                .cubic_curve_by((-0.2, 0.4, -0.4, 0.9, -0.4, 1.5)) // c
                .cubic_curve_by((0.1, 4.1, 2.0, 7.2, 5.5, 7.2)) // c
                .cubic_curve_to((31.7, 37.0, 33.6, 34.6, 33.6, 31.0)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        27 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((36.6, 14.1)) // M
                .vertical_line_by(2.2) // v
                .line_to((25.7, 39.2)) // L
                .horizontal_line_by(-3.5) // h
                .line_to((33.0, 17.0)) // L
                .vertical_line_by(-0.1) // v
                .horizontal_line_to(20.7) // H
                .vertical_line_by(-2.8) // v
                .horizontal_line_to(36.6) // H
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        28 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((20.0, 32.8)) // M
                .cubic_curve_by((0.0, -3.2, 1.9, -5.4, 5.0, -6.7)) // c
                .line_by((0.0, -0.1)) // l
                .cubic_curve_by((-2.8, -1.3, -4.0, -3.5, -4.0, -5.6)) // c
                .cubic_curve_by((0.0, -4.0, 3.4, -6.7, 7.8, -6.7)) // c
                .cubic_curve_by((4.9, 0.0, 7.3, 3.1, 7.3, 6.2)) // c
                .cubic_curve_by((0.0, 2.1, -1.0, 4.4, -4.1, 5.9)) // c
                .vertical_line_by(0.1) // v
                .cubic_curve_by((3.1, 1.2, 5.1, 3.4, 5.1, 6.5)) // c
                .cubic_curve_by((0.0, 4.4, -3.7, 7.3, -8.5, 7.3)) // c
                .cubic_curve_to((23.1, 39.6, 20.0, 36.5, 20.0, 32.8)) // C
                .close() // z
                .move_to((33.5, 32.7)) // M
                .cubic_curve_by((0.0, -3.1, -2.1, -4.5, -5.5, -5.5)) // c
                .cubic_curve_to((25.0, 28.0, 23.4, 30.0, 23.4, 32.4)) // C
                .cubic_curve_by((-0.1, 2.6, 1.8, 4.8, 5.0, 4.8)) // c
                .cubic_curve_to((31.5, 37.1, 33.5, 35.3, 33.5, 32.7)) // C
                .close() // z
                .move_to((24.1, 20.1)) // M
                .cubic_curve_by((0.0, 2.5, 1.9, 3.9, 4.8, 4.6)) // c
                .cubic_curve_by((2.2, -0.7, 3.8, -2.3, 3.8, -4.6)) // c
                .cubic_curve_by((0.0, -2.0, -1.2, -4.1, -4.2, -4.1)) // c
                .cubic_curve_to((25.7, 16.1, 24.1, 17.9, 24.1, 20.1)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        29 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((0.5, 39.2)) // M
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
                .horizontal_line_to(0.5) // H
                .close(); // z
            let data2 = Data::new()
                .move_to((22.3, 36.8)) // M
                .cubic_curve_by((0.7, 0.1, 1.6, 0.0, 2.7, -0.1)) // c
                .cubic_curve_by((2.0, -0.3, 3.8, -1.1, 5.3, -2.4)) // c
                .cubic_curve_by((1.7, -1.5, 2.9, -3.7, 3.3, -6.7)) // c
                .horizontal_line_by(-0.1) // h
                .cubic_curve_by((-1.4, 1.7, -3.4, 2.7, -5.9, 2.7)) // c
                .cubic_curve_by((-4.5, 0.0, -7.4, -3.4, -7.4, -7.7)) // c
                .cubic_curve_by((0.0, -4.8, 3.4, -8.9, 8.6, -8.9)) // c
                .cubic_curve_by((5.1, 0.0, 8.3, 4.2, 8.3, 10.6)) // c
                .cubic_curve_by((0.0, 5.5, -1.9, 9.4, -4.3, 11.8)) // c
                .cubic_curve_by((-1.9, 1.9, -4.6, 3.1, -7.3, 3.4)) // c
                .cubic_curve_by((-1.2, 0.2, -2.3, 0.2, -3.1, 0.2)) // c
                .vertical_line_to(36.8) // V
                .close() // z
                .move_to((23.5, 22.4)) // M
                .cubic_curve_by((0.0, 3.1, 1.9, 5.3, 4.8, 5.3)) // c
                .cubic_curve_by((2.3, 0.0, 4.1, -1.1, 4.9, -2.6)) // c
                .cubic_curve_by((0.2, -0.3, 0.3, -0.7, 0.3, -1.2)) // c
                .cubic_curve_by((0.0, -4.3, -1.6, -7.6, -5.1, -7.6)) // c
                .cubic_curve_to((25.5, 16.2, 23.5, 18.8, 23.5, 22.4)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
        30 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((1.3, 35.3)) // M
                .cubic_curve_by((1.0, 0.6, 3.2, 1.6, 5.6, 1.6)) // c
                .cubic_curve_by((4.4, 0.0, 5.7, -2.8, 5.7, -4.9)) // c
                .cubic_curve_by((0.0, -3.5, -3.2, -5.0, -6.5, -5.0)) // c
                .horizontal_line_to(4.1) // H
                .vertical_line_by(-2.5)
                .horizontal_line_to(6.0) // H
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
                .line_to((1.3, 35.3)) // L
                .close(); // z
            let data2 = Data::new()
                .move_to((37.0, 26.4)) // M
                .cubic_curve_by((0.0, 8.5, -3.2, 13.3, -8.7, 13.3)) // c
                .cubic_curve_by((-4.9, 0.0, -8.2, -4.6, -8.3, -12.9)) // c
                .cubic_curve_by((0.0, -8.4, 3.6, -13.1, 8.7, -13.1)) // c
                .cubic_curve_to((34.0, 13.7, 37.0, 18.4, 37.0, 26.4)) // C
                .close() // z
                .move_to((23.3, 26.8)) // M
                .cubic_curve_by((0.0, 6.5, 2.0, 10.2, 5.1, 10.2)) // c
                .cubic_curve_by((3.5, 0.0, 5.1, -4.1, 5.1, -10.5)) // c
                .cubic_curve_by((0.0, -6.2, -1.6, -10.2, -5.1, -10.2)) // c
                .cubic_curve_to((25.5, 16.3, 23.3, 19.9, 23.3, 26.8)) // C
                .close(); // z
            let data3 = Data::new()
                .move_to((49.7, 17.8)) // M
                .cubic_curve_by((0.0, 3.1, -2.5, 5.2, -5.2, 5.2)) // c
                .cubic_curve_by((-3.0, 0.0, -5.1, -2.3, -5.1, -5.0)) // c
                .cubic_curve_by((0.0, -3.0, 2.3, -5.2, 5.1, -5.2)) // c
                .cubic_curve_to((47.8, 12.7, 49.7, 15.1, 49.7, 17.8)) // C
                .close() // z
                .move_to((41.6, 17.9)) // M
                .cubic_curve_by((0.0, 1.8, 1.3, 3.2, 3.0, 3.2)) // c
                .cubic_curve_by((1.7, 0.0, 3.1, -1.4, 3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.4, -0.8, -3.2, -3.1, -3.2)) // c
                .cubic_curve_to((42.6, 14.5, 41.6, 16.3, 41.6, 17.9)) // C
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
            let path3 = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data3);
            let group = Group::new().add(path1).add(path2).add(path3);
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

pub fn draw_minute(minute: i16) -> Document {
    let size: (Number, Number);
    let document: Document;
    match minute {
        0 => {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((30.9, 26.4)) // M
                .cubic_curve_by((0.0, 8.5, -3.2, 13.3, -8.7, 13.3)) // c
                .cubic_curve_by((-4.9, 0.0, -8.2, -4.6, -8.3, -12.9)) // c
                .cubic_curve_by((0.0, -8.4, 3.6, -13.1, 8.7, -13.1)) // c
                .cubic_curve_to((27.9, 13.7, 30.9, 18.4, 30.9, 26.4)) // C
                .close() //z
                .move_to((17.2, 26.8)) // M
                .cubic_curve_by((0.0, 6.5, 2.0, 10.2, 5.1, 10.2)) // c
                .cubic_curve_by((3.5, 0.0, 5.1, -4.1, 5.1, -10.5)) // c
                .cubic_curve_by((0.0, -6.2, -1.6, -10.2, -5.1, -10.2)) // c
                .cubic_curve_to((19.4, 16.3, 17.2, 19.9, 17.2, 26.8)) // C
                .close(); // z
            let data2 = Data::new()
                .move_to((39.4, 12.7)) // M
                .cubic_curve_by((-0.9, 3.2, -2.5, 7.4, -3.6, 9.2)) // c
                .line_by((-2.2, 0.3)) // l
                .cubic_curve_by((0.8, -2.3, 1.9, -6.3, 2.3, -9.1)) // c
                .line_to((39.4, 12.7)) // L
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

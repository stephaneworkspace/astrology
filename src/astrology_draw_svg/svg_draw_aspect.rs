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
use libswe_sys::sweconst::Aspects;
use svg::node::element::path::{Data, Number};
use svg::node::element::{Ellipse, Group, Line, Path, Rectangle};
use svg::Document;
pub const ASPECT_SIZE: Number = 50.0;

pub fn draw_aspect(aspect: Aspects) -> Document {
    let size: (Number, Number) = (ASPECT_SIZE, ASPECT_SIZE);
    let document: Document;
    //let color: String =
    //    format!("#{:06X}", Bodies::EclNut.object_color() as i32);
    match aspect {
        Aspects::Conjunction => {
            let data = Data::new()
                .move_to((28.5, 20.9)) // M
                .cubic_curve_by((5.8, 5.5, 6.0, 14.7, 0.5, 20.4)) // c
                .smooth_cubic_curve_by((-14.7, 6.0, -20.4, 0.5)) // s
                .smooth_cubic_curve_by((-6.0, -14.7, -0.5, -20.4)) // s
                .smooth_cubic_curve_to((22.7, 15.4, 28.5, 20.9)) // S
                .line_by((18.1, -18.0));
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Aspects::Opposition => {
            let data = Data::new()
                .move_to((20.6, 29.0)) // M
                .cubic_curve_by((3.7, 3.5, 3.8, 9.4, 0.3, 13.1)) // c
                .smooth_cubic_curve_by((-9.4, 3.8, -13.1, 0.3)) // s
                .smooth_cubic_curve_to((4.0, 33.0, 7.6, 29.3)) // S
                .smooth_cubic_curve_to((16.9, 25.5, 20.6, 29.0)) // S
                .line_by((8.5, -8.5)) // l
                .cubic_curve_by((-3.7, -3.5, -3.8, -9.4, -0.3, -13.1)) // c
                .smooth_cubic_curve_by((9.4, -3.8, 13.1, -0.3)) // s
                .cubic_curve_by((3.7, 3.5, 3.8, 9.4, 0.3, 13.1)) // c
                .cubic_curve_by((-3.5, 3.7, -9.4, 3.8, -13.1, 0.3)) // c
                .line_by((0.0, 0.0));
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Aspects::Trine => {
            let data = Data::new()
                .move_to((5.5, 46.0)) // M
                .horizontal_line_by(38.8) // h
                .line_to((24.9, 7.2)) // L
                .line_to((5.5, 46.0)) // L
                .close(); // z
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Aspects::Square => {
            let rect = Rectangle::new()
                .set("x", 4.6)
                .set("y", 4.2)
                .set("width", 40.6)
                .set("height", 41.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(rect);
        },
        Aspects::Sextile => {
            let data = Data::new()
                .move_to((48.0, 24.7)) // M
                .horizontal_line_to(1.8) // H
                .move_to((13.3, 4.8)) // M
                .line_by((23.1, 39.9)) // l
                .move_to((36.4, 4.8)) // M
                .line_to((13.3, 44.7)); // L
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Aspects::Inconjunction => {
            let line1 = Line::new()
                .set("x1", 48.0)
                .set("y1", 23.0)
                .set("x2", 2.0)
                .set("y2", 23.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            let line2 = Line::new()
                .set("x1", 25.0)
                .set("y1", 23.0)
                .set("x2", 46.0)
                .set("y2", 36.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            let line3 = Line::new()
                .set("x1", 25.0)
                .set("y1", 23.0)
                .set("x2", 12.0)
                .set("y2", 33.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(line1)
                .add(line2)
                .add(line3);
        },
        Aspects::Sesquisquare => {
            let line1 = Line::new()
                .set("x1", 17.0)
                .set("y1", 26.0)
                .set("x2", 47.0)
                .set("y2", 26.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            let line2 = Line::new()
                .set("x1", 19.1)
                .set("y1", 26.9)
                .set("x2", 4.0)
                .set("y2", 12.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(line1)
                .add(line2);
        },
        Aspects::Semisquare => {
            let line1 = Line::new()
                .set("x1", 2.0)
                .set("y1", 26.0)
                .set("x2", 48.0)
                .set("y2", 26.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            let line2 = Line::new()
                .set("x1", 28.0)
                .set("y1", 4.0)
                .set("x2", 4.2)
                .set("y2", 25.3)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(line1)
                .add(line2);
        },
        Aspects::Semisextile => {
            let line1 = Line::new()
                .set("x1", 2.0)
                .set("y1", 26.0)
                .set("x2", 48.0)
                .set("y2", 26.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            let line2 = Line::new()
                .set("x1", 24.2)
                .set("y1", 25.7)
                .set("x2", 2.8)
                .set("y2", 13.3)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            let line3 = Line::new()
                .set("x1", 27.0)
                .set("y1", 25.6)
                .set("x2", 47.0)
                .set("y2", 14.0)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 6);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(line1)
                .add(line2)
                .add(line3);
        },
    }
    document
}

pub fn no_aspect() -> Document {
    let size: (Number, Number) = (ASPECT_SIZE, ASPECT_SIZE);
    let document: Document;
    //let color: String =
    //    format!("#{:06X}", Bodies::EclNut.object_color() as i32);
    let line = Line::new()
        .set("x1", 2.4)
        .set("y1", 47.6)
        .set("x2", 47.6)
        .set("y2", 2.4)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 6);
    let ellipse = Ellipse::new()
        .set("cx", 24.5)
        .set("cy", 25.0)
        .set("rx", 20.8)
        .set("ry", 21.3)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 6);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(line)
        .add(ellipse);
    document
}

pub fn maj_aspect() -> Document {
    let size: (Number, Number) = (ASPECT_SIZE, ASPECT_SIZE);
    let document: Document;
    //let color: String =
    //    format!("#{:06X}", Bodies::EclNut.object_color() as i32);
    let data1 = Data::new()
        .move_to((21.0, 27.3)) // M
        .cubic_curve_by((-0.1, -2.8, -0.3, -6.2, -0.3, -8.7)) // c
        .horizontal_line_by(-0.1) // h
        .cubic_curve_by((-0.7, 2.4, -1.5, 4.9, -2.5, 7.7)) // c
        .line_by((-3.6, 9.8)) // l
        .horizontal_line_by(-2.0) // h
        .line_by((-3.3, -9.6)) // l
        .cubic_curve_by((-1.0, -2.8, -1.8, -5.5, -2.3, -7.9)) // c
        .horizontal_line_to(6.8) // H
        .cubic_curve_by((-0.1, 2.5, -0.2, 5.9, -0.4, 8.9)) // c
        .line_by((-0.5, 8.7)) // l
        .horizontal_line_to(3.4) // H
        .line_to((4.8, 16.0)) // L
        .horizontal_line_by(3.3) // h
        .line_by((3.4, 9.8)) // l
        .cubic_curve_by((0.8, 2.5, 1.5, 4.7, 2.0, 6.8)) // c
        .horizontal_line_by(0.1) // h
        .cubic_curve_by((0.5, -2.0, 1.2, -4.3, 2.1, -6.8)) // c
        .line_by((3.6, -9.8)) // l
        .horizontal_line_by(3.3) // h
        .line_to((24.0, 36.2)) // L
        .horizontal_line_by(-2.5) // h
        .line_to((21.0, 27.3)) // L
        .close(); // z
    let path1 = Path::new().set("d", data1);
    let data2 = Data::new()
        .move_to((36.2, 36.2)) // M
        .line_to((36.0, 34.4)) // L
        .horizontal_line_by(-0.1) // h
        .cubic_curve_by((-0.8, 1.1, -2.4, 2.2, -4.4, 2.2)) // c
        .cubic_curve_by((-2.9, 0.0, -4.4, -2.1, -4.4, -4.2)) // c
        .cubic_curve_by((0.0, -3.5, 3.1, -5.4, 8.7, -5.4)) // c
        .vertical_line_by(-0.3) // v
        .cubic_curve_by((0.0, -1.2, -0.3, -3.4, -3.3, -3.4)) // c
        .cubic_curve_by((-1.3, 0.0, -2.8, 0.4, -3.8, 1.1)) // c
        .line_by((-0.6, -1.7)) // l
        .cubic_curve_by((1.2, -0.8, 2.9, -1.3, 4.8, -1.3)) // c
        .cubic_curve_by((4.4, 0.0, 5.5, 3.0, 5.5, 5.9)) // c
        .vertical_line_by(5.4) // v
        .cubic_curve_by((0.0, 1.3, 0.1, 2.5, 0.2, 3.5)) // c
        .horizontal_line_to(36.2) // H
        .close() // z
        .move_to((35.8, 28.8)) // M
        .cubic_curve_by((-2.9, -0.1, -6.2, 0.5, -6.2, 3.3)) // c
        .cubic_curve_by((0.0, 1.7, 1.1, 2.5, 2.5, 2.5)) // c
        .cubic_curve_by((1.9, 0.0, 3.1, -1.2, 3.5, -2.4)) // c
        .cubic_curve_by((0.1, -0.3, 0.2, -0.6, 0.2, -0.8)) // c
        .vertical_line_to(28.8)
        .close(); // z
    let path2 = Path::new().set("d", data2);
    let data3 = Data::new()
        .move_to((39.1, 40.4)) // M
        .cubic_curve_by((1.2, -0.1, 2.2, -0.4, 2.9, -1.1)) // c
        .cubic_curve_by((0.7, -0.8, 1.0, -2.0, 1.0, -5.5)) // c
        .vertical_line_to(21.7) // V
        .horizontal_line_by(2.6) // h
        .vertical_line_by(13.2) // v
        .cubic_curve_by((0.0, 2.8, -0.5, 4.6, -1.7, 6.0)) // c
        .cubic_curve_by((-1.2, 1.2, -3.1, 1.6, -4.5, 1.6)) // c
        .line_to((39.1, 40.4)) // L
        .close() // z
        .move_to((45.9, 17.6)) // M
        .cubic_curve_by((0.0, 0.9, -0.6, 1.6, -1.7, 1.6)) // c
        .cubic_curve_by((-1.0, 0.0, -1.6, -0.8, -1.6, -1.6)) // c
        .cubic_curve_by((0.0, -0.9, 0.7, -1.6, 1.7, -1.6)) // c
        .cubic_curve_to((45.3, 15.9, 45.9, 16.7, 45.9, 17.6)) // C
        .close(); // z
    let path3 = Path::new().set("d", data3);
    let group = Group::new().add(path1).add(path2).add(path3);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

pub fn min_aspect() -> Document {
    let size: (Number, Number) = (ASPECT_SIZE, ASPECT_SIZE);
    let document: Document;
    //let color: String =
    //    format!("#{:06X}", Bodies::EclNut.object_color() as i32);
    let data1 = Data::new()
        .move_to((19.9, 27.3)) // M
        .cubic_curve_by((-0.1, -2.8, -0.3, -6.2, -0.3, -8.7)) // c
        .horizontal_line_by(-0.1) // h
        .cubic_curve_to((18.9, 21.0, 18.0, 23.5, 17.0, 26.3)) // C
        .line_by((-3.6, 9.8)) // l
        .horizontal_line_by(-2.0) // h
        .line_by((-3.3, -9.6)) // l
        .cubic_curve_by((-1.0, -2.8, -1.8, -5.5, -2.3, -7.9)) // c
        .horizontal_line_to(5.8) // H
        .cubic_curve_by((-0.1, 2.5, -0.2, 5.9, -0.4, 8.9)) // c
        .line_by((-0.5, 8.7)) // l
        .horizontal_line_to(2.4) // H
        .line_to((3.8, 16.0)) // L
        .horizontal_line_by(3.3) // h
        .line_by((3.5, 9.8)) // l
        .cubic_curve_by((0.8, 2.5, 1.5, 4.7, 2.0, 6.8)) // c
        .horizontal_line_by(0.1) // h
        .cubic_curve_by((0.5, -2.0, 1.2, -4.3, 2.1, -6.8)) // c
        .line_by((3.6, -9.8)) // l
        .horizontal_line_by(3.3) // h
        .line_to((23.0, 36.2)) // L
        .horizontal_line_by(-2.5) // h
        .line_to((19.9, 27.3)) // L
        .close(); // z
    let path1 = Path::new().set("d", data1);
    let data2 = Data::new()
        .move_to((30.2, 17.6)) // M
        .cubic_curve_by((0.0, 0.9, -0.6, 1.6, -1.7, 1.6)) // c
        .cubic_curve_by((-0.9, 0.0, -1.6, -0.7, -1.6, -1.6)) // c
        .cubic_curve_by((0.0, -0.9, 0.7, -1.6, 1.6, -1.6)) // c
        .cubic_curve_to((29.6, 15.9, 30.2, 16.7, 30.2, 17.6)) // C
        .close() // z
        .move_to((27.3, 36.2)) // M
        .vertical_line_to(21.7) // V
        .horizontal_line_by(2.6) // h
        .vertical_line_by(14.5) // v
        .horizontal_line_to(27.3) // H
        .close(); // z
    let path2 = Path::new().set("d", data2);
    let data3 = Data::new()
        .move_to((34.3, 25.6)) // M
        .cubic_curve_by((0.0, -1.5, 0.0, -2.7, -0.1, -3.9)) // c
        .horizontal_line_by(2.3) // h
        .line_by((0.1, 2.4)) // l
        .horizontal_line_by(0.1) // h
        .cubic_curve_by((0.7, -1.4, 2.4, -2.7, 4.8, -2.7)) // c
        .cubic_curve_by((2.0, 0.0, 5.1, 1.2, 5.1, 6.2)) // c
        .vertical_line_by(8.7) // v
        .horizontal_line_to(44.0) // H
        .vertical_line_by(-8.4) // v
        .cubic_curve_by((0.0, -2.3, -0.9, -4.3, -3.4, -4.3)) // c
        .cubic_curve_by((-1.7, 0.0, -3.1, 1.2, -3.5, 2.7)) // c
        .cubic_curve_by((-0.1, 0.3, -0.2, 0.8, -0.2, 1.2)) // c
        .vertical_line_by(8.7) // v
        .horizontal_line_by(-2.6) // h
        .vertical_line_to(25.6) // V
        .close(); // z
    let path3 = Path::new().set("d", data3);
    let group = Group::new().add(path1).add(path2).add(path3);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

pub fn all_aspect() -> Document {
    let size: (Number, Number) = (ASPECT_SIZE, ASPECT_SIZE);
    let document: Document;
    //let color: String =
    //    format!("#{:06X}", Bodies::EclNut.object_color() as i32);
    let data1 = Data::new()
        .move_to((14.0, 29.8)) // M
        .line_by((-2.1, 6.4)) // l
        .horizontal_line_to(9.2) // H
        .line_to((16.1, 16.0)) // L
        .horizontal_line_by(3.1) // h
        .line_by((6.9, 20.2)) // l
        .horizontal_line_by(-2.8) // h
        .line_by((-2.2, -6.4)) // l
        .horizontal_line_to(14.0) // H
        .close() // z
        .move_to((20.7, 27.8)) // M
        .line_by((-2.0, -5.8)) // l
        .cubic_curve_by((-0.5, -1.3, -0.8, -2.5, -1.1, -3.7)) // c
        .horizontal_line_by(-0.1) // h
        .cubic_curve_by((-0.3, 1.2, -0.6, 2.4, -1.0, 3.7)) // c
        .line_by((-2.0, 5.9)) // l
        .horizontal_line_to(20.7) // H
        .close(); // z
    let path1 = Path::new().set("d", data1);
    let data2 = Data::new()
        .move_to((28.9, 14.9)) // M
        .horizontal_line_by(2.6) // h
        .vertical_line_by(21.3) // v
        .horizontal_line_by(-2.6) // h
        .vertical_line_to(14.9) // V
        .close(); // z
    let path2 = Path::new().set("d", data2);
    let data3 = Data::new()
        .move_to((36.0, 14.9)) // M
        .horizontal_line_by(2.6) // h
        .vertical_line_by(21.3) // v
        .horizontal_line_to(36.0) // H
        .vertical_line_to(14.9) // V
        .close(); // z
    let path3 = Path::new().set("d", data3);
    let group = Group::new().add(path1).add(path2).add(path3);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

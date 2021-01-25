/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (https://github.com/stephaneworkspace)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * This software uses the swiss ephemeris which is licensed GPL.
 *
 * Therefore, if you want to this source in your commercial projects, you must
 * adhere to the GPL license or buy a Swiss Ephemeris commercial license.
 */
extern crate strum;
use libswe_sys::sweconst::{Aspects, Language, Theme};
use svg::node::element::path::{Data, Number};
use svg::node::element::{Group, Line, Path, Rectangle};
use svg::Document;

pub const ASPECT_SIZE: Number = 50.0;

/// Draw aspects text
pub fn aspects_draw(
    aspect: Aspects,
    _theme: Theme,
    _lang: Language,
) -> Document {
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

/// Draw text for no aspect
pub fn aspects_no_aspect(_theme: Theme, lang: Language) -> Document {
    let size: (Number, Number) = (ASPECT_SIZE, ASPECT_SIZE);
    let document: Document;
    //let color: String =
    //    format!("#{:06X}", Bodies::EclNut.object_color() as i32);
    /*let line = Line::new()
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
        .add(ellipse);*/
    match lang {
        Language::English => {
            let data1 = Data::new()
                .move_to((8.9, 36.2)) // M
                .vertical_line_to(16.0) // V
                .horizontal_line_by(2.9) // h
                .line_by((6.5, 10.2)) // l
                .cubic_curve_by((1.5, 2.4, 2.7, 4.5, 3.6, 6.6)) //
                .line_by((0.1, 0.0)) // l
                .cubic_curve_by((-0.2, -2.7, -0.3, -5.2, -0.3, -8.3)) // c
                .vertical_line_to(16.0) // V
                .horizontal_line_to(24.0) // H
                .vertical_line_by(20.2) // v
                .horizontal_line_by(-2.6) // h
                .line_to((15.0, 25.9)) // L
                .cubic_curve_by((-1.4, -2.2, -2.8, -4.6, -3.8, -6.7)) // c
                .line_by((-0.1, 0.0)) // l
                .cubic_curve_by((0.1, 2.6, 0.2, 5.0, 0.2, 8.3)) // c
                .vertical_line_by(8.6) // v
                .horizontal_line_to(8.9) // H
                .close(); // z
            let path1 = Path::new().set("d", data1);
            let data2 = Data::new()
                .move_to((41.6, 28.8)) // M
                .cubic_curve_by((0.0, 5.4, -3.7, 7.7, -7.2, 7.7)) // c
                .cubic_curve_by((-3.9, 0.0, -7.0, -2.9, -7.0, -7.5)) // c
                .cubic_curve_by((0.0, -4.9, 3.2, -7.7, 7.2, -7.7)) // c
                .cubic_curve_to((38.8, 21.3, 41.6, 24.4, 41.6, 28.8)) // C
                .close() // z
                .move_to((30.1, 29.0))
                .cubic_curve_by((0.0, 3.2, 1.8, 5.6, 4.4, 5.6)) // c
                .cubic_curve_by((2.5, 0.0, 4.4, -2.4, 4.4, -5.6)) // c
                .cubic_curve_by((0.0, -2.5, -1.2, -5.6, -4.4, -5.6)) // c
                .cubic_curve_to((31.5, 23.3, 30.1, 26.2, 30.1, 29.0)) // C
                .close(); // z
            let path2 = Path::new().set("d", data2);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path1)
                .add(path2);
        },
        Language::French => {
            let data1 = Data::new()
                .move_to((3.7, 27.4)) // M
                .line_by((-1.3, 3.8)) // l
                .horizontal_line_to(0.8) // H
                .line_by((4.1, -12.1)) // l
                .horizontal_line_by(1.9) // h
                .line_by((4.1, 12.1)) // l
                .horizontal_line_to(9.3) // H
                .line_to((8.0, 27.4)) // L
                .horizontal_line_to(3.7) // H
                .close() // z
                .move_to((7.7, 26.2)) // M
                .line_by((-1.2, -3.5)) // l
                .cubic_curve_by((-0.3, -0.8, -0.4, -1.5, -0.6, -2.2)) // c
                .horizontal_line_by(0.0) // h
                .cubic_curve_by((-0.2, 0.7, -0.4, 1.5, -0.6, 2.2)) // c
                .line_to((4.0, 26.2)) // L
                .horizontal_line_to(7.7) // H
                .close(); // z
            let path1 = Path::new().set("d", data1);
            let data2 = Data::new()
                .move_to((19.7, 28.8)) // M
                .cubic_curve_by((0.0, 0.9, 0.0, 1.7, 0.1, 2.4)) // c
                .horizontal_line_by(-1.4) // h
                .line_by((-0.1, -1.4)) // l
                .horizontal_line_by(0.0) // h
                .cubic_curve_by((-0.4, 0.7, -1.3, 1.6, -2.9, 1.6)) // c
                .cubic_curve_by((-1.4, 0.0, -3.0, -0.8, -3.0, -3.8)) // c
                .vertical_line_by(-5.1) // v
                .horizontal_line_to(14.0) // H
                .vertical_line_by(4.8) // v
                .cubic_curve_by((0.0, 1.7, 0.5, 2.8, 1.9, 2.8)) // c
                .cubic_curve_by((1.1, 0.0, 1.8, -0.7, 2.1, -1.4)) // c
                .cubic_curve_by((0.1, -0.2, 0.1, -0.5, 0.1, -0.8)) // c
                .vertical_line_by(-5.3) // v
                .horizontal_line_by(1.6) // h
                .vertical_line_to(28.8) // V
                .close(); // z
            let path2 = Path::new().set("d", data2);
            let data3 = Data::new()
                .move_to((28.6, 30.9)) // M
                .cubic_curve_by((-0.4, 0.2, -1.3, 0.5, -2.5, 0.5)) // c
                .cubic_curve_by((-2.6, 0.0, -4.3, -1.8, -4.3, -4.4)) // c
                .cubic_curve_by((0.0, -2.7, 1.8, -4.6, 4.7, -4.6)) // c
                .cubic_curve_by((0.9, 0.0, 1.8, 0.2, 2.2, 0.5)) // c
                .line_to((28.3, 24.0)) // L
                .cubic_curve_by((-0.4, -0.2, -1.0, -0.4, -1.8, -0.4)) // c
                .cubic_curve_by((-2.0, 0.0, -3.1, 1.5, -3.1, 3.3)) // c
                .cubic_curve_by((0.0, 2.0, 1.3, 3.3, 3.0, 3.3)) // c
                .cubic_curve_by((0.9, 0.0, 1.5, -0.2, 1.9, -0.4)) // c
                .line_to((28.6, 30.9)) // L
                .close(); // z
            let path3 = Path::new().set("d", data3);
            let data4 = Data::new()
                .move_to((37.7, 28.8)) // M
                .cubic_curve_by((0.0, 0.9, 0.0, 1.7, 0.1, 2.4)) // c
                .horizontal_line_by(-1.4) // h
                .line_by((-0.1, -1.4)) // l
                .horizontal_line_by(0.0) // h
                .cubic_curve_by((-0.4, 0.7, -1.3, 1.6, -2.9, 1.6)) // c
                .cubic_curve_by((-1.4, 0.0, -3.0, -0.8, -3.0, -3.8)) // c
                .vertical_line_by(-5.1) // v
                .horizontal_line_to(32.0) // H
                .vertical_line_by(4.8) // v
                .cubic_curve_by((0.0, 1.7, 0.5, 2.8, 1.9, 2.8)) // c
                .cubic_curve_by((1.1, 0.0, 1.8, -0.7, 2.1, -1.4)) // c
                .cubic_curve_by((0.1, -0.2, 0.1, -0.5, 0.1, -0.8)) // c
                .vertical_line_by(-5.3) // v
                .horizontal_line_by(1.6) // h
                .vertical_line_to(28.8) // V
                .close(); // z
            let path4 = Path::new().set("d", data4);
            let data5 = Data::new()
                .move_to((40.4, 24.8)) // M
                .cubic_curve_by((0.0, -0.9, 0.0, -1.6, -0.1, -2.4)) // c
                .horizontal_line_by(1.4) // h
                .line_by((0.1, 1.4)) // l
                .horizontal_line_by(0.0) // h
                .cubic_curve_by((0.4, -0.8, 1.4, -1.6, 2.9, -1.6)) // c
                .cubic_curve_by((1.2, 0.0, 3.1, 0.7, 3.1, 3.7)) // c
                .vertical_line_by(5.2) // v
                .horizontal_line_by(-1.6) // h
                .vertical_line_by(-5.0) // v
                .cubic_curve_by((0.0, -1.4, -0.5, -2.6, -2.0, -2.6)) // c
                .cubic_curve_by((-1.0, 0.0, -1.9, 0.7, -2.1, 1.6)) // c
                .cubic_curve_by((-0.1, 0.2, -0.1, 0.5, -0.1, 0.7)) // c
                .vertical_line_by(5.2) // v
                .horizontal_line_by(-1.6) // h
                .vertical_line_by(24.8) // v
                .close(); // z
            let path5 = Path::new().set("d", data5);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path1)
                .add(path2)
                .add(path3)
                .add(path4)
                .add(path5);
        },
    }
    document
}

/// Draw text fort majors aspects
pub fn aspects_maj_aspects(_theme: Theme, _lang: Language) -> Document {
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

/// Draw text for minors aspects
pub fn aspects_min_aspects(_theme: Theme, _lang: Language) -> Document {
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

/// Draw text for all aspects
pub fn aspects_all_aspects(_theme: Theme, lang: Language) -> Document {
    let size: (Number, Number) = (ASPECT_SIZE, ASPECT_SIZE);
    let document: Document;
    //let color: String =
    //    format!("#{:06X}", Bodies::EclNut.object_color() as i32);
    match lang {
        Language::English => {
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
        },
        Language::French => {
            let data1 = Data::new()
                .move_to((6.5, 18.8)) // M
                .horizontal_line_to(1.6) // H
                .vertical_line_to(17.0) // V
                .horizontal_line_by(12.0) // h
                .vertical_line_by(1.8) // v
                .horizontal_line_to(8.6) // H
                .vertical_line_by(14.4) // v
                .horizontal_line_to(6.5) // H
                .vertical_line_to(18.8) // V
                .close(); // z
            let path1 = Path::new().set("d", data1);
            let data2 = Data::new()
                .move_to((24.1, 27.3)) // M
                .cubic_curve_by((0.0, 4.3, -3.0, 6.2, -5.8, 6.2)) // c
                .cubic_curve_by((-3.1, 0.0, -5.6, -2.3, -5.6, -6.0)) // c
                .cubic_curve_by((0.0, -3.9, 2.5, -6.2, 5.8, -6.2)) // c
                .cubic_curve_to((21.9, 21.3, 24.1, 23.7, 24.1, 27.3)) // C
                .close() // z
                .move_to((14.9, 27.4)) // M
                .cubic_curve_by((0.0, 2.5, 1.5, 4.5, 3.5, 4.5)) // c
                .cubic_curve_by((2.0, 0.0, 3.5, -1.9, 3.5, -4.5)) // c
                .cubic_curve_by((0.0, -2.0, -1.0, -4.5, -3.5, -4.5)) // c
                .smooth_cubic_curve_to((14.9, 25.2, 14.9, 27.4)) // S
                .close(); // z
            let path2 = Path::new().set("d", data2);
            let data3 = Data::new()
                .move_to((36.5, 30.0)) // M
                .cubic_curve_by((0.0, 1.2, 0.0, 2.3, 0.1, 3.2)) // c
                .horizontal_line_by(-1.9) // h
                .line_by((-0.1, -1.9)) // l
                .horizontal_line_by(0.0) // h
                .cubic_curve_by((-0.6, 0.9, -1.8, 2.2, -3.8, 2.2)) // c
                .cubic_curve_by((-1.8, 0.0, -4.0, -1.0, -4.0, -5.1)) // c
                .vertical_line_by(-6.8) // v
                .horizontal_line_by(2.1) // h
                .vertical_line_to(28.0) // V
                .cubic_curve_by((0.0, 2.2, 0.7, 3.7, 2.6, 3.7)) // c
                .cubic_curve_by((1.4, 0.0, 2.4, -1.0, 2.8, -1.9)) // c
                .cubic_curve_by((0.1, -0.3, 0.2, -0.7, 0.2, -1.1)) // c
                .vertical_line_by(-7.1)
                .horizontal_line_by(2.1) // h
                .vertical_line_to(30.0) // V
                .close(); // z
            let path3 = Path::new().set("d", data3);
            let data4 = Data::new()
                .move_to((39.7, 31.0)) // M
                .cubic_curve_by((0.6, 0.4, 1.7, 0.8, 2.8, 0.8)) // c
                .cubic_curve_by((1.5, 0.0, 2.3, -0.8, 2.3, -1.7)) // c
                .cubic_curve_by((0.0, -1.0, -0.6, -1.6, -2.2, -2.1)) // c
                .cubic_curve_by((-2.1, -0.7, -3.1, -1.9, -3.1, -3.3)) // c
                .cubic_curve_by((0.0, -1.9, 1.5, -3.4, 4.0, -3.4)) // c
                .cubic_curve_by((1.2, 0.0, 2.2, 0.3, 2.9, 0.7)) // c
                .line_by((-0.5, 1.5)) // l
                .cubic_curve_by((-0.5, -0.3, -1.3, -0.7, -2.4, -0.7)) // c
                .cubic_curve_by((-1.2, 0.0, -1.9, 0.7, -1.9, 1.6)) // c
                .cubic_curve_by((0.0, 1.0, 0.7, 1.4, 2.2, 2.0)) // c
                .cubic_curve_by((2.0, 0.8, 3.0, 1.8, 3.0, 3.5)) // c
                .cubic_curve_by((0.0, 2.0, -1.6, 3.5, -4.3, 3.5)) // c
                .cubic_curve_by((-1.3, 0.0, -2.4, -0.3, -3.3, -0.8)) // c
                .line_to((39.7, 31.0)) // L
                .close(); // z
            let path4 = Path::new().set("d", data4);
            let group =
                Group::new().add(path1).add(path2).add(path3).add(path4);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        },
    }
    document
}

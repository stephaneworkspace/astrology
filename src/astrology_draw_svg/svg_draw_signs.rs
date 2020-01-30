extern crate strum;
use libswe_sys::sweconst::Signs;
use svg::node::element::path::{Data, Number};
use svg::node::element::Path;
use svg::Document;

pub fn draw_sign(sign: Signs) -> Document {
    let size: (Number, Number);
    let document: Document;
    match sign {
        Signs::Aries => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((22.7, 48.8)) // M
                .vertical_line_to(45.0) // V
                .cubic_curve_by((0.0, -3.0, -0.6, -8.3, -1.9, -15.9)) // c
                .cubic_curve_by((-0.6, -3.6, -1.5, -7.4, -2.9, -11.2)) // c
                .cubic_curve_by((-1.4, -4.0, -2.8, -7.1, -4.2, -9.3)) // c
                .cubic_curve_by((-1.1, -1.7, -2.5, -2.6, -4.1, -2.6)) // c
                .cubic_curve_by((-1.8, 0.0, -3.0, 0.7, -3.8, 2.1)) // c
                .cubic_curve_by((-0.6, 1.3, -1.0, 2.7, -1.0, 4.2)) // c
                .cubic_curve_by((0.0, 3.2, 1.1, 6.0, 3.4, 8.6)) // c
                .horizontal_line_to(2.9) // H
                .cubic_curve_by((-1.8, -2.8, -2.8, -5.8, -2.8, -8.9)) // c
                .cubic_curve_to((0.1, 8.5, 1.0, 6.0, 2.8, 4.1)) // C
                .cubic_curve_by((1.8, -1.9, 4.0, -2.9, 6.6, -2.9)) // c
                .cubic_curve_by((3.3, 0.0, 5.9, 1.4, 7.8, 4.1)) // c
                .cubic_curve_by((2.1, 3.1, 3.9, 6.9, 5.3, 11.3)) // c
                .cubic_curve_by((1.0, 3.2, 1.8, 6.8, 2.5, 10.7)) // c
                .cubic_curve_by((0.7, -3.9, 1.5, -7.5, 2.5, -10.7)) // c
                .cubic_curve_by((1.3, -4.3, 3.1, -8.1, 5.3, -11.3)) // c
                .cubic_curve_by((1.9, -2.7, 4.5, -4.1, 7.8, -4.1)) // c
                .cubic_curve_by((2.6, 0.0, 4.8, 1.0, 6.6, 2.9)) // c
                .cubic_curve_by((1.7, 1.9, 2.6, 4.4, 2.6, 7.7)) // c
                .cubic_curve_by((0.0, 3.1, -0.9, 6.1, -2.8, 8.9)) // c
                .horizontal_line_by(-5.3) // h
                .cubic_curve_by((2.3, -2.6, 3.4, -5.4, 3.4, -8.6)) // c
                .cubic_curve_by((0.0, -1.5, -0.3, -2.9, -1.0, -4.2)) // c
                .cubic_curve_by((-0.7, -1.4, -2.0, -2.1, -3.8, -2.1)) // c
                .cubic_curve_by((-1.6, 0.0, -2.9, 0.9, -4.1, 2.6)) // c
                .cubic_curve_by((-1.5, 2.3, -2.9, 5.4, -4.2, 9.3)) // c
                .cubic_curve_by((-1.3, 3.9, -2.3, 7.6, -2.9, 11.2)) // c
                .cubic_curve_to((28.0, 36.7, 27.3, 42.0, 27.3, 45.0)) // C
                .vertical_line_by(3.8) // c
                .horizontal_line_to(22.7); // H
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Taurus => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((10.6, 12.6)) // M
                .cubic_curve_to((10.0, 11.3, 9.2, 10.0, 8.2, 9.0)) // C
                .smooth_cubic_curve_to((6.0, 7.2, 4.7, 6.7)) // S
                .cubic_curve_to((3.3, 6.1, 1.9, 5.9, 0.5, 5.9)) // C
                .vertical_line_to(1.2) // V
                .cubic_curve_by((2.0, 0.0, 4.1, 0.4, 5.9, 1.1)) // c
                .cubic_curve_by((1.9, 0.9, 3.6, 2.0, 5.1, 3.5)) // c
                .cubic_curve_by((1.5, 1.4, 2.6, 3.2, 3.4, 5.1)) // c
                .cubic_curve_by((0.6, 1.3, 1.4, 2.5, 2.4, 3.6)) // c
                .cubic_curve_by((1.0, 1.0, 2.2, 1.8, 3.5, 2.4)) // c
                .cubic_curve_by((1.3, 0.6, 2.7, 0.8, 4.2, 0.8)) // c
                .cubic_curve_by((1.4, 0.0, 2.8, -0.3, 4.2, -0.8)) // c
                .cubic_curve_by((1.3, -0.6, 2.5, -1.4, 3.5, -2.4)) // c
                .smooth_cubic_curve_by((1.8, -2.2, 2.4, -3.6)) // s
                .cubic_curve_by((0.8, -1.9, 2.0, -3.6, 3.4, -5.1)) // c
                .cubic_curve_by((1.5, -1.4, 3.2, -2.6, 5.1, -3.4)) // c
                .cubic_curve_by((1.9, -0.8, 3.9, -1.2, 6, -1.2)) // c
                .vertical_line_by(4.7) // v
                .cubic_curve_by((-1.4, 0.0, -2.8, 0.3, -4.2, 0.8)) // c
                .cubic_curve_to((44.0, 7.3, 42.8, 8.0, 41.8, 9.0)) // C
                .smooth_cubic_curve_by((-1.8, 2.2, -2.4, 3.6)) // s
                .cubic_curve_by((-0.8, 1.9, -2.0, 3.7, -3.4, 5.1)) // c
                .cubic_curve_by((-0.9, 0.8, -1.8, 1.6, -2.8, 2.2)) // c
                .cubic_curve_by((1.0, 0.6, 2.0, 1.4, 2.8, 2.2)) // c
                .cubic_curve_by((3.0, 2.9, 4.6, 6.9, 4.6, 11.1)) // c
                .cubic_curve_by((0.0, 2.0, -0.4, 4.1, -1.2, 5.9)) // c
                .cubic_curve_by((-0.8, 1.9, -2.0, 3.6, -3.4, 5.1)) // c
                .cubic_curve_by((-1.5, 1.4, -3.2, 2.6, -5.1, 3.4)) // c
                .smooth_cubic_curve_to((27.0, 48.8, 25.0, 48.8)) // S
                .smooth_cubic_curve_by((-4.1, -0.3, -5.9, -1.2)) // s
                .cubic_curve_by((-1.9, -0.8, -3.6, -2, -5.1, -3.4)) // c
                .cubic_curve_by((-1.5, -1.5, -2.6, -3.2, -3.4, -5.1)) // c
                .cubic_curve_by((-0.8, -1.9, -1.2, -3.9, -1.1, -5.9)) // c
                .cubic_curve_by((0.0, -2.1, 0.4, -4.1, 1.1, -6)) // c
                .cubic_curve_by((0.8, -1.9, 1.9, -3.7, 3.4, -5.1)) // c
                .cubic_curve_by((0.9, -0.8, 1.8, -1.6, 2.9, -2.2)) // c
                .cubic_curve_by((-1.0, -0.6, -2.0, -1.4, -2.9, -2.2)) // c
                .cubic_curve_to((12.6, 16.3, 11.4, 14.5, 10.6, 12.6)) // C
                .move_to((29.2, 23.1))
                .cubic_curve_by((-2.7, -1.1, -5.7, -1.1, -8.3, 0.0)) // c
                .cubic_curve_by((-1.3, 0.6, -2.5, 1.4, -3.5, 2.4)) // c
                .smooth_cubic_curve_by((-1.8, 2.2, -2.4, 3.6)) // s
                .cubic_curve_by((-0.6, 1.3, -0.8, 2.8, -0.8, 4.2)) // c
                .cubic_curve_by((0.0, 1.4, 0.3, 2.8, 0.8, 4.2)) // c
                .cubic_curve_by((0.6, 1.3, 1.4, 2.6, 2.4, 3.6)) // c
                .smooth_cubic_curve_by((2.2, 1.8, 3.5, 2.4)) // s
                .cubic_curve_by((2.7, 1.1, 5.7, 1.1, 8.3, 0.0)) // c
                .cubic_curve_by((1.3, -0.6, 2.5, -1.4, 3.5, -2.4)) // c
                .smooth_cubic_curve_by((1.8, -2.2, 2.4, -3.6)) // s
                .cubic_curve_by((0.5, -1.3, 0.8, -2.7, 0.8, -4.2)) // c
                .cubic_curve_by((0.0, -1.5, -0.3, -2.9, -0.8, -4.3)) // c
                .cubic_curve_by((-0.6, -1.3, -1.4, -2.5, -2.4, -3.6)) // c
                .cubic_curve_to((31.7, 24.5, 30.5, 23.7, 29.2, 23.1)); // C
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Gemini => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((38.2, 42.3)) // M
                .cubic_curve_by((3.5, 0.5, 6.9, 1.1, 10.2, 2.0)) // c
                .vertical_line_by(4.7) // v
                .cubic_curve_by((-7.4, -1.9, -15.1, -2.8, -23.3, -2.8))
                .cubic_curve_by((-8.2, 0.0, -15.9, 0.9, -23.3, 2.8))
                .vertical_line_by(-4.7) // v
                .cubic_curve_by((3.3, -0.8, 6.7, -1.5, 10.2, -2.0)) // c
                .vertical_line_to(7.2) // V
                .cubic_curve_to((8.5, 6.7, 5.1, 6.0, 1.8, 5.2)) // C
                .vertical_line_to(0.5) // V
                .cubic_curve_to((9.2, 2.4, 17.0, 3.3, 25.1, 3.3)) // C
                .cubic_curve_by((8.2, 0.0, 15.9, -0.9, 23.3, -2.8)) // c
                .vertical_line_by(4.7) // v
                .cubic_curve_by((-3.3, 0.8, -6.7, 1.5, -10.2, 2.0)) // c
                .vertical_line_to(42.3) // V
                .move_to((33.6, 7.7)) // M
                .cubic_curve_to((30.8, 7.9, 28.0, 8.0, 25.1, 8.0)) // C
                .cubic_curve_by((-2.9, 0.0, -5.7, -0.1, -8.4, -0.3)) // c
                .vertical_line_by(34.2) // v
                .cubic_curve_by((2.8, -0.2, 5.6, -0.3, 8.4, -0.3)) // c
                .cubic_curve_by((2.9, 0.0, 5.7, 0.1, 8.4, 0.3)) // c
                .vertical_line_to(7.7); // V
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Cancer => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((2.5, 32.2)) // M
                .cubic_curve_by((1.9, 1.2, 3.7, 2.2, 5.5, 2.9)) // c
                .cubic_curve_by((4.5, 1.9, 9.4, 2.9, 14.8, 2.9)) // c
                .cubic_curve_by((3.4, 0.0, 6.7, -0.4, 9.7, -1.2)) // c
                .cubic_curve_by((-0.3, -0.2, -0.5, -0.5, -0.8, -0.7)) // c
                .cubic_curve_by((-0.9, -1.0, -1.6, -2.0, -2.1, -3.1)) // c
                .cubic_curve_to((29.3, 32.0, 29.0, 30.8, 29.0, 29.6)) // C
                .cubic_curve_by((0.0, -1.2, 0.2, -2.4, 0.7, -3.6)) // c
                .cubic_curve_by((0.5, -1.2, 1.2, -2.2, 2.1, -3.1)) // c
                .cubic_curve_by((1.0, -0.9, 2.0, -1.6, 3.0, -2.1)) // c
                .cubic_curve_by((1.1, -0.5, 2.3, -0.7, 3.6, -0.7)) // c
                .cubic_curve_by((1.4, 0.0, 2.5, 0.2, 3.6, 0.7)) // c
                .cubic_curve_by((1.2, 0.5, 2.2, 1.2, 3.0, 2.1)) // c
                .cubic_curve_by((0.9, 1.0, 1.6, 2.0, 2.1, 3.1)) // c
                .cubic_curve_by((0.5, 1.1, 0.7, 2.3, 0.7, 3.6)) // c
                .cubic_curve_by((0.0, 1.3, -0.2, 2.4, -0.7, 3.6)) // c
                .cubic_curve_by((-0.5, 1.3, -1.3, 2.3, -2.4, 3.2)) // c
                .cubic_curve_by((-1.5, 1.3, -3.3, 2.3, -5.2, 3.1)) // c
                .cubic_curve_by((-5.1, 2.2, -10.6, 3.3, -16.6, 3.2)) // c
                .cubic_curve_by((-6.1, 0.0, -11.7, -1.1, -16.6, -3.2)) // c
                .cubic_curve_by((-1.3, -0.6, -2.5, -1.2, -3.7, -1.8)) // c
                .vertical_line_to(32.2) // V
                .move_to((36.6, 33.9)) // M
                .cubic_curve_by((0.5, 0.2, 1.1, 0.3, 1.8, 0.3)) // c
                .cubic_curve_by((0.6, 0.0, 1.2, -0.1, 1.8, -0.3)) // c
                .cubic_curve_by((0.6, -0.2, 1.1, -0.6, 1.5, -1.0)) // c
                .cubic_curve_by((0.4, -0.4, 0.8, -0.9, 1.0, -1.5)) // c
                .cubic_curve_by((0.2, -0.6, 0.3, -1.2, 0.3, -1.8)) // c
                .cubic_curve_by((0.0, -0.6, -0.1, -1.2, -0.3, -1.8)) // c
                .cubic_curve_by((-0.2, -0.5, -0.6, -1.1, -1.0, -1.5)) // c
                .cubic_curve_by((-0.4, -0.4, -0.9, -0.7, -1.5, -1.0)) // c
                .cubic_curve_by((-0.5, -0.2, -1.1, -0.3, -1.8, -0.3)) // c
                .cubic_curve_by((-0.6, 0.0, -1.2, 0.1, -1.8, 0.3)) // c
                .cubic_curve_by((-0.6, 0.3, -1.1, 0.6, -1.5, 1.0)) // c
                .cubic_curve_by((-0.4, 0.4, -0.8, 0.9, -1.0, 1.5)) // c
                .cubic_curve_by((-0.2, 0.6, -0.3, 1.2, -0.3, 1.8)) // c
                .cubic_curve_by((0.0, 0.6, 0.1, 1.3, 0.3, 1.8)) // c
                .cubic_curve_by((0.2, 0.5, 0.6, 1.1, 1.0, 1.5)) // c
                .cubic_curve_to((35.5, 33.3, 36.0, 33.6, 36.6, 33.9)) // C
                .move_to((47.7, 17.3)) // M
                .cubic_curve_by((-1.9, -1.2, -3.7, -2.2, -5.5, -2.9)) // c
                .cubic_curve_by((-4.5, -1.9, -9.4, -2.9, -14.8, -2.9)) // c
                .cubic_curve_by((-3.4, 0.0, -6.7, 0.4, -9.7, 1.2)) // c
                .cubic_curve_by((0.3, 0.2, 0.5, 0.5, 0.8, 0.7)) // c
                .cubic_curve_by((0.9, 1.0, 1.6, 2.0, 2.1, 3.1)) // c
                .cubic_curve_by((0.5, 1.1, 0.7, 2.3, 0.7, 3.6)) // c
                .cubic_curve_by((0.0, 1.3, -0.2, 2.4, -0.7, 3.6)) // c
                .cubic_curve_by((-0.5, 1.2, -1.2, 2.2, -2.1, 3.1)) // c
                .cubic_curve_by((-1.0, 0.9, -2.0, 1.6, -3.0, 2.1)) // c
                .cubic_curve_by((-1.1, 0.5, -2.3, 0.7, -3.6, 0.7)) // c
                .cubic_curve_by((-1.4, 0.0, -2.5, -0.2, -3.6, -0.7)) // c
                .cubic_curve_by((-1.2, -0.5, -2.2, -1.2, -3.0, -2.1)) // c
                .cubic_curve_by((-0.9, -1.0, -1.6, -2.0, -2.1, -3.1)) // c
                .cubic_curve_by((-0.5, -1.1, -0.7, -2.3, -0.7, -3.6)) // c
                .cubic_curve_by((0.0, -1.2, 0.2, -2.4, 0.7, -3.6)) // c
                .cubic_curve_by((0.5, -1.2, 1.3, -2.3, 2.4, -3.2)) // c
                .cubic_curve_by((1.5, -1.2, 3.3, -2.3, 5.2, -3.1)) // c
                .cubic_curve_by((5.1, -2.2, 10.6, -3.2, 16.6, -3.2)) // c
                .cubic_curve_by((6.1, 0.0, 11.7, 1.1, 16.6, 3.2)) // c
                .cubic_curve_by((1.3, 0.6, 2.5, 1.2, 3.7, 1.8)) // c
                .vertical_line_to(17.3) // V
                .move_to((13.6, 15.5)) // M
                .cubic_curve_by((-0.5, -0.2, -1.1, -0.3, -1.8, -0.3)) // c
                .cubic_curve_by((-0.6, 0.0, -1.2, 0.1, -1.8, 0.3)) // c
                .cubic_curve_by((-0.6, 0.3, -1.1, 0.6, -1.5, 1.0)) // c
                .cubic_curve_by((-0.4, 0.4, -0.8, 0.9, -1.0, 1.5)) // c
                .cubic_curve_by((-0.2, 0.6, -0.3, 1.2, -0.3, 1.8)) // c
                .cubic_curve_by((0.0, 0.6, 0.1, 1.2, 0.3, 1.8)) // c
                .cubic_curve_by((0.2, 0.5, 0.6, 1.1, 1.0, 1.5)) // c
                .cubic_curve_by((0.4, 0.4, 0.9, 0.7, 1.5, 1.0)) // c
                .cubic_curve_by((0.5, 0.2, 1.1, 0.3, 1.8, 0.3)) // c
                .cubic_curve_by((0.6, 0.0, 1.2, -0.1, 1.8, -0.3)) // c
                .cubic_curve_by((0.6, -0.2, 1.1, -0.6, 1.5, -1.0)) // c
                .cubic_curve_by((0.4, -0.4, 0.8, -0.9, 1.0, -1.5)) // c
                .cubic_curve_by((0.2, -0.6, 0.3, -1.2, 0.3, -1.8)) // c
                .cubic_curve_by((0.0, -0.6, -0.1, -1.2, -0.3, -1.8)) // c
                .cubic_curve_by((-0.2, -0.5, -0.6, -1.1, -1.0, -1.5)) // c
                .cubic_curve_to((14.8, 16.2, 14.3, 15.8, 13.6, 15.5)); // C
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Leo => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((2833.0, 4324.7)) // M
                .cubic_curve_by((-0.7, 0.6, -1.3, 0.9, -1.9, 0.9)) // c
                .cubic_curve_by((-0.5, 0.0, -1.0, -0.2, -1.3, -0.5)) // c
                .cubic_curve_by((-0.4, -0.3, -0.5, -0.8, -0.5, -1.3)) // c
                .cubic_curve_by((0.0, -0.7, 0.4, -1.8, 1.1, -3.3)) // c
                .line_by((0.5, -1.2)) // l
                .cubic_curve_by((0.4, -0.9, 0.6, -1.6, 0.6, -2.2)) // c
                .cubic_curve_by((0.0, -0.7, -0.2, -1.3, -0.6, -1.7)) // c
                .cubic_curve_by((-0.4, -0.4, -1.0, -0.6, -1.6, -0.6)) // c
                .cubic_curve_by((-0.6, 0.0, -1.2, 0.2, -1.6, 0.6)) // c
                .cubic_curve_by((-0.4, 0.4, -0.6, 0.9, -0.6, 1.5)) // c
                .cubic_curve_by((0.0, 0.6, 0.2, 1.3, 0.6, 2.0)) // c
                .line_by((0.2, 0.5)) // l
                .cubic_curve_by((0.4, 0.8, 0.6, 1.3, 0.6, 1.7)) // c
                .cubic_curve_by((0.0, 0.6, -0.2, 1.1, -0.7, 1.5)) // c
                .cubic_curve_by((-0.4, 0.4, -1.0, 0.6, -1.6, 0.6)) // c
                .cubic_curve_by((-0.6, 0.0, -1.1, -0.2, -1.6, -0.6)) // c
                .cubic_curve_by((-0.4, -0.4, -0.6, -1.0, -0.6, -1.6)) // c
                .cubic_curve_by((0.0, -0.6, 0.2, -1.1, 0.6, -1.5)) // c
                .cubic_curve_by((0.4, -0.4, 0.9, -0.6, 1.5, -0.6)) // c
                .cubic_curve_by((0.2, 0.0, 0.4, 0.0, 0.7, 0.1)) // c
                .cubic_curve_by((-0.4, -0.8, -0.6, -1.5, -0.6, -2.1)) // c
                .cubic_curve_by((0.0, -0.8, 0.3, -1.4, 0.8, -2.0)) // c
                .cubic_curve_by((0.6, -0.5, 1.3, -0.8, 2.1, -0.8)) // c
                .cubic_curve_by((0.9, 0.0, 1.7, 0.3, 2.3, 0.8)) // c
                .cubic_curve_by((0.6, 0.6, 0.9, 1.3, 0.9, 2.2)) // c
                .cubic_curve_by((0.0, 0.5, 0.0, 0.8, -0.1, 1.2)) // c
                .cubic_curve_by((-0.1, 0.3, -0.3, 0.9, -0.7, 1.6)) // c
                .line_by((-0.4, 0.9)) // l
                .cubic_curve_by((-0.7, 1.5, -1.0, 2.5, -1.0, 2.9)) // c
                .cubic_curve_by((0.0, 0.3, 0.1, 0.6, 0.3, 0.8)) // c
                .cubic_curve_by((0.2, 0.2, 0.4, 0.3, 0.7, 0.3)) // c
                .cubic_curve_by((0.4, 0.0, 0.9, -0.2, 1.4, -0.7)) // c
                .line_to((2833.0, 4324.7)) // L
                .close() // z
                .move_to((2824.6, 4321.0)) // M
                .cubic_curve_by((0.0, 0.4, 0.2, 0.8, 0.4, 1.1)) // c
                .cubic_curve_by((0.3, 0.3, 0.7, 0.5, 1.1, 0.5)) // c
                .cubic_curve_by((0.4, 0.0, 0.8, -0.2, 1.1, -0.5)) // c
                .cubic_curve_by((0.3, -0.3, 0.5, -0.7, 0.5, -1.1)) // c
                .cubic_curve_by((0.0, -0.4, -0.2, -0.8, -0.4, -1.1)) // c
                .cubic_curve_by((-0.3, -0.3, -0.7, -0.5, -1.1, -0.5)) // c
                .cubic_curve_by((-0.4, 0.0, -0.8, 0.2, -1.1, 0.5)) // c
                .cubic_curve_to((
                    2824.7, 4320.2, 2824.6, 4320.6, 2824.6, 4321.0,
                )) // c
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set(
                    "transform",
                    "matrix(4.260087,0,0,4.260933,-12024.51,-18381.66)",
                )
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Virgo => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((2873.0, 4448.4)) // M
                .vertical_line_by(5.4) // v
                .horizontal_line_by(-1.0) // h
                .vertical_line_by(-6.2) // v
                .cubic_curve_by((0.0, -0.9, -0.3, -1.7, -0.9, -2.4)) // c
                .horizontal_line_by(1.1) // h
                .cubic_curve_by((0.4, 0.4, 0.7, 1.0, 0.8, 1.7)) // c
                .cubic_curve_by((0.4, -0.7, 0.8, -1.3, 1.2, -1.7)) // c
                .line_by((1.0, -0.2)) // l
                .cubic_curve_by((0.5, 0.5, 0.8, 1.2, 0.9, 2.0)) // c
                .cubic_curve_by((0.2, -0.6, 0.7, -1.2, 1.3, -1.8)) // c
                .line_by((1.0, -0.2)) // l
                .cubic_curve_by((0.6, 0.6, 0.8, 1.4, 0.8, 2.1)) // c
                .vertical_line_by(1.6) // v
                .cubic_curve_by((0.3, -0.7, 0.6, -1.2, 0.9, -1.5)) // c
                .line_by((0.9, -0.3)) // l
                .cubic_curve_by((0.7, 0.9, 1.1, 1.9, 1.1, 3.0)) // c
                .cubic_curve_by((0.0, 0.8, -0.2, 1.6, -0.7, 2.3)) // c
                .smooth_cubic_curve_by((-1.1, 1.4, -2, 2)) // s
                .cubic_curve_by((0.1, 0.5, 0.4, 1.1, 1.1, 1.7)) // c
                .horizontal_line_by(-1.2) // h
                .cubic_curve_by((-0.3, -0.3, -0.6, -0.7, -0.8, -1.2)) // c
                .cubic_curve_by((-0.6, 0.3, -1.4, 0.5, -2.4, 0.6)) // c
                .vertical_line_by(-0.7) // v
                .cubic_curve_by((0.9, -0.1, 1.6, -0.4, 2.2, -0.6)) // c
                .cubic_curve_by((-0.1, -0.4, -0.1, -0.7, -0.1, -1.1)) // c
                .vertical_line_by(-5.2) // v
                .cubic_curve_by((0.0, -0.9, -0.1, -1.5, -0.4, -1.9)) // c
                .cubic_curve_by((-0.7, 0.7, -1.3, 1.6, -1.6, 2.8)) // c
                .vertical_line_by(5.2) // v
                .horizontal_line_by(-1.0) // h
                .vertical_line_by(-5.7) // c
                .cubic_curve_by((0.0, -1.1, -0.2, -1.8, -0.5, -2.2)) // c
                .cubic_curve_to((
                    2873.9, 4446.5, 2873.4, 4447.3, 2873.0, 4448.4,
                )) // C
                .close() // z
                .move_to((2879.3, 4453.5)) // M
                .cubic_curve_by((0.6, -0.5, 1.1, -1.1, 1.4, -1.6)) // c
                .cubic_curve_by((0.3, -0.6, 0.5, -1.2, 0.5, -1.9)) // c
                .cubic_curve_by((0.0, -0.7, -0.2, -1.4, -0.5, -2.0)) // c
                .cubic_curve_by((-0.5, 0.5, -0.9, 1.2, -1.3, 2.1)) // c
                .vertical_line_to(4453.5) // V
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set(
                    "transform",
                    "matrix(4.538455,0,0,4.531784,-13030.44,-20143.78)",
                )
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Libra => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((19.7, 31.9)) // M
                .horizontal_line_to(1.8) // H
                .vertical_line_by(-4.7) // v
                .horizontal_line_by(10.8) // h
                .cubic_curve_by((-0.8, -1.0, -1.4, -2.0, -1.9, -3.1)) // c
                .cubic_curve_by((-0.8, -1.8, -1.2, -3.6, -1.2, -5.6)) // c
                .cubic_curve_by((0.0, -2.0, 0.4, -3.9, 1.2, -5.7)) // c
                .cubic_curve_by((0.8, -1.8, 2.0, -3.4, 3.4, -4.8)) // c
                .cubic_curve_by((1.5, -1.4, 3.2, -2.5, 5.1, -3.2)) // c
                .cubic_curve_to((21.0, 3.9, 23.0, 3.5, 25.1, 3.5)) // C
                .cubic_curve_by((2.2, 0.0, 4.2, 0.4, 5.9, 1.1)) // c
                .cubic_curve_by((2.0, 0.9, 3.7, 1.9, 5.1, 3.2)) // c
                .cubic_curve_by((1.5, 1.4, 2.6, 3.0, 3.4, 4.8)) // c
                .cubic_curve_by((0.8, 1.8, 1.2, 3.7, 1.2, 5.7)) // c
                .cubic_curve_by((0.0, 2.0, -0.4, 3.9, -1.2, 5.6)) // c
                .cubic_curve_by((-0.5, 1.1, -1.1, 2.2, -1.9, 3.1)) // c
                .horizontal_line_by(10.8) // h
                .vertical_line_by(4.7) // v
                .horizontal_line_to(30.6) // H
                .vertical_line_by(-4.7) // v
                .cubic_curve_by((0.8, -0.4, 1.5, -1.0, 2.3, -1.6)) // c
                .cubic_curve_by((1.0, -0.9, 1.8, -2.0, 2.4, -3.3)) // c
                .cubic_curve_by((0.5, -1.2, 0.8, -2.5, 0.8, -3.9)) // c
                .cubic_curve_by((0.0, -1.4, -0.3, -2.7, -0.8, -3.9)) // c
                .cubic_curve_by((-0.6, -1.2, -1.4, -2.3, -2.4, -3.3)) // c
                .cubic_curve_by((-1.0, -0.9, -2.2, -1.6, -3.5, -2.2)) // c
                .cubic_curve_by((-1.2, -0.5, -2.6, -0.7, -4.2, -0.8)) // c
                .cubic_curve_by((-1.5, 0.0, -2.9, 0.3, -4.2, 0.8)) // c
                .cubic_curve_by((-1.3, 0.5, -2.5, 1.3, -3.5, 2.2)) // c
                .cubic_curve_by((-1.0, 1.0, -1.8, 2.1, -2.4, 3.3)) // c
                .cubic_curve_by((-0.5, 1.2, -0.8, 2.5, -0.8, 3.9)) // c
                .cubic_curve_by((0.0, 1.4, 0.3, 2.6, 0.8, 3.9)) // c
                .cubic_curve_by((0.6, 1.2, 1.4, 2.3, 2.4, 3.3)) // c
                .cubic_curve_by((0.6, 0.6, 1.4, 1.1, 2.2, 1.6)) // c
                .vertical_line_to(31.9) // v
                .move_to((1.8, 45.9)) // M
                .vertical_line_by(-4.7) // v
                .horizontal_line_by(46.6) // v
                .vertical_line_by(4.7) // v
                .horizontal_line_to(1.8); // H
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Scorpio => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((40.1, 46.6)) // M
                .cubic_curve_by((-1.3, 0.0, -2.4, -0.2, -3.2, -0.5)) // c
                .cubic_curve_by((-0.9, -0.4, -1.7, -0.9, -2.3, -1.6)) // c
                .cubic_curve_by((-0.7, -0.7, -1.2, -1.5, -1.5, -2.3)) // c
                .cubic_curve_by((-0.3, -0.9, -0.5, -1.9, -0.5, -3)) // c
                .vertical_line_to(15.5) // V
                .cubic_curve_by((0.0, -3.1, -0.2, -5.4, -0.5, -6.9)) // c
                .cubic_curve_by((-0.5, -2.4, -1.2, -3.6, -2.0, -3.6)) // c
                .cubic_curve_by((-0.8, 0.0, -2.0, 1.5, -3.4, 4.6)) // c
                .cubic_curve_by((-1.1, 2.4, -1.7, 5.0, -1.7, 7.8)) // c
                .vertical_line_by(22.0) // V
                .horizontal_line_to(21.0) // H
                .vertical_line_to(15.5) // V
                .cubic_curve_by((0.0, -3.0, -0.2, -5.4, -0.5, -6.9)) // c
                .cubic_curve_by((-0.5, -2.4, -1.2, -3.6, -2.0, -3.6)) // c
                .cubic_curve_by((-0.8, 0.0, -2.0, 1.5, -3.4, 4.6)) // c
                .cubic_curve_by((-1.1, 2.4, -1.7, 5.0, -1.7, 7.8)) // c
                .vertical_line_by(22.0) // v
                .horizontal_line_to(9.5) // H
                .vertical_line_to(15.5) // V
                .cubic_curve_by((0.0, -3.6, -0.5, -6.8, -1.4, -9.6)) // c
                .cubic_curve_to((7.5, 4.3, 6.6, 2.7, 5.4, 1.0)) // C
                .horizontal_line_by(3.9) // h
                .cubic_curve_by((0.7, 0.7, 1.3, 1.5, 1.8, 2.5)) // c
                .cubic_curve_by((0.6, 1.1, 1.0, 2.4, 1.4, 3.8)) // c
                .cubic_curve_by((0.5, -1.5, 1.2, -2.9, 2.1, -4.0)) // c
                .cubic_curve_to((16.2, 1.1, 17.7, 0.0, 18.7, 0.0)) // C
                .cubic_curve_by((1.2, 0.0, 2.5, 1.2, 3.9, 3.5)) // c
                .cubic_curve_by((0.5, 0.9, 1.0, 2.2, 1.4, 3.8)) // c
                .cubic_curve_by((0.5, -1.5, 1.2, -2.9, 2.1, -4.0)) // c
                .cubic_curve_to((27.8, 1.1, 29.3, 0.0, 30.5, 0.0)) // C
                .cubic_curve_by((1.2, 0.0, 2.5, 1.2, 3.9, 3.5)) // c
                .cubic_curve_by((0.7, 1.2, 1.2, 2.5, 1.4, 3.8)) // c
                .cubic_curve_by((0.3, 2.4, 0.5, 4.5, 0.5, 6.2)) // c
                .vertical_line_by(25.8) // v
                // After that line the -> of Scprpio
                .cubic_curve_by((0.0, 0.7, 0.1, 1.1, 0.2, 1.5)) // c
                .cubic_curve_by((0.2, 0.4, 0.4, 0.8, 0.7, 1.1)) // c
                .cubic_curve_by((0.3, 0.3, 0.6, 0.5, 1.0, 0.7)) // c
                .cubic_curve_by((0.4, 0.2, 1.0, 0.3, 1.7, 0.3)) // c
                .horizontal_line_by(1.5) // h
                .vertical_line_by(-3.4) // v
                .line_by((5.5, 5.3)) // l
                .line_to((41.5, 50.0)) // L
                .vertical_line_by(-3.4) // v
                .horizontal_line_to(40.1); // H
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Sagittarius => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((44.1, 10.0)) // M
                .line_to((22.7, 31.4)) // L
                .line_by((10.8, 10.8)) // l
                .line_by((-4.2, 4.2)) // l
                .line_to((18.4, 35.7)) // L
                .line_to((4.4, 49.7)) // L
                .line_by((-4.2, -4.2)) // l
                .line_by((14.1, -14.1)) // l
                .line_to((3.4, 20.7)) // L
                .line_by((4.2, -4.2)) // l
                .line_by((10.8, 10.8)) // l
                .line_to((40.0, 5.7)) // L
                .horizontal_line_to(23.6) // H
                .vertical_line_by(-6.0) // v
                .horizontal_line_by(26.5) // h
                .vertical_line_by(26.5) // v
                .horizontal_line_by(-6.0) // h
                .vertical_line_to(10.0); // V
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Capricorn => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((8.0, 1.5)) // M
                .horizontal_line_by(33.0) // h
                .cubic_curve_by((-40.0, 7.0, -35.0, 47.0, -15.5, 47.0)) // c
                .cubic_curve_by((8.8, 0.0, 16.0, -7.2, 16.0, -16.0)) // c
                .smooth_cubic_curve_by((-7.2, -16.0, -16.0, -16.0)) // s
                .smooth_cubic_curve_by((-16.0, 7.2, -16.0, 16.0)) // s
                .cubic_curve_by((0.0, 3.3, 0.3, 8.3, 1.0, 15.0)); // c
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        Signs::Aquarius => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((17.0, 21.0)) // M
                .line_by((-3.6, -6.3)) // l
                .line_to((2.5, 21.0)) // L
                .line_to((0.1, 17.0)) // L
                .line_by((15.0, -8.6)) // L
                .line_by((3.6, 6.3)) // l
                .line_by((10.9, -6.3)) // l
                .line_by((3.6, 6.3)) // l
                .line_by((10.9, -6.3)) // l
                .line_by((6.0, 10.3)) // l
                .line_to((46.0, 21.0)) // L
                .line_by((-3.6, -6.3)) // l
                .line_to((31.5, 21.0)) // L
                .line_by((-3.6, -6.3)) // l
                .line_to((17.0, 21.0)) // L
                .move_to((17.0, 41.1)) // M
                .line_by((-3.6, -6.3)) // l
                .line_to((2.5, 41.1)) // L
                .line_by((-2.4, -4.1)) // l
                .line_by((15.0, -8.6)) // l
                .line_by((3.6, 6.3)) // l
                .line_by((10.9, -6.3)) // l
                .line_by((3.6, 6.3)) // l
                .line_by((10.9, -6.3)) // l
                .line_by((6.0, 10.3)) // l
                .line_to((46.0, 41.1)) // L
                .line_by((-3.6, -6.3)) // l
                .line_by((-10.9, 6.3)) // l
                .line_by((-3.6, -6.3)) // l
                .line_to((17.0, 41.1)); // L
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 0)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        },
        // Center to 50x50 and recopy (for other sign)
        _ => {
            document = Document::new().set("viewBox", (0, 0, 50, 50));
        },
    }
    document
}

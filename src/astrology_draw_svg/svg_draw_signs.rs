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
        _ => {
            document = Document::new().set("viewBox", (0, 0, 50, 50));
        },
    }
    document
}

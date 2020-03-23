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
use svg::node::element::{Group, Path};
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
        _ => {
            document = Document::new();
        },
    }
    document
}

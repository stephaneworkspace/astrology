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
//use strum::AsStaticRef;
use libswe_sys::sweconst::Bodies;
use svg::node::element::path::{Data, Number};
use svg::node::element::{Circle, Group, Line, Path};
use svg::Document;
pub const BODIE_SIZE: Number = 50.0;

pub fn is_retrograde(sw: bool, color: String) -> Path {
    let data;
    if sw {
        data = Data::new()
            .move_to((43.1, 40.2)) // M
            .cubic_curve_by((0.6, -0.1, 1.5, -0.2, 2.3, -0.2)) // c
            .cubic_curve_by((1.3, 0.0, 2.1, 0.2, 2.7, 0.8)) // c
            .cubic_curve_by((0.5, 0.4, 0.7, 1.1, 0.7, 1.8)) // c
            .cubic_curve_by((0.0, 1.2, -0.8, 2.1, -1.8, 2.4)) // c
            .vertical_line_by(0.0) // v
            .cubic_curve_by((0.7, 0.3, 1.2, 0.9, 1.4, 1.9)) // c
            .cubic_curve_by((0.3, 1.3, 0.5, 2.2, 0.7, 2.6)) // c
            .horizontal_line_to(48.0) // H
            .cubic_curve_by((-0.2, -0.3, -0.4, -1.1, -0.6, -2.3)) // c
            .cubic_curve_by((-0.3, -1.3, -0.8, -1.8, -1.9, -1.8)) // c
            .horizontal_line_by(-1.1) // h
            .vertical_line_by(4.1) // v
            .horizontal_line_by(-1.2) // h
            .vertical_line_to(40.2) // V
            .close() // z
            .move_to((44.3, 44.5)) // M
            .horizontal_line_by(1.2) // h
            .cubic_curve_by((1.3, 0.0, 2.1, -0.7, 2.1, -1.8)) // c
            .cubic_curve_by((0.0, -1.2, -0.9, -1.8, -2.2, -1.8)) // c
            .cubic_curve_by((-0.6, 0.0, -1.0, 0.1, -1.2, 0.1)) // c
            .vertical_line_to(44.5) // V
            .close(); // z
    } else {
        data = Data::new();
    }

    Path::new()
        .set("stroke", color)
        .set("stroke-width", 1)
        .set("d", data)
}

pub fn draw_bodie(bodie: Bodies, sw_retrograde: bool) -> Document {
    let size: (Number, Number) = (BODIE_SIZE, BODIE_SIZE);
    let path: Path;
    let document: Document;
    let color: String = format!("#{:06X}", bodie.object_color());
    match bodie {
        Bodies::Sun => {
            let data = Data::new()
                .move_by((7.0, 25.0)) // m
                .elliptical_arc_by((18.0, 18.0, 0, 1, 1, 0, 0.1)) // a
                .close() // z
                .move_by((3.0, 0.0)) // m
                .elliptical_arc_by((15.0, 15.0, 0, 1, 0, 0, -0.1)) // a
                .close() // z
                .move_by((11.0, 0.0)) // m
                .elliptical_arc_by((4.0, 4.0, 0, 1, 0, 0, -0.1)) // a
                .close(); // z
            path = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Moon => {
            let data = Data::new()
                .move_to((12.5, 3.5)) // M
                .elliptical_arc_by((22.5, 22.5, 0, 0, 1, 0, 43)) // a
                .elliptical_arc_by((22.5, 22.5, 0, 1, 0, 0, -43)) // a
                .close(); // z
            path = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Mercury => {
            let data1 = Data::new()
                .move_to((112.0, 36.5)) // M
                .elliptical_arc_to((11.5, 11.5, 0, 1, 1, 89, 36.5)) // A
                .elliptical_arc_to((11.5, 11.5, 0, 1, 1, 112, 36.5)) // A
                .close(); // z
            let data2 = Data::new()
                .move_to((111.9469, 37.603862)) // M
                .elliptical_arc_to((11.5, 11.5, 0, 0, 1, 89.052015, 37.592533));
            let data3 = Data::new()
                .move_to((373.83706, 512.99267)) // M
                .line_to((373.83706, 524.99267)); // L
            let data4 = Data::new()
                .move_to((368.83706, 519.99267)) // M
                .line_to((378.83706, 519.99267)); // L
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data1)
                .set("transform", "matrix(0.96,0,0,0.96,277.357,466.9525)");
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2)
                .set(
                    "transform",
                    "matrix(0.810715,0,0,0.810715,292.4483,451.9429)",
                );
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data4);
            let group = Group::new()
                .set("transform", "translate(-348.7552,-478.0905)")
                .add(path1)
                .add(path2)
                .add(path3)
                .add(path4);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Venus => {
            let data = Data::new()
                .move_to((31.6, 39.1)) // M
                .horizontal_line_to(17.9) // H
                .move_to((24.8, 46.3)) // M
                .vertical_line_to(29.9) // V
                .cubic_curve_by((-7.3, 0.0, -13.2, -5.9, -13.1, -13.2)) // c
                .cubic_curve_by((0.0, -7.3, 5.9, -13.2, 13.2, -13.1)) // c
                .cubic_curve_to((32.1, 3.6, 38.0, 9.5, 38.0, 16.7)) // C
                .cubic_curve_by((0.0, 7.2, -5.9, 13.1, -13.1, 13.1)); // c
            path = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Mars => {
            let data = Data::new()
                .move_by((30.0, 21.0)) // m
                .elliptical_arc_by((12.2, 12.2, 0.0, 1.0, 0.0, 2.0, 2.0))
                .close()
                .line_by((1, 1, 11, -11)) // l
                .move_by((-9, 0)) // m
                .horizontal_line_by(9) // h
                .vertical_line_by(9); // v
            path = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3.3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Jupiter => {
            let data1 = Data::new()
                .move_to((382.83736, 486.87888))
                .line_to((382.83736, 519.93338));
            let data2 = Data::new()
                .move_to((388.2865, 511.54787))
                .line_to((361.949, 511.45787));
            let data3 = Data::new()
                .move_to((364.67357, 498.7446)) // M
                .cubic_curve_to((
                    363.76538, 498.7446, 361.949, 497.89705, 361.949, 494.50684,
                )) // C
                .cubic_curve_to((
                    361.949, 491.11663, 365.58176, 487.72643, 369.21452,
                    487.72643,
                )) // C
                .cubic_curve_to((
                    372.84728, 487.72643, 376.48003, 490.26908, 376.48003,
                    496.20194,
                )) // C
                .cubic_curve_to((
                    376.48003, 502.1348, 371.93909, 511.45787, 362.85719,
                    511.45787,
                )); //C
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data3);
            let group = Group::new()
                .set("transform", "translate(-348.7552,-478.0905)")
                .add(path1)
                .add(path2)
                .add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Saturn => {
            let data1 = Data::new()
                .move_to((368.54632, 484.01327)) // M
                .line_to((368.54632, 513.01327)); // L
            let data2 = Data::new()
                .move_to((363.65347, 488.76327)) // M
                .line_to((375.65347, 488.76327));
            let data3 = Data::new()
                .move_to((382.54632, 519.01327)) // M
                .cubic_curve_to((
                    381.54632, 520.01327, 380.54632, 521.01327, 379.54632,
                    521.01327,
                )) // C
                .cubic_curve_to((
                    378.54632, 521.01327, 376.54632, 520.01327, 376.54632,
                    518.01327,
                )) // C
                .cubic_curve_to((
                    376.54632, 516.01327, 377.54632, 514.01327, 379.54632,
                    512.01327,
                ))
                .cubic_curve_to((
                    381.54632, 510.01327, 383.54632, 506.01327, 383.54632,
                    502.01327,
                )) // C
                .cubic_curve_to((
                    383.54632, 498.01327, 381.54632, 494.01327, 377.54632,
                    494.01327,
                )) // C
                .cubic_curve_to((
                    373.76313, 494.01327, 370.54632, 496.01327, 368.54632,
                    500.01327,
                )); // C
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data3);
            let group = Group::new()
                .set("transform", "translate(-348.7552,-478.0905)")
                .add(path1)
                .add(path2)
                .add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Uranus => {
            let data1 = Data::new()
                .move_to((363.40346, 509.72756)) // M
                .line_to((356.40346, 509.72756)) // L
                .line_to((356.40346, 508.72756)) // L
                .line_to((360.40346, 507.72756)) // L
                .line_to((360.40346, 487.72756)) // L
                .line_to((356.40346, 486.72756)) // L
                .line_to((356.40346, 485.72756)) // L
                .line_to((363.40346, 485.72756)) // L
                .line_to((363.40346, 509.72756)) // L
                .close();
            let data2 = Data::new()
                .move_to((385.40346, 509.72756)) // M
                .line_to((392.40346, 509.72756)) // L
                .line_to((392.40346, 508.72756)) // L
                .line_to((388.40346, 507.72756)) // L
                .line_to((388.40346, 487.72756)) // L
                .line_to((392.40346, 486.72756)) // L
                .line_to((392.40346, 485.72756)) // L
                .line_to((385.40346, 485.72756)) // L
                .line_to((385.40346, 509.72756)) // L
                .close();
            let data3 = Data::new()
                .move_to((362.40346, 497.72756)) // M
                .line_to((386.40346, 497.72756)); // L
            let data4 = Data::new()
                .move_to((374.40346, 485.72756)) // M
                .line_to((374.40346, 511.72756)); // L
            let data5 = Data::new()
                .move_to((40, 211)) // M
                .elliptical_arc_to((4, 4, 0, 1, 1, 32, 211)) // A
                .elliptical_arc_to((4, 4, 0, 1, 1, 40, 211)) // A
                .close();
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data4);
            let path5 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 1)
                .set("d", data5)
                .set("transform", "translate(338.4034,305.7276)");
            let group = Group::new()
                .set("transform", "translate(-348.7552,-478.0905)")
                .add(path1)
                .add(path2)
                .add(path3)
                .add(path4)
                .add(path5);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Neptune => {
            let data1 = Data::new()
                .move_to((363.87696, 487.23598)) // M
                .cubic_curve_to((
                    361.22262, 505.2704, 365.64653, 507.97556, 374.49435,
                    507.97556,
                )) // C
                .cubic_curve_to((
                    383.34217, 507.97556, 387.76609, 505.2704, 385.11174,
                    487.23598,
                )); // C
            let data2 = Data::new()
                .move_to((374.49435, 489.03942)) // M
                .line_to((374.49435, 522.40309)); // L
            let data3 = Data::new()
                .move_to((367.41609, 515.18933)) // M
                .line_to((381.57261, 515.18933)); // L
            let data4 = Data::new()
                .move_to((358.98361, 489.72545)) // M
                .line_to((364.00408, 485.92077)) // L
                .line_to((367.73728, 491.03737)); // L
            let data5 = Data::new()
                .move_to((369.98609, 494.03404)) // M
                .line_to((374.36075, 489.47578)) // L
                .line_to((378.83336, 493.93421)); // L
            let data6 = Data::new()
                .move_to((381.18598, 491.35241)) // M
                .line_to((384.98296, 486.28478)) // L
                .line_to((389.95536, 490.15447)); // L
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data4);
            let path5 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data5);
            let path6 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data6);
            let group = Group::new()
                .set("transform", "translate(-348.7552,-478.0905)")
                .add(path1)
                .add(path2)
                .add(path3)
                .add(path4)
                .add(path5)
                .add(path6);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Pluto => {
            let data1 = Data::new()
                .move_to((275.59914, 423.24813)) // M
                .line_to((291.59914, 423.24813)); // L
            let data2 = Data::new()
                .move_to((283.59914, 431.24813)) // M
                .line_to((283.59914, 414.24813)); // L
            let data3 = Data::new()
                .move_to((172, 184)) // M
                .elliptical_arc_to((7, 7, 0, 1, 1, 158, 184)) // A
                .elliptical_arc_to((7, 7, 0, 1, 1, 172, 184)) // A
                .close(); // z
            let data4 = Data::new()
                .move_to((177, 184)) // M
                .elliptical_arc_to((12, 12, 0, 1, 1, 153, 184)); // A
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data3)
                .set("transform", "translate(118.5991,218.2481)");
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data4)
                .set("transform", "translate(118.5991,218.2481)");
            let group = Group::new()
                .set("transform", "translate(-258.5991,-387.1767)")
                .add(path1)
                .add(path2)
                .add(path3)
                .add(path4);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::TrueNode => {
            // Nord Node
            let data = Data::new()
                .move_to((22.0, 6.3)) // M
                .cubic_curve_by((-4.5, 1.1, -8.4, 4.4, -9.9, 8.8)) // c
                .cubic_curve_by((-1.0, 2.1, -1.0, 4.5, -0.3, 6.6)) // c
                .cubic_curve_by((0.6, 2.7, 2.1, 5.1, 3.8, 7.3)) // c
                .cubic_curve_by((2.2, 3.4, 2.3, 9.1, -0.7, 11.4)) // c
                .cubic_curve_by((-2.2, 1.7, -4.5, -0.2, -4.8, -2.0)) // c
                .cubic_curve_by((-0.8, -2.6, 1.4, -6.3, 4.3, -6.3)) // c
                .cubic_curve_by((0.9, 0.2, 1.9, 0.4, 0.6, -0.5)) // c
                .cubic_curve_by((-1.8, -1.3, -4.5, -1.1, -6.3, 0.2)) // c
                .cubic_curve_by((-1.7, 1.2, -2.4, 3.4, -1.9, 5.4)) // c
                .cubic_curve_by((0.4, 2.4, 2.1, 4.3, 4.4, 5.0)) // c
                .cubic_curve_by((2.2, 0.9, 5.0, 0.7, 6.9, -0.7)) // c
                .cubic_curve_by((2.9, -1.6, 3.9, -5.3, 3.4, -8.4)) // c
                .cubic_curve_by((-0.5, -2.3, -1.1, -4.4, -2.6, -7.0)) // c
                .cubic_curve_by((-2.4, -3.6, -4.1, -8.8, -2.0, -12.7)) // c
                .cubic_curve_by((1.2, -2.6, 4.4, -4.8, 7.6, -4.9)) // c
                .cubic_curve_by((2.9, -0.3, 5.5, 1.4, 7.2, 3.5)) // c
                .cubic_curve_by((1.5, 1.8, 2.3, 4.6, 2.1, 7.1)) // c
                .cubic_curve_by((-0.1, 3.6, -2.2, 7.1, -4.0, 9.8)) // c
                .cubic_curve_by((-1.0, 1.9, -1.1, 3.4, -1.2, 5.6)) // c
                .cubic_curve_by((0.2, 2.9, 0.6, 5.7, 3.3, 7.1)) // c
                .cubic_curve_by((3.4, 2.0, 8.5, 1.0, 10.3, -2.7)) // c
                .cubic_curve_by((1.3, -2.6, 0.6, -6.4, -2.3, -7.6)) // c
                .cubic_curve_by((-2.0, -1.1, -4.8, -0.7, -6.3, 1.1)) // c
                .cubic_curve_by((1.8, 0.0, 4.3, -0.2, 5.1, 2.0)) // c
                .cubic_curve_by((1.1, 2.2, 1.1, 5.6, -1.5, 6.8)) // c
                .cubic_curve_by((-2.1, 0.9, -4.3, -1.0, -4.7, -3.1)) // c
                .cubic_curve_by((-0.6, -2.8, -0.5, -5.9, 0.9, -8.4)) // c
                .cubic_curve_by((1.1, -2.0, 2.9, -3.6, 3.7, -5.8)) // c
                .cubic_curve_by((1.8, -4.0, 1.0, -8.8, -1.6, -12.2)) // c
                .cubic_curve_to((33.1, 8.1, 28.9, 6.0, 24.6, 6.1)) // C
                .cubic_curve_to((23.7, 6.1, 22.9, 6.2, 22.0, 6.3)) // C
                .close(); // z
            let path = Path::new()
                .set("fill", color.clone())
                .set("stroke", color.clone())
                .set("stroke-width", 1)
                .set("d", data);
            let circle = Circle::new()
                .set("cx", 24.5)
                .set("cy", 17.5)
                .set("r", 2.8)
                .set("fill", color.clone())
                .set("stroke", color.clone())
                .set("stroke-width", 1);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(circle)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::SouthNode => {
            // South Node
            let data = Data::new()
                .move_to((27.4, 42.6)) // M
                .cubic_curve_by((4.5, -1.1, 8.4, -4.4, 9.9, -8.8)) // c
                .cubic_curve_by((1.0, -2.1, 1.0, -4.5, 0.3, -6.6)) // c
                .cubic_curve_by((-0.6, -2.7, -2.1, -5.1, -3.8, -7.3)) // c
                .cubic_curve_by((-2.2, -3.4, -2.3, -9.1, 0.7, -11.4)) // c
                .cubic_curve_by((2.2, -1.7, 4.5, 0.2, 4.8, 2.0)) // c
                .cubic_curve_by((0.8, 2.6, -1.4, 6.3, -4.3, 6.3)) // c
                .cubic_curve_by((-0.9, -0.2, -1.9, -0.4, -0.6, 0.5)) // c
                .cubic_curve_by((1.8, 1.3, 4.5, 1.1, 6.3, -0.2)) // c
                .cubic_curve_by((1.7, -1.2, 2.4, -3.4, 1.9, -5.4)) // c
                .cubic_curve_by((-0.4, -2.4, -2.1, -4.3, -4.4, -5.0)) // c
                .cubic_curve_by((-2.2, -0.9, -5.0, -0.7, -6.9, 0.7)) // c
                .cubic_curve_by((-2.9, 1.6, -3.9, 5.3, -3.4, 8.4)) // c
                .cubic_curve_by((0.5, 2.3, 1.1, 4.4, 2.6, 7.0)) // c
                .cubic_curve_by((2.4, 3.6, 4.1, 8.8, 2.0, 12.7)) // c
                .cubic_curve_by((-1.2, 2.6, -4.4, 4.8, -7.6, 4.9)) // c
                .cubic_curve_by((-2.9, 0.3, -5.5, -1.4, -7.2, -3.5)) // c
                .cubic_curve_by((-1.5, -1.8, -2.3, -4.6, -2.1, -7.1)) // c
                .cubic_curve_by((0.1, -3.6, 2.2, -7.1, 4.0, -9.8)) // c
                .cubic_curve_by((1.0, -1.9, 1.1, -3.4, 1.2, -5.6)) // c
                .cubic_curve_by((-0.2, -2.9, -0.6, -5.7, -3.3, -7.1)) // c
                .cubic_curve_by((-3.4, -2.0, -8.5, -1.0, -10.3, 2.7)) // c
                .cubic_curve_by((-1.3, 2.6, -0.6, 6.4, 2.3, 7.6)) // c
                .cubic_curve_by((2.0, 1.1, 4.8, 0.7, 6.3, -1.1)) // c
                .cubic_curve_by((-1.8, 0.0, -4.3, 0.2, -5.1, -2.0)) // c
                .cubic_curve_by((-1.1, -2.2, -1.1, -5.6, 1.5, -6.8)) // c
                .cubic_curve_by((2.1, -0.9, 4.3, 1.0, 4.7, 3.1)) // c
                .cubic_curve_by((0.6, 2.8, 0.5, 5.9, -0.9, 8.4)) // c
                .cubic_curve_by((-1.1, 2.0, -2.9, 3.6, -3.7, 5.8)) // c
                .cubic_curve_by((-1.8, 4.0, -1.0, 8.8, 1.6, 12.2)) // c
                .cubic_curve_by((2.4, 3.5, 6.6, 5.7, 10.9, 5.6)) // c
                .cubic_curve_to((25.7, 42.8, 26.5, 42.7, 27.4, 42.6)) // C
                .close(); // z
            let path = Path::new()
                .set("fill", color.clone())
                .set("stroke", color.clone())
                .set("stroke-width", 1)
                .set("d", data);
            let circle = Circle::new()
                .set("cx", 24.9)
                .set("cy", 31.4)
                .set("r", 2.8)
                .set("fill", color.clone())
                .set("stroke", color.clone())
                .set("stroke-width", 1);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(circle)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Chiron => {
            let data1 = Data::new()
                .move_to((305.71428, 485.93362)) // M
                .elliptical_arc_to((
                    23.571428, 23.571428, 0, 1, 1, 258.57142, 485.93362,
                )) // A
                .elliptical_arc_to((
                    23.571428, 23.571428, 0, 1, 1, 305.71428, 485.93362,
                )) // A
                .close();
            let data2 = Data::new()
                .move_to((279.83739, 413.95933)) // M
                .line_to((280.14437, 391.24295)); // L
            let data3 = Data::new()
                .move_to((280.16299, 402.23989)) // M
                .line_to((289.98629, 393.95148)); // L
            let data4 = Data::new()
                .move_to((280.19864, 402.03214)) // M
                .line_to((290.02193, 410.32054)); // L
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 7)
                .set("d", data1)
                .set(
                    "transform",
                    "matrix(0.408124,0,0,0.408124,168.3595,224.7729)",
                );
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data4);
            let group = Group::new()
                .set("transform", "translate(-258.5991,-387.1767)")
                .add(path1)
                .add(path2)
                .add(path3)
                .add(path4);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::AsteroidLilith => {
            let data1 = Data::new()
                .move_to((26.1, 6.9)) // M
                .cubic_curve_to((20.1, 8.7, 16.8, 15.0, 18.6, 21.0)) // C
                .cubic_curve_by((1.8, 5.9, 8.1, 9.3, 14.1, 7.4)) // c
                .cubic_curve_by((-3.6, -1.1, -6.3, -3.9, -7.4, -7.4)) // c
                .cubic_curve_to((23.4, 15.0, 26.8, 8.7, 32.7, 6.9)) // C
                .cubic_curve_to((30.5, 6.2, 28.2, 6.2, 26.1, 6.9)) // C
                .close();
            let path1 = Path::new()
                .set("stroke", color.clone())
                .set("stroke-width", 1)
                .set("d", data1);
            let line = Line::new()
                .set("x1", 32.7)
                .set("y1", 37.0)
                .set("x2", 18.5)
                .set("y2", 37.0)
                .set("stroke", color.clone())
                .set("stroke-width", 1);
            let data2 = Data::new()
                .move_to((25.6, 44.5)) // M
                .cubic_curve_by((0.0, -5.6, 0.0, -11.1, 0.0, -16.7));
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path1)
                .add(line)
                .add(path2)
                .add(is_retrograde(sw_retrograde, color));
        },
        Bodies::Ceres => {
            let data1 = Data::new()
                .move_to((26.1, 6.9)) // M
                .cubic_curve_to((20.1, 8.7, 16.8, 15.0, 18.6, 21.0)) // C
                .cubic_curve_by((1.8, 5.9, 8.1, 9.3, 14.1, 7.4)) // c
                .cubic_curve_by((-3.6, -1.1, -6.3, -3.9, -7.4, -7.4)) // c
                .cubic_curve_to((23.4, 15.0, 26.8, 8.7, 32.7, 6.9)) // C
                .cubic_curve_to((30.5, 6.2, 28.2, 6.2, 26.1, 6.9)) // C
                .close();
            let path1 = Path::new()
                .set("stroke", color.clone())
                .set("stroke-width", 1)
                .set("d", data1);
            let line = Line::new()
                .set("x1", 32.7)
                .set("y1", 37.0)
                .set("x2", 18.5)
                .set("y2", 37.0)
                .set("stroke", color.clone())
                .set("stroke-width", 1);
            let data2 = Data::new()
                .move_to((25.6, 44.5)) // M
                .cubic_curve_by((0.0, -5.6, 0.0, -11.1, 0.0, -16.7));
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 3)
                .set("d", data2);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path1)
                .add(line)
                .add(path2)
                .add(is_retrograde(sw_retrograde, color));
        },
        _ => {
            document = Document::new().set("viewBox", (0, 0, 50, 50));
        },
    }
    document
}

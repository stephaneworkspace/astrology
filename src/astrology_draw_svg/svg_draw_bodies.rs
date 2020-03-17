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
use svg::node::element::{Group, Path};
use svg::Document;
pub const BODIE_SIZE: Number = 50.0;

pub fn is_retrograde(sw: bool) -> Path {
    let data;
    if sw {
        data = Data::new()
            .move_to((43.9, 41.6)) // M
            .cubic_curve_by((0.5, -0.1, 1.3, -0.2, 2.0, -0.2)) // c
            .cubic_curve_by((1.1, 0.0, 1.8, 0.2, 2.3, 0.7)) // c
            .cubic_curve_by((0.4, 0.4, 0.6, 0.9, 0.6, 1.5)) // c
            .cubic_curve_by((0.0, 1.1, -0.7, 1.8, -1.5, 2.1)) // c
            .vertical_line_by(0) // v
            .cubic_curve_by((0.6, 0.2, 1.0, 0.8, 1.2, 1.6)) // c
            .cubic_curve_by((0.3, 1.1, 0.5, 1.9, 0.6, 2.2)) // c
            .horizontal_line_by(-1.1) // h
            .cubic_curve_by((-0.1, -0.2, -0.3, -0.9, -0.5, -1.9)) // c
            .cubic_curve_by((-0.2, -1.1, -0.7, -1.5, -1.6, -1.6)) // c
            .horizontal_line_by(-1.0) // h
            .vertical_line_by(3.5) // v
            .horizontal_line_by(-1) // h
            .vertical_line_to(41.6) // V
            .close() //z
            .move_to((45.0, 45.2)) // M
            .horizontal_line_to(46.0) // H
            .cubic_curve_by((1.1, 0.0, 1.8, -0.6, 1.8, -1.5)) // c
            .cubic_curve_by((0.0, -1.0, -0.8, -1.5, -1.9, -1.5)) // c
            .cubic_curve_by((-0.5, 0.0, -0.9, 0.0, -1.0, 0.1)) // c
            .vertical_line_to(45.2) // V
            .close(); // z
    } else {
        data = Data::new();
    }

    Path::new().set("d", data)
}

pub fn draw_bodie(bodie: Bodies) -> Document {
    let size: (Number, Number);
    let path: Path;
    let document: Document;
    match bodie {
        Bodies::Sun => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(true));
        },
        Bodies::Moon => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((12.5, 3.5)) // M
                .elliptical_arc_by((22.5, 22.5, 0, 0, 1, 0, 43)) // a
                .elliptical_arc_by((22.5, 22.5, 0, 1, 0, 0, -43)) // a
                .close(); // z
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(true));
        },
        Bodies::Mercury => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1)
                .set("transform", "matrix(0.96,0,0,0.96,277.357,466.9525)");
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2)
                .set(
                    "transform",
                    "matrix(0.810715,0,0,0.810715,292.4483,451.9429)",
                );
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
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
                .add(is_retrograde(true));
        },
        Bodies::Venus => {
            size = (75.0, 75.0);
            let data = Data::new()
                .move_to((47.0, 59.0)) // M
                .horizontal_line_to(28.0) // H
                .move_by((9.5, 10.0)) // m
                .vertical_line_to(46.2) // v
                .elliptical_arc_by((18.3, 18.3, 0.0, 1.0, 1.0, 0.1, 0.0)) // a
                .close();
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 5)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(true));
        },
        Bodies::Mars => {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_by((30.0, 21.0)) // m
                .elliptical_arc_by((12.2, 12.2, 0, 1, 0, 2, 2))
                .close()
                .line_by((1, 1, 11, -11)) // l
                .move_by((-9, 0)) // m
                .horizontal_line_by(9) // h
                .vertical_line_by(9); // v
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3.3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(true));
        },
        Bodies::Jupiter => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
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
                .add(is_retrograde(true));
        },
        Bodies::Saturn => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
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
                .add(is_retrograde(true));
        },
        Bodies::Uranus => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data4);
            let path5 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
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
                .add(is_retrograde(true));
        },
        Bodies::Neptune => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data4);
            let path5 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data5);
            let path6 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
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
                .add(is_retrograde(true));
        },
        Bodies::Pluto => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data3)
                .set("transform", "translate(118.5991,218.2481)");
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
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
                .add(is_retrograde(true));
        },
        Bodies::TrueNode => {
            // Nord Node
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((21.176252, 1.2059591)) // M
                .cubic_curve_to((
                    15.228025, 2.7204431, 9.9876165, 7.0905621, 8.0006675,
                    12.977051,
                )) // C
                .cubic_curve_to((
                    6.7301655, 15.765889, 6.6746855, 18.928229, 7.5774885,
                    21.827786,
                )) // C
                .cubic_curve_to((
                    8.3866265, 25.47396, 10.353226, 28.690578, 12.668968,
                    31.574724,
                )) // C
                .cubic_curve_to((
                    15.591412, 36.146471, 15.774268, 43.729215, 11.712284,
                    46.710009,
                )) // C
                .cubic_curve_to((
                    8.7390125, 48.932494, 5.7435485, 46.404455, 5.3230575,
                    44.079526,
                )) //  C
                .cubic_curve_to((
                    4.3067545, 40.577398, 7.1279375, 35.698781, 11.06621,
                    35.699006,
                )) //  C
                .cubic_curve_to((
                    12.294152, 36.031677, 13.623987, 36.173304, 11.829475,
                    34.990644,
                )) //  C
                .cubic_curve_to((
                    9.3676865, 33.307515, 5.8037275, 33.58719, 3.3704395,
                    35.210424,
                )) // C
                .cubic_curve_to((
                    1.0623275, 36.770655, 0.13929045, 39.754219, 0.82235245,
                    42.405366,
                )) // C
                .cubic_curve_to((
                    1.3046175, 45.566282, 3.6848405, 48.193108, 6.7213125,
                    49.129468,
                )) // C
                .cubic_curve_to((
                    9.7104595, 50.27834, 13.341653, 50.11482, 15.914542,
                    48.217129,
                )) // C
                .cubic_curve_to((
                    19.719853, 46.146325, 21.076807, 41.160698, 20.493903,
                    36.98712,
                )) // C
                .cubic_curve_to((
                    19.768413, 33.866569, 18.962052, 31.135444, 17.062178,
                    27.719868,
                )) // C
                .cubic_curve_to((
                    13.874366, 22.916047, 11.614725, 15.94564, 14.417225,
                    10.804622,
                )) //  C
                .cubic_curve_to((
                    16.06608, 7.3630031, 20.222278, 4.3452381, 24.61182,
                    4.2941071,
                )) // C
                .cubic_curve_to((
                    28.481951, 3.8427771, 31.966035, 6.1413621, 34.193283,
                    8.8995061,
                )) //  C
                .cubic_curve_to((
                    36.148768, 11.336879, 37.254951, 15.072932, 37.020528,
                    18.316145,
                )) //  C
                .cubic_curve_to((
                    36.875189, 23.066435, 34.111899, 27.803757, 31.749827,
                    31.409094,
                )) //  C
                .cubic_curve_to((
                    30.449695, 33.956638, 30.230795, 35.986132, 30.134148,
                    38.872811,
                )) // C
                .cubic_curve_to((
                    30.444635, 42.747977, 30.996602, 46.447766, 34.497823,
                    48.311979,
                )) // C
                .cubic_curve_to((
                    39.010912, 50.93287, 45.806814, 49.664633, 48.167235,
                    44.739305,
                )) // C
                .cubic_curve_to((
                    49.889663, 41.266664, 48.970202, 36.193929, 45.128703,
                    34.587162,
                )) // C
                .cubic_curve_to((
                    42.452396, 33.103059, 38.787955, 33.701495, 36.772945,
                    36.021578,
                )) // C
                .cubic_curve_to((
                    39.220943, 36.083499, 42.478682, 35.800383, 43.550367,
                    38.627459,
                )) //  C
                .cubic_curve_to((
                    45.035209, 41.557959, 44.955267, 46.150083, 41.581781,
                    47.730808,
                )) // C
                .cubic_curve_to((
                    38.774221, 48.917712, 35.844772, 46.333854, 35.289896,
                    43.637513,
                )) // C
                .cubic_curve_to((
                    34.508899, 39.933417, 34.607447, 35.836391, 36.486662,
                    32.458849,
                )) // C
                .cubic_curve_to((
                    37.966082, 29.775828, 40.327674, 27.64413, 41.390812,
                    24.709601,
                )) //  C
                .cubic_curve_to((
                    43.759608, 19.387289, 42.683624, 13.015331, 39.237443,
                    8.4147421,
                )) // C
                .cubic_curve_to((
                    36.010865, 3.7053851, 30.404865, 0.8651331, 24.706606,
                    1.0049331,
                )) // C
                .cubic_curve_to((
                    23.52701, 0.9920241, 22.347291, 1.0659511, 21.176252,
                    1.2059591,
                )) // C
                .close(); // z
            let path = Path::new()
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path)
                .add(is_retrograde(true));
        },
        Bodies::Chiron => {
            size = (50.0, 50.0);
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
                .set("stroke", "black")
                .set("stroke-width", 7)
                .set("d", data1)
                .set(
                    "transform",
                    "matrix(0.408124,0,0,0.408124,168.3595,224.7729)",
                );
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
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
                .add(is_retrograde(true));
        },
        _ => {
            document = Document::new().set("viewBox", (0, 0, 50, 50));
        },
    }
    document
}

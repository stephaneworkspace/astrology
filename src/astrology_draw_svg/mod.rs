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
extern crate base64;
extern crate strum;
use libswe_sys::sweconst::{Bodies, Signs};
use svg::node::element::path::Data;
use svg::node::element::path::Number;
use svg::node::element::{Circle, Path};
use svg::Document;
pub mod svg_draw_bodies;
pub mod svg_draw_numbers;
pub mod svg_draw_signs;
use base64::encode;
use svg_draw_bodies::draw_bodie;
use svg_draw_numbers::draw_degre;
use svg_draw_signs::draw_sign;
//use strum::{AsStaticRef, IntoEnumIterator};
use std::fs::File;
use std::io::prelude::*;
//use strum::AsStaticRef;
pub mod html_draw;
pub mod svg_draw;
use svg_draw::*;
/// Create a html file with the natal chart
pub fn chart_html(
    max_size: Number,
    path_and_file_export: &str,
) -> std::io::Result<()> {
    // File
    let mut file = File::create(path_and_file_export)?;

    // Object calc draw for calcul in svg x,y width, height
    let ws = svg_draw::WorkingStorage::new(max_size);
    let ws_draw = svg_draw::WorkingStorageDraw::new(ws.clone());
    let document = format!(
        r#"
        {}
        <body>
            <center>
                <div style="height: {}px; width: {}px">
                    <div 
                        class="element svg-base" 
                        style="background-image:url('data:image/svg+xml;base64,{}')"
                    >
                    <!--{}-->
                    <!--
                    Sun
                    {}
                    -->
                    <!--
                    Moon
                    {}
                    -->
                    <!--
                    Mercury
                    {}
                    -->
                    <!--
                    Venus
                    {}
                    -->
                    <!--
                    Mars
                    {}
                    -->
                    <!--
                    Jupiter
                    {}
                    -->
                    <!--
                    Saturn
                    {}
                    -->
                    <!--
                    Uranus
                    {}
                    -->
                    <!--
                    Neptune
                    {}
                    -->
                    <!--
                    Pluto
                    {}
                    -->
                    <!--
                    Nord Node
                    {}
                    -->
                    <!--
                    Chiron
                    {}
                    -->
                    <!--
                    Aries
                    {}
                    -->
                    <!--
                    Taurus
                    {}
                    -->
                    <!--
                    Gemini
                    {}
                    -->
                    <!--
                    Cancer
                    {}
                    -->
                    <!--
                    Leo
                    {}
                    -->
                    <!--
                    Virgo
                    {}
                    -->
                    <!--
                    Libra
                    {}
                    -->
                    <!--
                    Scorpio
                    {}
                    -->
                    <!--
                    Sagittarius
                    {}
                    -->
                    <!--
                    Capricorn
                    {}
                    -->
                    <!--
                    Aquarius
                    {}
                    -->
                    <!--
                    Pisces
                    {}
                    -->
                    <!--
                    0°
                    {}
                    -->
                    <!--
                    1°
                    {}
                    -->
                    <!--
                    2°
                    {}
                    -->
                    <!--
                    3°
                    {}
                    -->
                    <!--
                    4°
                    {}
                    -->
                    <!--
                    5°
                    {}
                    -->
                    <!--
                    6°
                    {}
                    -->
                    <!--
                    7°
                    {}
                    -->
                    <!--
                    8°
                    {}
                    -->
                    </div>
                </div>
            </center>
        </body>
    </html>
    "#,
        html_draw::HTML_HEAD,
        ws.max_size.clone(),
        ws.max_size.clone(),
        encode(&ws_draw.draw_base().to_string()),
        ws_draw.draw_base(),
        draw_bodie(Bodies::Sun),
        draw_bodie(Bodies::Moon),
        draw_bodie(Bodies::Mercury),
        draw_bodie(Bodies::Venus),
        draw_bodie(Bodies::Mars),
        draw_bodie(Bodies::Jupiter),
        draw_bodie(Bodies::Saturn),
        draw_bodie(Bodies::Uranus),
        draw_bodie(Bodies::Neptune),
        draw_bodie(Bodies::Pluto),
        draw_bodie(Bodies::TrueNode),
        draw_bodie(Bodies::Chiron),
        draw_sign(Signs::Aries),
        draw_sign(Signs::Taurus),
        draw_sign(Signs::Gemini),
        draw_sign(Signs::Cancer),
        draw_sign(Signs::Leo),
        draw_sign(Signs::Virgo),
        draw_sign(Signs::Libra),
        draw_sign(Signs::Scorpio),
        draw_sign(Signs::Sagittarius),
        draw_sign(Signs::Capricorn),
        draw_sign(Signs::Aquarius),
        draw_sign(Signs::Pisces),
        draw_degre(0),
        draw_degre(1),
        draw_degre(2),
        draw_degre(3),
        draw_degre(4),
        draw_degre(5),
        draw_degre(6),
        draw_degre(7),
        draw_degre(8)
    );

    if path_and_file_export != "" {
        file.write_all(&document.as_bytes())?;
    }
    //println!("{}", document.clone().to_string());
    Ok(())
}

/// Actualy create one svg with path, but in future export a Vec<SomeStruct>
/// Now i'm concentred in char_html
pub fn chart(max_size: Number, path_export: &str) -> String {
    let calc_draw = svg_draw::WorkingStorage::new(max_size);
    // Center circle
    // let center: (f64, f64) =
    //    (calc_draw.get_radius_total(), calc_draw.get_radius_total());
    let center: (Number, Number) = (
        calc_draw.get_radius_total() as Number,
        calc_draw.get_radius_total() as Number,
    );

    /*
    // Circle visible to draw
    let mut circle: Vec<Data> = Vec::new();
    let mut ell: Vec<Number> = Vec::new();
    ell.push(30.0);
    ell.push(20.0);
    //let ell_arc = EllipticalArc(Position::Absolute, Parameters::from(ell));
    circle.push(
        Data::new()
            .move_to(center)
            .elliptical_arc_by(Parameters::from(ell))
            /*.elliptical_arc_to(EllipticalArc(
                Position::Absolute,
                Parameters::from(ell),
            ))*/
            .close(),
    );*/

    let data1 = Circle::new()
        .set("fill", "none")
        .set("cx", center.0)
        .set("cy", center.1)
        .set("r", calc_draw.get_radius_circle(0).0)
        .set("stroke", "black")
        .set("stroke-width", 3);

    let data2 = Circle::new()
        .set("fill", "none")
        .set("cx", center.0)
        .set("cy", center.1)
        .set("r", calc_draw.get_radius_circle(1).0)
        .set("stroke", "red")
        .set("stroke-width", 2);

    /*let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        //.set("d", circle[0].clone());
        .set("d", moon);
    */

    let document = Document::new()
        .set("baseProfile", "full")
        .set("version", "1.1")
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink")
        .set("viewBox", (0, 0, max_size as i32, max_size as i32))
        .add(data1)
        .add(data2);
    if path_export != "" {
        svg::save(format!("{}{}", path_export, "image.svg"), &document)
            .unwrap();
    }
    //document.to_string()
    let html = format!(
        r#"
        <!DOCTYPE html>
        <meta charset="utf8">
        <head>
        <title>Astrology</title>
        <style>
        .svg-base {{
            background-repeat: no-repeat;
        }}
        .element {{
            position: absolute; 
            width: 100%; 
            height: 100%; 
            display: flex; 
            justify-content: center; 
        }}
        </style>
        </head>
        <h1>Astral chart</h1>
        <center>
            <div style="height: {}px; width: {}px">
                <div 
                    class="element svg-base" 
                    style="background-image:url('data:image/svg+xml;base64,{}')"
                >
                <!--{}-->
                </div>
            </div>
        </center>
    "#,
        max_size as i32,
        max_size as i32,
        encode(&document.to_string()),
        &document.to_string()
    );
    html
}

/// Example (not used, to be deleted in future)
pub fn write() -> String {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 70, 70)).add(path);

    //svg::save("image.svg", &document).unwrap();
    document.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

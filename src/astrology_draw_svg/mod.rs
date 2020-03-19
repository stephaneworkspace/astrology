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
extern crate libswe_sys;
extern crate serde;
// extern crate serde_derive; // Deserialize
// extern crate serde_json; // Deserialize
extern crate strum;
use libswe_sys::sweconst::{
    Angle, Bodies, Calandar, Object, ObjectType, OptionalFlag, Signs,
};
use libswe_sys::swerust;
use serde::Deserialize;
use svg::node::element::path::Data;
use svg::node::element::path::Number;
use svg::node::element::Path;
use svg::Document;
pub mod svg_draw_angle;
pub mod svg_draw_bodies;
pub mod svg_draw_house;
pub mod svg_draw_numbers;
pub mod svg_draw_zodiac;
use base64::encode;
use std::fs::File;
use std::io::prelude::*;
use strum::AsStaticRef;
use svg_draw_angle::{draw_asc, draw_desc, draw_fc, draw_mc};
use svg_draw_numbers::{draw_degre, draw_minute};
pub mod html_draw;
pub mod svg_draw;
use crate::DataChartNatalC;
use serde::Serialize;
use strum::IntoEnumIterator;
use svg_draw::*;

#[derive(Debug, Deserialize)]
pub struct DataChartNatal {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hourf32: f32,
    pub hour: i32,
    pub min: i32,
    pub sec: f32,
    pub lat: f32,
    pub lng: f32,
}

/// Put the struct/enum in const file in future
#[derive(Serialize, Deserialize, Debug)]
pub struct DataObjectSvg {
    pub svg: String,
    pub object_type: DataObjectType,
    pub size_x: f32,
    pub size_y: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataObjectType {
    Angle,
    AngleDeg,
    AngleMin,
    AngleTrait,
    Chart,
    House,
    Zodiac,
    Planet,
    PlanetDeg,
    PlanetMin,
    PlanetTrait,
}

/// Create a chart for C export
/// Without path like chart_html for now
pub fn chart(max_size: Number, data: DataChartNatalC) -> Vec<DataObjectSvg> {
    // Natal chart
    println!("Version swephem: {}", swerust::handler_swe02::version());
    /*let swe02_path: &str =
        "/Users/stephanebressani/Code/Rust/libswe-sys/src/swisseph/sweph/";
    swerust::handler_swe02::set_ephe_path(&swe02_path);*/
    println!("{}", data.year);
    println!(
        "Library path (Todo): {}",
        swerust::handler_swe02::get_library_path()
    );
    // House natal chart
    println!("Hsys: {}", swerust::handler_swe14::house_name('P')); // Placidus
    let utc_time_zone: swerust::handler_swe08::UtcTimeZoneResult =
        swerust::handler_swe08::utc_time_zone(
            data.year,
            data.month,
            data.day,
            data.hour,
            data.min,
            data.sec as f64, // need to change libswe_sys f64 -> f32
            2.0,
        ); // 2.0 = Timezone -> to compute
    println!("UtcTimeZone: {:?}", utc_time_zone);
    let utc_to_jd: swerust::handler_swe08::UtcToJdResult =
        swerust::handler_swe08::utc_to_jd(
            utc_time_zone.year[0],
            utc_time_zone.month[0],
            utc_time_zone.day[0],
            utc_time_zone.hour[0],
            utc_time_zone.min[0],
            utc_time_zone.sec[0],
            Calandar::Gregorian,
        );
    println!("GregorianTimeZone: {:?}", utc_to_jd);
    let house_result = swerust::handler_swe14::houses(
        utc_to_jd.julian_day_ut,
        data.lat as f64, // Todo in libswe_sys f64 -> f32
        data.lng as f64, // Todo in libswe_sys f64 -> f32
        'P',             // Placidus
    );

    let mut object: Vec<Object> = Vec::new();
    let mut calc: swerust::handler_swe03::CalcUtResult;
    for bodies in Bodies::iter() {
        if bodies.clone().object_type() == ObjectType::PlanetOrStar {
            calc = swerust::handler_swe03::calc_ut(
                utc_to_jd.julian_day_ut, // debug julianday in orginal file
                bodies.clone(),
                OptionalFlag::Speed as i32,
            );
            object.push(Object::new(
                bodies.clone(),
                bodies.clone().as_static(),
                bodies.clone().object_type(),
                calc.longitude,
                calc.latitude,
                calc.speed_longitude,
            ));
        }
    }

    // Object calc draw for calcul in svg x,y width, height
    let ws = svg_draw::WorkingStorage::new(max_size, house_result, object);
    let ws_draw = svg_draw::WorkingStorageDraw::new(ws.clone());

    let mut res: Vec<DataObjectSvg> = Vec::new();

    // Chart
    res.push(DataObjectSvg {
        svg: ws_draw.draw_base().to_string(),
        object_type: DataObjectType::Chart,
        size_x: max_size as f32,
        size_y: max_size as f32,
        pos_x: 0.0,
        pos_y: 0.0,
    });

    // Zodiac
    for sign in Signs::iter() {
        let draw = ws_draw.draw_zodiac(sign);
        res.push(DataObjectSvg {
            svg: draw.svg,
            object_type: DataObjectType::Zodiac,
            size_x: draw.size_x as f32,
            size_y: draw.size_y as f32,
            pos_x: draw.pos_x as f32,
            pos_y: draw.pos_y as f32,
        });
    }

    for i in 0..12 {
        let draw = ws_draw.draw_house(i + 1);
        res.push(DataObjectSvg {
            svg: draw.svg,
            object_type: DataObjectType::House,
            size_x: draw.size_x as f32,
            size_y: draw.size_y as f32,
            pos_x: draw.pos_x as f32,
            pos_y: draw.pos_y as f32,
        });
    }

    for a in Angle::iter() {
        if a == Angle::Asc || a == Angle::Mc
        //  || a == Angle::Fc
        //  || a == Angle::Desc
        {
            let draw = ws_draw.draw_angle(a);
            res.push(DataObjectSvg {
                svg: draw.svg,
                object_type: DataObjectType::Angle,
                size_x: draw.size_x as f32,
                size_y: draw.size_y as f32,
                pos_x: draw.pos_x as f32,
                pos_y: draw.pos_y as f32,
            });
            res.push(DataObjectSvg {
                svg: draw.deg_svg,
                object_type: DataObjectType::AngleDeg,
                size_x: draw.deg_size_x as f32,
                size_y: draw.deg_size_y as f32,
                pos_x: draw.deg_pos_x as f32,
                pos_y: draw.deg_pos_y as f32,
            });
            res.push(DataObjectSvg {
                svg: draw.min_svg,
                object_type: DataObjectType::AngleMin,
                size_x: draw.min_size_x as f32,
                size_y: draw.min_size_y as f32,
                pos_x: draw.min_pos_x as f32,
                pos_y: draw.min_pos_y as f32,
            });
            res.push(DataObjectSvg {
                svg: draw.trait_svg,
                object_type: DataObjectType::AngleTrait,
                size_x: draw.trait_size_x as f32,
                size_y: draw.trait_size_y as f32,
                pos_x: draw.trait_pos_x as f32,
                pos_y: draw.trait_pos_y as f32,
            });
        }
    }
    for b in Bodies::iter() {
        if b == Bodies::Sun
            || b == Bodies::Moon
            || b == Bodies::Mars
            || b == Bodies::Mercury
            || b == Bodies::Venus
            || b == Bodies::Jupiter
            || b == Bodies::Saturn
            || b == Bodies::Uranus
            || b == Bodies::Neptune
            || b == Bodies::Pluto
        //          || b == Bodies::TrueNode
        //          || b == Bodies::Chiron
        {
            let draw = ws_draw.draw_bodie(b);
            res.push(DataObjectSvg {
                svg: draw.svg,
                object_type: DataObjectType::Planet,
                size_x: draw.size_x as f32,
                size_y: draw.size_y as f32,
                pos_x: draw.pos_x as f32,
                pos_y: draw.pos_y as f32,
            });
            res.push(DataObjectSvg {
                svg: draw.deg_svg,
                object_type: DataObjectType::PlanetDeg,
                size_x: draw.deg_size_x as f32,
                size_y: draw.deg_size_y as f32,
                pos_x: draw.deg_pos_x as f32,
                pos_y: draw.deg_pos_y as f32,
            });
            res.push(DataObjectSvg {
                svg: draw.min_svg,
                object_type: DataObjectType::PlanetMin,
                size_x: draw.min_size_x as f32,
                size_y: draw.min_size_y as f32,
                pos_x: draw.min_pos_x as f32,
                pos_y: draw.min_pos_y as f32,
            });
            res.push(DataObjectSvg {
                svg: draw.trait_svg,
                object_type: DataObjectType::PlanetTrait,
                size_x: draw.trait_size_x as f32,
                size_y: draw.trait_size_y as f32,
                pos_x: draw.trait_pos_x as f32,
                pos_y: draw.trait_pos_y as f32,
            });
        }
    }

    res
}

/// Create a html file with the natal chart
pub fn chart_html(
    max_size: Number,
    path_and_file_export: &str,
    data: DataChartNatal,
) -> std::io::Result<()> {
    // File
    let mut file = File::create(path_and_file_export)?;

    // Natal chart
    println!("Version swephem: {}", swerust::handler_swe02::version());
    let swe02_path: &str =
        "/Users/stephanebressani/Code/Rust/libswe-sys/src/swisseph/sweph/";
    swerust::handler_swe02::set_ephe_path(&swe02_path);
    println!("{}", data.year);
    println!(
        "Library path (Todo): {}",
        swerust::handler_swe02::get_library_path()
    );
    // House natal chart
    println!("Hsys: {}", swerust::handler_swe14::house_name('P')); // Placidus
    let utc_time_zone: swerust::handler_swe08::UtcTimeZoneResult =
        swerust::handler_swe08::utc_time_zone(
            data.year,
            data.month,
            data.day,
            data.hour,
            data.min,
            data.sec as f64, // need to change libswe_sys f64 -> f32
            2.0,
        ); // 2.0 = Timezone -> to compute
    println!("UtcTimeZone: {:?}", utc_time_zone);
    let utc_to_jd: swerust::handler_swe08::UtcToJdResult =
        swerust::handler_swe08::utc_to_jd(
            utc_time_zone.year[0],
            utc_time_zone.month[0],
            utc_time_zone.day[0],
            utc_time_zone.hour[0],
            utc_time_zone.min[0],
            utc_time_zone.sec[0],
            Calandar::Gregorian,
        );
    println!("GregorianTimeZone: {:?}", utc_to_jd);
    let house_result = swerust::handler_swe14::houses(
        utc_to_jd.julian_day_ut,
        data.lat as f64, // Todo in libswe_sys f64 -> f32
        data.lng as f64, // Todo in libswe_sys f64 -> f32
        'P',             // Placidus
    );
    let object: Vec<Object> = Vec::new();
    // Object calc draw for calcul in svg x,y width, height
    let ws = svg_draw::WorkingStorage::new(max_size, house_result, object);
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
                    <!--
                    9°
                    {}
                    -->
                    <!--
                    10°
                    {}
                    -->
                    <!--
                    11°
                    {}
                    -->
                    <!--
                    12°
                    {}
                    -->
                    <!--
                    13°
                    {}
                    -->
                    <!--
                    14°
                    {}
                    -->
                    <!--
                    15°
                    {}
                    -->
                    <!--
                    16°
                    {}
                    -->
                    <!--
                    17°
                    {}
                    -->
                    <!--
                    18°
                    {}
                    -->
                    <!--
                    19°
                    {}
                    -->
                    <!--
                    20°
                    {}
                    -->
                    <!--
                    21°
                    {}
                    -->
                    <!--
                    22°
                    {}
                    -->
                    <!--
                    23°
                    {}
                    -->
                    <!--
                    24°
                    {}
                    -->
                    <!--
                    25°
                    {}
                    -->
                    <!--
                    26°
                    {}
                    -->
                    <!--
                    27°
                    {}
                    -->
                    <!--
                    28°
                    {}
                    -->
                    <!--
                    29°
                    {}
                    -->
                    <!--
                    30°
                    {}
                    -->
                    <!--
                    0'
                    {}
                    -->
                    <!--
                    1'
                    {}
                    -->
                    <!--
                    2'
                    {}
                    -->
                    <!--
                    3'
                    {}
                    -->
                    <!--
                    4'
                    {}
                    -->
                    <!--
                    5'
                    {}
                    -->
                    <!--
                    6'
                    {}
                    -->
                    <!--
                    7'
                    {}
                    -->
                    <!--
                    8'
                    {}
                    -->
                    <!--
                    9'
                    {}
                    -->
                    <!--
                    10'
                    {}
                    -->
                    <!--
                    11'
                    {}
                    -->
                    <!--
                    12'
                    {}
                    -->
                    <!--
                    13'
                    {}
                    -->
                    <!--
                    14'
                    {}
                    -->
                    <!--
                    15'
                    {}
                    -->
                    <!--
                    16'
                    {}
                    -->
                    <!--
                    17'
                    {}
                    -->
                    <!--
                    18'
                    {}
                    -->
                    <!--
                    19'
                    {}
                    -->
                    <!--
                    20'
                    {}
                    -->
                    <!--
                    21'
                    {}
                    -->
                    <!--
                    22'
                    {}
                    -->
                    <!--
                    23'
                    {}
                    -->
                    <!--
                    24'
                    {}
                    -->
                    <!--
                    25'
                    {}
                    -->
                    <!--
                    26'
                    {}
                    -->
                    <!--
                    27'
                    {}
                    -->
                    <!--
                    28'
                    {}
                    -->
                    <!--
                    29'
                    {}
                    -->
                    <!--
                    30'
                    {}
                    -->
                    <!--
                    31'
                    {}
                    -->
                    <!--
                    32'
                    {}
                    -->
                    <!--
                    33'
                    {}
                    -->
                    <!--
                    34'
                    {}
                    -->
                    <!--
                    35'
                    {}
                    -->
                    <!--
                    36'
                    {}
                    -->
                    <!--
                    37'
                    {}
                    -->
                    <!--
                    38'
                    {}
                    -->
                    <!--
                    39'
                    {}
                    -->
                    <!--
                    40'
                    {}
                    -->
                    <!--
                    41'
                    {}
                    -->
                    <!--
                    42'
                    {}
                    -->
                    <!--
                    43'
                    {}
                    -->
                    <!--
                    44'
                    {}
                    -->
                    <!--
                    45'
                    {}
                    -->
                    <!--
                    46'
                    {}
                    -->
                    <!--
                    47'
                    {}
                    -->
                    <!--
                    48'
                    {}
                    -->
                    <!--
                    49'
                    {}
                    -->
                    <!--
                    50'
                    {}
                    -->
                    <!--
                    51'
                    {}
                    -->
                    <!--
                    52'
                    {}
                    -->
                    <!--
                    53'
                    {}
                    -->
                    <!--
                    54'
                    {}
                    -->
                    <!--
                    55'
                    {}
                    -->
                    <!--
                    56'
                    {}
                    -->
                    <!--
                    57'
                    {}
                    -->
                    <!--
                    58'
                    {}
                    -->
                    <!--
                    59'
                    {}
                    -->
                    <!--
                    House 1
                    {}
                    -->
                    <!--
                    House 2
                    {}
                    -->
                    <!--
                    House 3
                    {}
                    -->
                    <!--
                    House 4
                    {}
                    -->
                    <!--
                    House 5
                    {}
                    -->
                    <!--
                    House 6
                    {}
                    -->
                    <!--
                    House 7
                    {}
                    -->
                    <!--
                    House 8
                    {}
                    -->
                    <!--
                    House 9
                    {}
                    -->
                    <!--
                    House 10
                    {}
                    -->
                    <!--
                    House 11
                    {}
                    -->
                    <!--
                    House 12
                    {}
                    -->
                    <!--
                    AC
                    {}
                    -->
                    <!--
                    FC
                    {}
                    -->
                    <!--
                    DC
                    {}
                    -->
                    <!--
                    MC
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
        ws_draw.draw_bodie(Bodies::Sun).svg,
        ws_draw.draw_bodie(Bodies::Moon).svg,
        ws_draw.draw_bodie(Bodies::Mercury).svg,
        ws_draw.draw_bodie(Bodies::Venus).svg,
        ws_draw.draw_bodie(Bodies::Mars).svg,
        ws_draw.draw_bodie(Bodies::Jupiter).svg,
        ws_draw.draw_bodie(Bodies::Saturn).svg,
        ws_draw.draw_bodie(Bodies::Uranus).svg,
        ws_draw.draw_bodie(Bodies::Neptune).svg,
        ws_draw.draw_bodie(Bodies::Pluto).svg,
        ws_draw.draw_bodie(Bodies::TrueNode).svg,
        ws_draw.draw_bodie(Bodies::Chiron).svg,
        ws_draw.draw_zodiac(Signs::Aries).svg,
        ws_draw.draw_zodiac(Signs::Taurus).svg,
        ws_draw.draw_zodiac(Signs::Gemini).svg,
        ws_draw.draw_zodiac(Signs::Cancer).svg,
        ws_draw.draw_zodiac(Signs::Leo).svg,
        ws_draw.draw_zodiac(Signs::Virgo).svg,
        ws_draw.draw_zodiac(Signs::Libra).svg,
        ws_draw.draw_zodiac(Signs::Scorpio).svg,
        ws_draw.draw_zodiac(Signs::Sagittarius).svg,
        ws_draw.draw_zodiac(Signs::Capricorn).svg,
        ws_draw.draw_zodiac(Signs::Aquarius).svg,
        ws_draw.draw_zodiac(Signs::Pisces).svg,
        draw_degre(0),
        draw_degre(1),
        draw_degre(2),
        draw_degre(3),
        draw_degre(4),
        draw_degre(5),
        draw_degre(6),
        draw_degre(7),
        draw_degre(8),
        draw_degre(9),
        draw_degre(10),
        draw_degre(11),
        draw_degre(12),
        draw_degre(13),
        draw_degre(14),
        draw_degre(15),
        draw_degre(16),
        draw_degre(17),
        draw_degre(18),
        draw_degre(19),
        draw_degre(20),
        draw_degre(21),
        draw_degre(22),
        draw_degre(23),
        draw_degre(24),
        draw_degre(25),
        draw_degre(26),
        draw_degre(27),
        draw_degre(28),
        draw_degre(29),
        draw_degre(30),
        draw_minute(0),
        draw_minute(1),
        draw_minute(2),
        draw_minute(3),
        draw_minute(4),
        draw_minute(5),
        draw_minute(6),
        draw_minute(7),
        draw_minute(8),
        draw_minute(9),
        draw_minute(10),
        draw_minute(11),
        draw_minute(12),
        draw_minute(13),
        draw_minute(14),
        draw_minute(15),
        draw_minute(16),
        draw_minute(17),
        draw_minute(18),
        draw_minute(19),
        draw_minute(20),
        draw_minute(21),
        draw_minute(22),
        draw_minute(23),
        draw_minute(24),
        draw_minute(25),
        draw_minute(26),
        draw_minute(27),
        draw_minute(28),
        draw_minute(29),
        draw_minute(30),
        draw_minute(31),
        draw_minute(32),
        draw_minute(33),
        draw_minute(34),
        draw_minute(35),
        draw_minute(36),
        draw_minute(37),
        draw_minute(38),
        draw_minute(39),
        draw_minute(40),
        draw_minute(41),
        draw_minute(42),
        draw_minute(43),
        draw_minute(44),
        draw_minute(45),
        draw_minute(46),
        draw_minute(47),
        draw_minute(48),
        draw_minute(49),
        draw_minute(50),
        draw_minute(51),
        draw_minute(52),
        draw_minute(53),
        draw_minute(54),
        draw_minute(55),
        draw_minute(56),
        draw_minute(57),
        draw_minute(58),
        draw_minute(59),
        ws_draw.draw_house(1).svg,
        ws_draw.draw_house(2).svg,
        ws_draw.draw_house(3).svg,
        ws_draw.draw_house(4).svg,
        ws_draw.draw_house(5).svg,
        ws_draw.draw_house(6).svg,
        ws_draw.draw_house(7).svg,
        ws_draw.draw_house(8).svg,
        ws_draw.draw_house(9).svg,
        ws_draw.draw_house(10).svg,
        ws_draw.draw_house(11).svg,
        ws_draw.draw_house(12).svg,
        draw_asc(),
        draw_fc(),
        draw_desc(),
        draw_mc()
    );

    if path_and_file_export != "" {
        file.write_all(&document.as_bytes())?;
    }
    //println!("{}", document.clone().to_string());
    Ok(())
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

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
use libswe_sys::sweconst::{Bodies, Calandar, Signs};
use libswe_sys::swerust;
use serde::Deserialize;
use svg::node::element::path::Data;
use svg::node::element::path::Number;
use svg::node::element::Path;
use svg::Document;
pub mod svg_draw_bodies;
pub mod svg_draw_house;
pub mod svg_draw_numbers;
pub mod svg_draw_signs;
use base64::encode;
use svg_draw_bodies::draw_bodie;
use svg_draw_house::draw_house;
use svg_draw_numbers::{draw_degre, draw_minute};
use svg_draw_signs::draw_sign;
//use strum::{AsStaticRef, IntoEnumIterator};
use std::fs::File;
use std::io::prelude::*;
//use strum::AsStaticRef;
pub mod html_draw;
pub mod svg_draw;
use serde::Serialize;
use std::os::raw::{c_double, c_int};
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

pub struct DataChartNatalC {
    pub year: c_int,
    pub month: c_int,
    pub day: c_int,
    pub hourf32: c_double,
    pub hour: c_int,
    pub min: c_int,
    pub sec: c_double,
    pub lat: c_double,
    pub lng: c_double,
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
    Chart,
    House,
    Zodiac,
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

    // Object calc draw for calcul in svg x,y width, height
    let ws = svg_draw::WorkingStorage::new(max_size, house_result);
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
        res.push(DataObjectSvg {
            svg: draw_sign(sign).to_string(),
            object_type: DataObjectType::Zodiac,
            size_x: 100.0,
            size_y: 100.0,
            pos_x: 100.0,
            pos_y: 100.0,
        });
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

    // Object calc draw for calcul in svg x,y width, height
    let ws = svg_draw::WorkingStorage::new(max_size, house_result);
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
        draw_house(1),
        draw_house(2),
        draw_house(3),
        draw_house(4),
        draw_house(5),
        draw_house(6),
        draw_house(7),
        draw_house(8),
        draw_house(9),
        draw_house(10),
        draw_house(11),
        draw_house(12),
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

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
    Angle, Aspects, Bodies, Calandar, Object, ObjectType, OptionalFlag, Signs,
};
use libswe_sys::swerust;
use serde::Deserialize;
use svg::node::element::path::Data;
use svg::node::element::path::Number;
use svg::node::element::Path;
use svg::Document;
pub mod svg_draw_angle;
pub mod svg_draw_aspect;
pub mod svg_draw_bodies;
pub mod svg_draw_house;
pub mod svg_draw_numbers;
pub mod svg_draw_zodiac;
use base64::encode;
use std::fs::File;
use std::io::prelude::*;
use strum::AsStaticRef;
use svg_draw_angle::{draw_asc, draw_desc, draw_fc, draw_mc};
use svg_draw_aspect::{draw_aspect, maj_aspect, no_aspect};
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
    pub aspects: Vec<Aspects>, // If null no aspects
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataObjectType {
    Angle,
    AngleDeg,
    AngleMin,
    AngleTrait,
    Aspect,
    Chart,
    House,
    Zodiac,
    Planet,
    PlanetDeg,
    PlanetMin,
    PlanetTrait,
}

/// Put the struct/enum in const file in future
#[derive(Serialize, Deserialize, Debug)]
pub struct DataObjectAspectSvg {
    pub svg: String,
    pub text: String,
    pub aspects: Vec<Aspects>,
}

/// Create a chart for C export
/// Without path like chart_html for now
pub fn chart(
    max_size: Number,
    data: DataChartNatalC,
    path: &str,
) -> Vec<DataObjectSvg> {
    // Natal chart
    println!("Version swephem: {}", swerust::handler_swe02::version());
    //let swe02_path: &str =
    //    "/Users/stephanebressani/Code/Rust/libswe-sys/src/swisseph/sweph/";
    swerust::handler_swe02::set_ephe_path(&path);
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
    for bodie in Bodies::iter() {
        if bodie.clone().object_type() == ObjectType::PlanetOrStar
            || bodie.clone().object_type() == ObjectType::Fiction
        {
            calc = swerust::handler_swe03::calc_ut(
                utc_to_jd.julian_day_ut, // debug julianday in orginal file
                bodie.clone(),
                OptionalFlag::Speed as i32,
            );
            object.push(Object::new(
                bodie.clone(),
                bodie.clone().as_static(),
                bodie.clone().object_type(),
                calc.longitude,
                calc.latitude,
                calc.speed_longitude,
            ));
        }
    }

    // Object calc draw for calcul in svg x,y width, height
    let mut ws = svg_draw::WorkingStorage::new(max_size, house_result, object);
    ws.set_fix_compute();
    let ws_draw = svg_draw::WorkingStorageDraw::new(ws.clone());

    let mut res: Vec<DataObjectSvg> = Vec::new();

    let aspects_null: Vec<Aspects> = Vec::new();

    // Chart
    res.push(DataObjectSvg {
        svg: ws_draw.draw_base().to_string(),
        object_type: DataObjectType::Chart,
        size_x: max_size as f32,
        size_y: max_size as f32,
        pos_x: 0.0,
        pos_y: 0.0,
        aspects: aspects_null.clone(),
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
            aspects: aspects_null.clone(),
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
            aspects: aspects_null.clone(),
        });
    }

    for a in Angle::iter() {
        if ws.get_angle_is_on_chart(a.clone()) {
            let draw = ws_draw.draw_angle(a);
            res.push(DataObjectSvg {
                svg: draw.svg,
                object_type: DataObjectType::Angle,
                size_x: draw.size_x as f32,
                size_y: draw.size_y as f32,
                pos_x: draw.pos_x as f32,
                pos_y: draw.pos_y as f32,
                aspects: aspects_null.clone(),
            });
            res.push(DataObjectSvg {
                svg: draw.deg_svg,
                object_type: DataObjectType::AngleDeg,
                size_x: draw.deg_size_x as f32,
                size_y: draw.deg_size_y as f32,
                pos_x: draw.deg_pos_x as f32,
                pos_y: draw.deg_pos_y as f32,
                aspects: aspects_null.clone(),
            });
            res.push(DataObjectSvg {
                svg: draw.min_svg,
                object_type: DataObjectType::AngleMin,
                size_x: draw.min_size_x as f32,
                size_y: draw.min_size_y as f32,
                pos_x: draw.min_pos_x as f32,
                pos_y: draw.min_pos_y as f32,
                aspects: aspects_null.clone(),
            });
            res.push(DataObjectSvg {
                svg: draw.trait_svg,
                object_type: DataObjectType::AngleTrait,
                size_x: draw.trait_size_x as f32,
                size_y: draw.trait_size_y as f32,
                pos_x: draw.trait_pos_x as f32,
                pos_y: draw.trait_pos_y as f32,
                aspects: aspects_null.clone(),
            });
        }
    }
    for b in Bodies::iter() {
        if ws.get_bodie_is_on_chart(b.clone()) {
            let draw = ws_draw.draw_bodie(b);
            res.push(DataObjectSvg {
                svg: draw.svg,
                object_type: DataObjectType::Planet,
                size_x: draw.size_x as f32,
                size_y: draw.size_y as f32,
                pos_x: draw.pos_x as f32,
                pos_y: draw.pos_y as f32,
                aspects: aspects_null.clone(),
            });
            res.push(DataObjectSvg {
                svg: draw.deg_svg,
                object_type: DataObjectType::PlanetDeg,
                size_x: draw.deg_size_x as f32,
                size_y: draw.deg_size_y as f32,
                pos_x: draw.deg_pos_x as f32,
                pos_y: draw.deg_pos_y as f32,
                aspects: aspects_null.clone(),
            });
            res.push(DataObjectSvg {
                svg: draw.min_svg,
                object_type: DataObjectType::PlanetMin,
                size_x: draw.min_size_x as f32,
                size_y: draw.min_size_y as f32,
                pos_x: draw.min_pos_x as f32,
                pos_y: draw.min_pos_y as f32,
                aspects: aspects_null.clone(),
            });
            res.push(DataObjectSvg {
                svg: draw.trait_svg,
                object_type: DataObjectType::PlanetTrait,
                size_x: draw.trait_size_x as f32,
                size_y: draw.trait_size_y as f32,
                pos_x: draw.trait_pos_x as f32,
                pos_y: draw.trait_pos_y as f32,
                aspects: aspects_null.clone(),
            });
        }
    }
    // Aspects
    let mut asp_vec: Vec<Aspects> = Vec::new();
    let mut asp: u16;
    let mut abs_separation: Number;
    let mut separation: Number;
    let mut pair: Vec<(Bodies, Bodies)> = Vec::new();
    for bodie in ws.object.clone() {
        if ws.get_bodie_is_on_chart(bodie.object_enum) {
            for b in ws.object.clone() {
                let mut sw = false;
                for p in pair {
                    if (p.0 == bodie.object_enum && p.1 == b.object_enum)
                        || (p.0 == b.object_enum && p.1 == bodie.object_enum)
                    {
                        sw = true;
                        break;
                    }
                }
                if ws.get_bodie_is_on_chart(b.object_enum)
                    && !sw
                    && bodie.object_enum != b.object_enum
                {
                    pair.push((bodie.object_enum, b.object_enum));
                    separation = closestdistance(
                        ws.get_bodie_longitude(bodie.object_enum),
                        ws.get_bodie_longitude(b.object_enum),
                    );
                    abs_separation = separation.abs();
                    // Conjunction 0° - orbe 10°
                    /*println!(
                        "{}->{} / sep: {} / orb: {}",
                        bodie.object_name,
                        b.object_name,
                        abs_separation,
                        (abs_separation - 0 as f32).abs() // real ORBE HERE
                    );
                    println!(
                        "{}-120->{} / sep: {} / orb: {}",
                        bodie.object_name,
                        b.object_name,
                        abs_separation,
                        (abs_separation - 120 as f32).abs() // real ORBE HERE
                    );*/
                    asp = 0;
                    if (abs_separation - asp as f32).abs() <= 10.0 {
                        asp_vec.push(Aspects::Conjunction);
                        let draw = ws_draw.draw_aspect(
                            ws.get_bodie_longitude(bodie.object_enum),
                            ws.get_bodie_longitude(b.object_enum),
                            Aspects::Conjunction,
                        );
                        res.push(DataObjectSvg {
                            svg: draw.svg,
                            object_type: DataObjectType::Aspect,
                            size_x: draw.size_x as f32,
                            size_y: draw.size_y as f32,
                            pos_x: draw.pos_x as f32,
                            pos_y: draw.pos_y as f32,
                            aspects: asp_vec.clone(),
                        });
                        asp_vec.clear();
                    }
                    // Opposition 180° - orbe 8°
                    asp = 180;
                    if (abs_separation - asp as f32).abs() <= 8.0 {
                        asp_vec.push(Aspects::Opposition);
                        let draw = ws_draw.draw_aspect(
                            ws.get_bodie_longitude(bodie.object_enum),
                            ws.get_bodie_longitude(b.object_enum),
                            Aspects::Opposition,
                        );
                        res.push(DataObjectSvg {
                            svg: draw.svg,
                            object_type: DataObjectType::Aspect,
                            size_x: draw.size_x as f32,
                            size_y: draw.size_y as f32,
                            pos_x: draw.pos_x as f32,
                            pos_y: draw.pos_y as f32,
                            aspects: asp_vec.clone(),
                        });
                        asp_vec.clear();
                    }
                    // Trine 120° - orbe 7°
                    asp = 120;
                    if (abs_separation - asp as f32).abs() <= 7.0 {
                        asp_vec.push(Aspects::Trine);
                        let draw = ws_draw.draw_aspect(
                            ws.get_bodie_longitude(bodie.object_enum),
                            ws.get_bodie_longitude(b.object_enum),
                            Aspects::Trine,
                        );
                        res.push(DataObjectSvg {
                            svg: draw.svg,
                            object_type: DataObjectType::Aspect,
                            size_x: draw.size_x as f32,
                            size_y: draw.size_y as f32,
                            pos_x: draw.pos_x as f32,
                            pos_y: draw.pos_y as f32,
                            aspects: asp_vec.clone(),
                        });
                        asp_vec.clear();
                    }
                    // Square 90° - orbe 6°
                    asp = 90;
                    if (abs_separation - asp as f32).abs() <= 6.0 {
                        asp_vec.push(Aspects::Square);
                        let draw = ws_draw.draw_aspect(
                            ws.get_bodie_longitude(bodie.object_enum),
                            ws.get_bodie_longitude(b.object_enum),
                            Aspects::Square,
                        );
                        res.push(DataObjectSvg {
                            svg: draw.svg,
                            object_type: DataObjectType::Aspect,
                            size_x: draw.size_x as f32,
                            size_y: draw.size_y as f32,
                            pos_x: draw.pos_x as f32,
                            pos_y: draw.pos_y as f32,
                            aspects: asp_vec.clone(),
                        });
                        asp_vec.clear();
                    }
                    // Sextile 60° - orbe 5°
                    asp = 60;
                    if (abs_separation - asp as f32).abs() <= 5.0 {
                        asp_vec.push(Aspects::Sextile);
                        let draw = ws_draw.draw_aspect(
                            ws.get_bodie_longitude(bodie.object_enum),
                            ws.get_bodie_longitude(b.object_enum),
                            Aspects::Sextile,
                        );
                        res.push(DataObjectSvg {
                            svg: draw.svg,
                            object_type: DataObjectType::Aspect,
                            size_x: draw.size_x as f32,
                            size_y: draw.size_y as f32,
                            pos_x: draw.pos_x as f32,
                            pos_y: draw.pos_y as f32,
                            aspects: asp_vec.clone(),
                        });
                        asp_vec.clear();
                    }
                }
            } /*
              for i in 0..12 {
                  if i == 0 || i == 9 {
                      // Only Asc et Mc
                      asp = 0;
                      separation = closestdistance(
                          bodie.longitude as f32,
                          ws.house.clone()[i].longitude as f32,
                      );
                      abs_separation = separation.abs();
                      println!(
                          "{}->Angle{} / sep: {} / orb: {}",
                          bodie.object_name,
                          i,
                          separation,
                          (abs_separation - asp as f32).abs() // real ORBE HERE
                      );
                  }
              }*/
        }
    }
    res
}

pub fn all_aspects() -> Vec<DataObjectAspectSvg> {
    let mut res: Vec<DataObjectAspectSvg> = Vec::new();
    // No aspect
    let va_no_aspect: Vec<Aspects> = Vec::new();
    res.push(DataObjectAspectSvg {
        svg: no_aspect().to_string(),
        text: "No aspect".to_string(), // TO do const
        aspects: va_no_aspect,
    });

    // Maj aspects
    let mut va_maj_aspects: Vec<Aspects> = Vec::new();
    va_maj_aspects.push(Aspects::Conjunction);
    va_maj_aspects.push(Aspects::Opposition);
    va_maj_aspects.push(Aspects::Trine);
    va_maj_aspects.push(Aspects::Square);
    va_maj_aspects.push(Aspects::Sextile);
    res.push(DataObjectAspectSvg {
        svg: maj_aspect().to_string(),
        text: "Majors aspects".to_string(), // TO do const
        aspects: va_maj_aspects,
    });

    // Single Maj aspects
    for a in Aspects::iter() {
        let mut va: Vec<Aspects> = Vec::new();
        va.push(a as Aspects);
        res.push(DataObjectAspectSvg {
            svg: draw_aspect(a).to_string(),
            text: a.as_static().to_string(),
            aspects: va.clone(),
        });
        va.clear()
    }
    res
}

fn closestdistance(angle1: Number, angle2: Number) -> Number {
    znorm(angle2 - angle1)
}

fn znorm(mut angle: Number) -> Number {
    angle = angle % 360.0;
    if angle <= 180.0 {
        angle
    } else {
        angle - 360.0
    }
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
    let mut object: Vec<Object> = Vec::new();
    let mut calc: swerust::handler_swe03::CalcUtResult;
    for bodie in Bodies::iter() {
        calc = swerust::handler_swe03::calc_ut(
            utc_to_jd.julian_day_ut, // debug julianday in orginal file
            bodie.clone(),
            OptionalFlag::Speed as i32,
        );
        object.push(Object::new(
            bodie.clone(),
            bodie.clone().as_static(),
            bodie.clone().object_type(),
            calc.longitude,
            calc.latitude,
            calc.speed_longitude,
        ));
    }
    // Object calc draw for calcul in svg x,y width, height
    let mut ws = svg_draw::WorkingStorage::new(max_size, house_result, object);
    ws.set_fix_compute();
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
                    <!--
                    Aspect Opposition
                    {}
                    -->
                    <!--
                    Aspect Conjunction
                    {}
                    -->
                    <!--
                    Aspect Trine
                    {}
                    -->
                    <!--
                    Aspect Square
                    {}
                    -->
                    <!--
                    Aspect Sextile
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
        draw_degre(0, Bodies::EclNut),
        draw_degre(1, Bodies::EclNut),
        draw_degre(2, Bodies::EclNut),
        draw_degre(3, Bodies::EclNut),
        draw_degre(4, Bodies::EclNut),
        draw_degre(5, Bodies::EclNut),
        draw_degre(6, Bodies::EclNut),
        draw_degre(7, Bodies::EclNut),
        draw_degre(8, Bodies::EclNut),
        draw_degre(9, Bodies::EclNut),
        draw_degre(10, Bodies::EclNut),
        draw_degre(11, Bodies::EclNut),
        draw_degre(12, Bodies::EclNut),
        draw_degre(13, Bodies::EclNut),
        draw_degre(14, Bodies::EclNut),
        draw_degre(15, Bodies::EclNut),
        draw_degre(16, Bodies::EclNut),
        draw_degre(17, Bodies::EclNut),
        draw_degre(18, Bodies::EclNut),
        draw_degre(19, Bodies::EclNut),
        draw_degre(20, Bodies::EclNut),
        draw_degre(21, Bodies::EclNut),
        draw_degre(22, Bodies::EclNut),
        draw_degre(23, Bodies::EclNut),
        draw_degre(24, Bodies::EclNut),
        draw_degre(25, Bodies::EclNut),
        draw_degre(26, Bodies::EclNut),
        draw_degre(27, Bodies::EclNut),
        draw_degre(28, Bodies::EclNut),
        draw_degre(29, Bodies::EclNut),
        draw_degre(30, Bodies::EclNut),
        draw_minute(0, Bodies::EclNut),
        draw_minute(1, Bodies::EclNut),
        draw_minute(2, Bodies::EclNut),
        draw_minute(3, Bodies::EclNut),
        draw_minute(4, Bodies::EclNut),
        draw_minute(5, Bodies::EclNut),
        draw_minute(6, Bodies::EclNut),
        draw_minute(7, Bodies::EclNut),
        draw_minute(8, Bodies::EclNut),
        draw_minute(9, Bodies::EclNut),
        draw_minute(10, Bodies::EclNut),
        draw_minute(11, Bodies::EclNut),
        draw_minute(12, Bodies::EclNut),
        draw_minute(13, Bodies::EclNut),
        draw_minute(14, Bodies::EclNut),
        draw_minute(15, Bodies::EclNut),
        draw_minute(16, Bodies::EclNut),
        draw_minute(17, Bodies::EclNut),
        draw_minute(18, Bodies::EclNut),
        draw_minute(19, Bodies::EclNut),
        draw_minute(20, Bodies::EclNut),
        draw_minute(21, Bodies::EclNut),
        draw_minute(22, Bodies::EclNut),
        draw_minute(23, Bodies::EclNut),
        draw_minute(24, Bodies::EclNut),
        draw_minute(25, Bodies::EclNut),
        draw_minute(26, Bodies::EclNut),
        draw_minute(27, Bodies::EclNut),
        draw_minute(28, Bodies::EclNut),
        draw_minute(29, Bodies::EclNut),
        draw_minute(30, Bodies::EclNut),
        draw_minute(31, Bodies::EclNut),
        draw_minute(32, Bodies::EclNut),
        draw_minute(33, Bodies::EclNut),
        draw_minute(34, Bodies::EclNut),
        draw_minute(35, Bodies::EclNut),
        draw_minute(36, Bodies::EclNut),
        draw_minute(37, Bodies::EclNut),
        draw_minute(38, Bodies::EclNut),
        draw_minute(39, Bodies::EclNut),
        draw_minute(40, Bodies::EclNut),
        draw_minute(41, Bodies::EclNut),
        draw_minute(42, Bodies::EclNut),
        draw_minute(43, Bodies::EclNut),
        draw_minute(44, Bodies::EclNut),
        draw_minute(45, Bodies::EclNut),
        draw_minute(46, Bodies::EclNut),
        draw_minute(47, Bodies::EclNut),
        draw_minute(48, Bodies::EclNut),
        draw_minute(49, Bodies::EclNut),
        draw_minute(50, Bodies::EclNut),
        draw_minute(51, Bodies::EclNut),
        draw_minute(52, Bodies::EclNut),
        draw_minute(53, Bodies::EclNut),
        draw_minute(54, Bodies::EclNut),
        draw_minute(55, Bodies::EclNut),
        draw_minute(56, Bodies::EclNut),
        draw_minute(57, Bodies::EclNut),
        draw_minute(58, Bodies::EclNut),
        draw_minute(59, Bodies::EclNut),
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
        draw_mc(),
        draw_aspect(Aspects::Opposition),
        draw_aspect(Aspects::Conjunction),
        draw_aspect(Aspects::Trine),
        draw_aspect(Aspects::Square),
        draw_aspect(Aspects::Sextile)
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

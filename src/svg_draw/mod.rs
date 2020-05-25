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
extern crate strum;
use libswe_sys::sweconst::{
    Angle, Aspects, Bodies, Calandar, Language, Object, ObjectType,
    OptionalFlag, Signs, Theme,
};
use libswe_sys::swerust;
use serde::Deserialize;
use svg::node::element::path::Number;
pub mod angles;
pub mod aspects;
pub mod bodies;
pub mod houses;
pub mod numbers;
pub mod zodiacs;
use aspects::{
    aspects_all_aspects, aspects_draw, aspects_maj_aspects,
    aspects_min_aspects, aspects_no_aspect,
};
use strum::AsStaticRef;
pub mod svg_draw;
use serde::Serialize;
use strum::IntoEnumIterator;
use svg_draw::*;

/// Data chart
#[derive(Debug, Deserialize)]
pub struct DataChartNatal {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub min: i32,
    pub sec: f32,
    pub lat: f32,
    pub lng: f32,
}

/// Data object for json svg
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataObjectSvg {
    pub svg: String,
    pub object_type: DataObjectType,
    pub size_x: f32,
    pub size_y: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub aspects: Vec<Aspects>, // If null no aspects
}

/// Type of object used in struct DataObjectSvg
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

/// Create a chart
/// Without path like chart_html for now
pub fn chart(
    max_size: Number,
    data: DataChartNatal,
    path: &str,
    lang: Language,
) -> Vec<DataObjectSvg> {
    // To do theme
    let theme = Theme::Light;
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
            calc = if bodie.clone() == Bodies::FortunaPart {
                swerust::handler_swe03::calc_ut_fp(
                    utc_to_jd.julian_day_ut,
                    data.lat as f64,
                    data.lng as f64,
                    'P',
                    OptionalFlag::Speed as i32,
                )
            } else {
                swerust::handler_swe03::calc_ut(
                    utc_to_jd.julian_day_ut, // debug julianday in orginal file
                    bodie.clone(),
                    OptionalFlag::Speed as i32,
                )
            };
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
    let mut ws = svg_draw::WorkingStoragePolyMorphNatal::new(
        max_size,
        theme,
        lang,
        house_result,
        object,
    );
    ws.set_fix_compute(false);
    let ws_draw = svg_draw::WorkingStorageDrawPolyMorphNatal::new(ws.clone());

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
            let draw = ws_draw.draw_bodie(b, false);
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
    let mut orb: u16;
    let mut abs_separation: Number;
    let mut separation: Number;
    let mut pair: Vec<(Bodies, Bodies)> = Vec::new();
    for bodie in ws.object.clone() {
        if ws.get_bodie_is_on_chart(bodie.object_enum) {
            for b in ws.object.clone() {
                let mut sw = false;
                for p in pair.clone() {
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
                    separation = ws.get_closest_distance(
                        ws.get_bodie_longitude(bodie.object_enum, false),
                        ws.get_bodie_longitude(b.object_enum, false),
                    );
                    abs_separation = separation.abs();

                    for record_asp in Aspects::iter() {
                        asp = record_asp.angle().0;
                        orb = record_asp.angle().1;
                        if (abs_separation - asp as f32).abs() <= orb as f32 {
                            asp_vec.push(record_asp.clone());
                            let draw = ws_draw.draw_aspect(
                                ws.get_bodie_longitude(
                                    bodie.object_enum,
                                    false,
                                ),
                                ws.get_bodie_longitude(b.object_enum, false),
                                record_asp.clone(),
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
                }
            }
            for i in 0..12 {
                if i == 0 || i == 9 {
                    // Only Asc et Mc
                    separation = ws.get_closest_distance(
                        bodie.longitude as f32,
                        ws.house.clone()[i].longitude as f32,
                    );
                    abs_separation = separation.abs();
                    for record_asp in Aspects::iter() {
                        asp = record_asp.angle().0;
                        orb = record_asp.angle().1;
                        if (abs_separation - asp as f32).abs() <= orb as f32 {
                            asp_vec.push(record_asp.clone());
                            let draw = ws_draw.draw_aspect(
                                ws.get_bodie_longitude(
                                    bodie.object_enum,
                                    false,
                                ),
                                ws.get_angle_longitude(
                                    ws.house.clone()[i].angle,
                                ),
                                record_asp.clone(),
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
                }
            }
        }
    }
    res
}
/// Create a chart for C export
/// Without path like chart_html for now
pub fn chart_with_transit(
    max_size: Number,
    data: DataChartNatal,
    data_transit: DataChartNatal,
    path: &str,
    lang: Language,
) -> Vec<DataObjectSvg> {
    // To do better
    let theme: Theme = Theme::Light;
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
    let utc_time_zone_transit: swerust::handler_swe08::UtcTimeZoneResult =
        swerust::handler_swe08::utc_time_zone(
            data_transit.year,
            data_transit.month,
            data_transit.day,
            data_transit.hour,
            data_transit.min,
            data_transit.sec as f64, // need to change libswe_sys f64 -> f32
            2.0,
        ); // 2.0 = Timezone -> to compute
    println!("UtcTimeZone transit: {:?}", utc_time_zone_transit);
    let utc_to_jd_transit: swerust::handler_swe08::UtcToJdResult =
        swerust::handler_swe08::utc_to_jd(
            utc_time_zone_transit.year[0],
            utc_time_zone_transit.month[0],
            utc_time_zone_transit.day[0],
            utc_time_zone_transit.hour[0],
            utc_time_zone_transit.min[0],
            utc_time_zone_transit.sec[0],
            Calandar::Gregorian,
        );
    println!("GregorianTimeZonei transit: {:?}", utc_to_jd_transit);
    let house_result = swerust::handler_swe14::houses(
        utc_to_jd.julian_day_ut,
        data.lat as f64, // Todo in libswe_sys f64 -> f32
        data.lng as f64, // Todo in libswe_sys f64 -> f32
        'P',             // Placidus
    );

    let mut object: Vec<Object> = Vec::new();
    let mut object_transit: Vec<Object> = Vec::new();
    let mut calc: swerust::handler_swe03::CalcUtResult;
    for bodie in Bodies::iter() {
        if bodie.clone().object_type() == ObjectType::PlanetOrStar
            || bodie.clone().object_type() == ObjectType::Fiction
        {
            // Natal
            calc = if bodie.clone() == Bodies::FortunaPart {
                swerust::handler_swe03::calc_ut_fp(
                    utc_to_jd.julian_day_ut,
                    data.lat as f64,
                    data.lng as f64,
                    'P',
                    OptionalFlag::Speed as i32,
                )
            } else {
                swerust::handler_swe03::calc_ut(
                    utc_to_jd.julian_day_ut, // debug julianday in orginal file
                    bodie.clone(),
                    OptionalFlag::Speed as i32,
                )
            };
            object.push(Object::new(
                bodie.clone(),
                bodie.clone().as_static(),
                bodie.clone().object_type(),
                calc.longitude,
                calc.latitude,
                calc.speed_longitude,
            ));

            // Transit
            calc = if bodie.clone() == Bodies::FortunaPart {
                swerust::handler_swe03::calc_ut_fp(
                    utc_to_jd_transit.julian_day_ut,
                    data_transit.lat as f64,
                    data_transit.lng as f64,
                    'P',
                    OptionalFlag::Speed as i32,
                )
            } else {
                swerust::handler_swe03::calc_ut(
                    utc_to_jd_transit.julian_day_ut, // debug julianday
                    // in orginal file
                    bodie.clone(),
                    OptionalFlag::Speed as i32,
                )
            };
            object_transit.push(Object::new(
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
    let mut ws = WorkingStoragePolyMorphTransit::new(
        max_size,
        theme,
        lang,
        house_result,
        object,
        object_transit,
    );
    ws.set_fix_compute(true);
    ws.set_fix_compute(false);
    let ws_draw = WorkingStorageDrawPolyMorphTransit::new(ws.clone());

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
            /*
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
            });*/
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
            // Natal
            let draw = ws_draw.draw_bodie(b, false);
            res.push(DataObjectSvg {
                svg: draw.svg,
                object_type: DataObjectType::Planet,
                size_x: draw.size_x as f32,
                size_y: draw.size_y as f32,
                pos_x: draw.pos_x as f32,
                pos_y: draw.pos_y as f32,
                aspects: aspects_null.clone(),
            });
            /*
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
            });*/
            res.push(DataObjectSvg {
                svg: draw.trait_svg,
                object_type: DataObjectType::PlanetTrait,
                size_x: draw.trait_size_x as f32,
                size_y: draw.trait_size_y as f32,
                pos_x: draw.trait_pos_x as f32,
                pos_y: draw.trait_pos_y as f32,
                aspects: aspects_null.clone(),
            });

            // Transit
            let draw = ws_draw.draw_bodie(b, true);
            res.push(DataObjectSvg {
                svg: draw.svg,
                object_type: DataObjectType::Planet,
                size_x: draw.size_x as f32,
                size_y: draw.size_y as f32,
                pos_x: draw.pos_x as f32,
                pos_y: draw.pos_y as f32,
                aspects: aspects_null.clone(),
            });
            /*
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
            });*/
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
    let mut orb: u16;
    let mut abs_separation: Number;
    let mut separation: Number;
    let mut pair: Vec<(Bodies, Bodies)> = Vec::new();
    for bodie in ws.object_natal.clone() {
        if ws.get_bodie_is_on_chart(bodie.object_enum) {
            // Transit
            for bt in ws.object_transit.clone() {
                if ws.get_bodie_is_on_chart(bt.object_enum) {
                    separation = ws.get_closest_distance(
                        ws.get_bodie_longitude(bodie.object_enum, false), // no transit
                        ws.get_bodie_longitude(bt.object_enum, true), // transit
                    );
                    abs_separation = separation.abs();
                    for record_asp in Aspects::iter() {
                        asp = record_asp.angle().0;
                        orb = record_asp.angle().1;
                        if (abs_separation - asp as f32).abs() <= orb as f32 {
                            asp_vec.push(record_asp.clone());
                            let draw = ws_draw.draw_aspect(
                                ws.get_bodie_longitude(
                                    bodie.object_enum,
                                    false, // no transit
                                ),
                                ws.get_bodie_longitude(bt.object_enum, true), // transit
                                record_asp.clone(),
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
                }
            }
            // Regular
            for b in ws.object_natal.clone() {
                let mut sw = false;
                for p in pair.clone() {
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
                    separation = ws.get_closest_distance(
                        ws.get_bodie_longitude(bodie.object_enum, false),
                        ws.get_bodie_longitude(b.object_enum, false),
                    );
                    abs_separation = separation.abs();

                    for record_asp in Aspects::iter() {
                        asp = record_asp.angle().0;
                        orb = record_asp.angle().1;
                        if (abs_separation - asp as f32).abs() <= orb as f32 {
                            asp_vec.push(record_asp.clone());
                            let draw = ws_draw.draw_aspect(
                                ws.get_bodie_longitude(
                                    bodie.object_enum,
                                    false, // no transit
                                ),
                                ws.get_bodie_longitude(b.object_enum, false),
                                record_asp.clone(),
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
                }
            }
            // Transit
            for i in 0..12 {
                if i == 0 || i == 9 {
                    // Only Asc et Mc
                    separation = ws.get_closest_distance(
                        bodie.longitude as f32,
                        ws.house.clone()[i].longitude as f32,
                    );
                    abs_separation = separation.abs();
                    for record_asp in Aspects::iter() {
                        asp = record_asp.angle().0;
                        orb = record_asp.angle().1;
                        if (abs_separation - asp as f32).abs() <= orb as f32 {
                            asp_vec.push(record_asp.clone());
                            let draw = ws_draw.draw_aspect(
                                ws.get_bodie_longitude(
                                    bodie.object_enum,
                                    true, // true = transit
                                ),
                                ws.get_angle_longitude(
                                    ws.house.clone()[i].angle,
                                ),
                                record_asp.clone(),
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
                }
            }
            // Regular
            for i in 0..12 {
                if i == 0 || i == 9 {
                    // Only Asc et Mc
                    separation = ws.get_closest_distance(
                        bodie.longitude as f32,
                        ws.house.clone()[i].longitude as f32,
                    );
                    abs_separation = separation.abs();
                    for record_asp in Aspects::iter() {
                        asp = record_asp.angle().0;
                        orb = record_asp.angle().1;
                        if (abs_separation - asp as f32).abs() <= orb as f32 {
                            asp_vec.push(record_asp.clone());
                            let draw = ws_draw.draw_aspect(
                                ws.get_bodie_longitude(
                                    bodie.object_enum,
                                    false, // no transit
                                ),
                                ws.get_angle_longitude(
                                    ws.house.clone()[i].angle,
                                ),
                                record_asp.clone(),
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
                }
            }
        }
    }
    res
}

pub fn all_aspects(lang: Language) -> Vec<DataObjectAspectSvg> {
    let mut res: Vec<DataObjectAspectSvg> = Vec::new();
    // No aspect
    let va_no_aspect: Vec<Aspects> = Vec::new();
    let mut t;
    t = match lang {
        // To do const
        Language::English => "No aspect".to_string(),
        Language::French => "Pas d'aspect".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_no_aspect(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_no_aspect,
    });

    // Maj aspects
    let mut va_maj_aspects: Vec<Aspects> = Vec::new();
    for a in Aspects::iter() {
        if a.maj() {
            va_maj_aspects.push(a.clone());
        }
    }
    t = match lang {
        // To do const
        Language::English => "Majors aspects".to_string(),
        Language::French => "Aspects majeurs".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_maj_aspects(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_maj_aspects,
    });

    // Single Maj aspects
    for a in Aspects::iter() {
        if a.maj() {
            let mut va: Vec<Aspects> = Vec::new();
            va.push(a as Aspects);
            res.push(DataObjectAspectSvg {
                svg: aspects_draw(a, Theme::Light, lang).to_string(),
                text: a.text(lang),
                aspects: va.clone(),
            });
            va.clear()
        }
    }

    // Min aspects
    let mut va_min_aspects: Vec<Aspects> = Vec::new();
    for a in Aspects::iter() {
        if !a.maj() {
            va_min_aspects.push(a.clone());
        }
    }
    t = match lang {
        // To do const
        Language::English => "Minors aspects".to_string(),
        Language::French => "Aspects mineurs".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_min_aspects(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_min_aspects,
    });

    // Single Min aspects
    for a in Aspects::iter() {
        if !a.maj() {
            let mut va: Vec<Aspects> = Vec::new();
            va.push(a as Aspects);
            res.push(DataObjectAspectSvg {
                svg: aspects_draw(a, Theme::Light, lang).to_string(),
                text: a.text(lang),
                aspects: va.clone(),
            });
            va.clear()
        }
    }

    // All aspects
    let mut va_all_aspects: Vec<Aspects> = Vec::new();
    for a in Aspects::iter() {
        va_all_aspects.push(a.clone());
    }
    t = match lang {
        // To do const
        Language::English => "All aspects".to_string(),
        Language::French => "Tous les aspects".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_all_aspects(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_all_aspects,
    });

    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

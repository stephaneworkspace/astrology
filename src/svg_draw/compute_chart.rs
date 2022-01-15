extern crate libswe_sys;
use super::aspects::{
    aspects_all_aspects, aspects_draw, aspects_maj_aspects,
    aspects_min_aspects, aspects_no_aspect,
};
use super::svg_draw::{
    CalcDraw, Draw, WorkingStorageDrawPolyMorphTransit,
    WorkingStoragePolyMorphTransit,
};
use crate::svg_draw::svg_draw::{
    WorkingStorageDrawPolyMorphNatal, WorkingStoragePolyMorphNatal,
};
use base64::encode;
use libswe_sys::sweconst::{
    Angle, Aspects, AspectsFilter, Bodies, Calandar, Language, Object,
    ObjectType, OptionalFlag, Signs, Theme,
};
use libswe_sys::swerust;
use libswe_sys::swerust::handler_swe08::{UtcTimeZoneResult, UtcToJdResult};
use libswe_sys::swerust::handler_swe14::HousesResult;
use serde::{Deserialize, Serialize};
use std::f32;
use strum::AsStaticRef;
use strum::IntoEnumIterator;
use svg::node::element::path::Number;

/// Data chart
#[derive(Debug, Deserialize)]
pub struct DataChartNatal {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub min: u32,
    pub sec: f32,
    pub lat: f32,
    pub lng: f32,
    pub time_zone: f32,
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

pub struct Chart {
    pub theme: Theme,
    pub svg_max_size: Number,
    pub natal_data: DataChartNatal,
    pub transit_data: Option<DataChartNatal>,
    pub path: String,
    pub lang: Language,
    pub data_object_svg: Vec<DataObjectSvg>,
    pub vec_aspects: Vec<String>,
}

pub struct SweChart {
    pub utc_time_zone: UtcTimeZoneResult,
    pub utc_to_jd: UtcToJdResult,
    pub utc_time_zone_transit: Option<UtcTimeZoneResult>,
    pub utc_to_jd_transit: Option<UtcToJdResult>,
    pub house_result: HousesResult,
}

impl Chart {
    /// Constructor natal chart
    pub fn new_natal(
        svg_max_size: Number,
        natal_data: DataChartNatal,
        path: String,
    ) -> Self {
        Self {
            theme: Theme::Light,
            svg_max_size,
            natal_data,
            transit_data: None,
            path,
            lang: Language::English,
            data_object_svg: vec![],
            vec_aspects: vec![],
        }
    }

    /// Constructor transit chart
    pub fn new_transit(
        svg_max_size: Number,
        natal_data: DataChartNatal,
        transit_data: DataChartNatal,
        path: String,
    ) -> Self {
        Self {
            theme: Theme::Light,
            svg_max_size,
            natal_data,
            transit_data: Some(transit_data),
            path,
            lang: Language::English,
            data_object_svg: vec![],
            vec_aspects: vec![],
        }
    }

    pub fn swe(&mut self) -> SweChart {
        // Natal chart
        println!("Version swephem: {}", swerust::handler_swe02::version());
        swerust::handler_swe02::set_ephe_path(&self.path);
        println!("{}", self.natal_data.year);
        // House natal chart
        println!("Hsys: {}", swerust::handler_swe14::house_name('P')); // Placidus
        let utc_time_zone: swerust::handler_swe08::UtcTimeZoneResult =
            swerust::handler_swe08::utc_time_zone(
                self.natal_data.year,
                self.natal_data.month as i32,
                self.natal_data.day as i32,
                self.natal_data.hour as i32,
                self.natal_data.min as i32,
                self.natal_data.sec.into(),
                self.natal_data.time_zone.into(),
            );
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
        let utc_time_zone_transit: Option<
            swerust::handler_swe08::UtcTimeZoneResult,
        > = match &self.transit_data {
            Some(data_transit) => Some(swerust::handler_swe08::utc_time_zone(
                data_transit.year,
                data_transit.month as i32,
                data_transit.day as i32,
                data_transit.hour as i32,
                data_transit.min as i32,
                data_transit.sec.into(),
                self.natal_data.time_zone.into(),
            )),
            None => None,
        };
        let utc_to_jd_transit: Option<swerust::handler_swe08::UtcToJdResult> =
            match &utc_time_zone_transit {
                Some(x) => Some(swerust::handler_swe08::utc_to_jd(
                    x.year[0],
                    x.month[0],
                    x.day[0],
                    x.hour[0],
                    x.min[0],
                    x.sec[0],
                    Calandar::Gregorian,
                )),
                None => None,
            };
        let house_result = swerust::handler_swe14::houses(
            utc_to_jd.julian_day_ut,
            self.natal_data.lat as f64,
            self.natal_data.lng as f64,
            'P', // Placidus
        );
        SweChart {
            utc_time_zone,
            utc_to_jd,
            utc_time_zone_transit,
            utc_to_jd_transit,
            house_result,
        }
    }

    /// Draw svg from self.data_objet_svg
    /// with AspectsFilter
    fn parse_svg(&self, aspects: AspectsFilter) -> String {
        let mut svg_res: String = "".to_string();
        for d in self.data_object_svg.clone() {
            if d.object_type == DataObjectType::Chart {
                svg_res = d.svg;
            }
        }
        if svg_res != "" {
            svg_res = svg_res.replace("</svg>", "");
            for d in self.data_object_svg.clone() {
                if d.object_type != DataObjectType::Chart {
                    if d.object_type == DataObjectType::Aspect {
                        let mut sw_res = false;
                        for a in d.aspects {
                            for aa in &aspects.vec_aspects() {
                                if a == *aa {
                                    sw_res = true;
                                }
                            }
                        }
                        if sw_res {
                            svg_res = format!("{}<image width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" href=\"data:image/svg+xml;base64,{}\"/>", svg_res, d.size_x, d.size_y, d.pos_x, d.pos_y, encode(d.svg.as_str()));
                        }
                    } else {
                        svg_res = format!("{}<image width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" href=\"data:image/svg+xml;base64,{}\"/>", svg_res, d.size_x, d.size_y, d.pos_x, d.pos_y, encode(d.svg.as_str()));
                    }
                }
            }
        } else {
            svg_res = "<svg>".to_string();
        }
        svg_res = format!("{}</svg>", svg_res);
        svg_res
    }

    /// Create svg Vector natal
    fn get_vec_data_object_svg_natal(&mut self) -> Self {
        let swe = self.swe();
        let mut object: Vec<Object> = Vec::new();
        let mut calc: swerust::handler_swe03::CalcUtResult;

        for bodie in Bodies::iter() {
            if bodie.clone().object_type() == ObjectType::PlanetOrStar
                || bodie.clone().object_type() == ObjectType::Fiction
            {
                // Natal
                calc = if bodie.clone() == Bodies::FortunaPart {
                    swerust::handler_swe03::calc_ut_fp(
                        swe.utc_to_jd.julian_day_ut,
                        self.natal_data.lat as f64,
                        self.natal_data.lng as f64,
                        'P',
                        OptionalFlag::Speed as i32,
                    )
                } else {
                    swerust::handler_swe03::calc_ut(
                        swe.utc_to_jd.julian_day_ut,
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

        // Natal
        // Object calc draw for calcul in svg x,y width, height
        let mut ws = WorkingStoragePolyMorphNatal::new(
            self.svg_max_size,
            self.theme,
            self.lang,
            swe.house_result,
            object,
        );
        ws.set_fix_compute(true);
        ws.set_fix_compute(false);
        let ws_draw = WorkingStorageDrawPolyMorphNatal::new(ws.clone());

        let mut res: Vec<DataObjectSvg> = Vec::new();

        let aspects_null: Vec<Aspects> = Vec::new();
        // Chart
        res.push(DataObjectSvg {
            svg: ws_draw.draw_base().to_string(),
            object_type: DataObjectType::Chart,
            size_x: self.svg_max_size as f32,
            size_y: self.svg_max_size as f32,
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

        // Houses
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
                // Regular
                for b in ws.object.clone() {
                    let mut sw = false;
                    for p in pair.clone() {
                        if (p.0 == bodie.object_enum && p.1 == b.object_enum)
                            || (p.0 == b.object_enum
                                && p.1 == bodie.object_enum)
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
                            if (abs_separation - asp as f32).abs() <= orb as f32
                            {
                                asp_vec.push(record_asp.clone());
                                let draw = ws_draw.draw_aspect(
                                    ws.get_bodie_longitude(
                                        bodie.object_enum,
                                        false, // no transit
                                    ),
                                    ws.get_bodie_longitude(
                                        b.object_enum,
                                        false,
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
                                self.vec_aspects.push(format!(
                                    "{:?} {:?} {:?} {:?}",
                                    asp_vec.clone(),
                                    bodie.object_enum,
                                    b.object_enum,
                                    record_asp.clone()
                                ));
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
                        // TODO
                        // important change
                        /*
                        .filter(|x| x == &Aspects::Trine || x ==
                            &Aspects::Square || x == &Aspects::Sextile || x == &Aspects::Opposition ||
                            x == &Aspects::Conjunction)
                         */

                        for record_asp in Aspects::iter() {
                            asp = record_asp.angle().0;
                            orb = record_asp.angle().1;
                            if (abs_separation - asp as f32).abs() <= orb as f32
                            {
                                asp_vec.push(record_asp.clone());
                                /*
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
                                // TODO unactivated now anngle
                                res.push(DataObjectSvg {
                                    svg: draw.svg,
                                    object_type: DataObjectType::Aspect,
                                    size_x: draw.size_x as f32,
                                    size_y: draw.size_y as f32,
                                    pos_x: draw.pos_x as f32,
                                    pos_y: draw.pos_y as f32,
                                    aspects: asp_vec.clone(),
                                });
                                 */
                                asp_vec.clear();
                            }
                        }
                    }
                }
            }
        }
        Self {
            theme: self.theme,
            svg_max_size: self.svg_max_size,
            natal_data: DataChartNatal {
                year: self.natal_data.year,
                month: self.natal_data.month,
                day: self.natal_data.day,
                hour: self.natal_data.hour,
                min: self.natal_data.min,
                sec: self.natal_data.sec,
                lat: self.natal_data.lat,
                lng: self.natal_data.lng,
                time_zone: self.natal_data.time_zone,
            },
            transit_data: None,
            path: self.path.clone(),
            lang: self.lang,
            data_object_svg: res,
            vec_aspects: self.vec_aspects.iter().map(|x| x.clone()).collect(),
        }
    }
    /// Create svg Vector transit
    fn get_vec_data_object_svg_transit(&mut self) -> Self {
        let swe = self.swe();
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
                        swe.utc_to_jd.julian_day_ut,
                        self.natal_data.lat as f64,
                        self.natal_data.lng as f64,
                        'P',
                        OptionalFlag::Speed as i32,
                    )
                } else {
                    swerust::handler_swe03::calc_ut(
                        swe.utc_to_jd.julian_day_ut,
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
                match &swe.utc_to_jd_transit {
                    Some(x) => {
                        match &self.transit_data {
                            Some(y) => {
                                calc = if bodie.clone() == Bodies::FortunaPart {
                                    swerust::handler_swe03::calc_ut_fp(
                                        x.julian_day_ut,
                                        y.lat as f64,
                                        y.lng as f64,
                                        'P',
                                        OptionalFlag::Speed as i32,
                                    )
                                } else {
                                    swerust::handler_swe03::calc_ut(
                                        x.julian_day_ut, // debug julianday
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
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }

        // Object calc draw for calcul in svg x,y width, height
        let mut ws = WorkingStoragePolyMorphTransit::new(
            self.svg_max_size,
            self.theme,
            self.lang,
            swe.house_result,
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
            size_x: self.svg_max_size as f32,
            size_y: self.svg_max_size as f32,
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

        // Houses
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
                });
                res.push(DataObjectSvg {
                    svg: draw.trait_svg,
                    object_type: DataObjectType::AngleTrait,
                    size_x: draw.trait_size_x as f32,
                    size_y: draw.trait_size_y as f32,
                    pos_x: draw.trait_pos_x as f32,
                    pos_y: draw.trait_pos_y as f32,
                    aspects: aspects_null.clone(),
                });*/
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
                });
                */
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
                match self.transit_data {
                    Some(_) => {
                        for bt in ws.object_transit.clone() {
                            if ws.get_bodie_is_on_chart(bt.object_enum) {
                                separation = ws.get_closest_distance(
                                    ws.get_bodie_longitude(
                                        bodie.object_enum,
                                        false,
                                    ), // no transit
                                    ws.get_bodie_longitude(
                                        bt.object_enum,
                                        true,
                                    ), // transit
                                );
                                abs_separation = separation.abs();
                                for record_asp in Aspects::iter() {
                                    asp = record_asp.angle().0;
                                    orb = record_asp.angle().1;
                                    if (abs_separation - asp as f32).abs()
                                        <= orb as f32
                                    {
                                        asp_vec.push(record_asp.clone());
                                        let draw = ws_draw.draw_aspect(
                                            ws.get_bodie_longitude(
                                                bodie.object_enum,
                                                false, // no transit
                                            ),
                                            ws.get_bodie_longitude(
                                                bt.object_enum,
                                                true,
                                            ), // transit
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
                                        self.vec_aspects.push(format!(
                                            "{:?} N-{:?} T-{:?} {:?}",
                                            asp_vec.clone(),
                                            bodie.object_enum,
                                            bt.object_enum,
                                            record_asp.clone()
                                        ));
                                        asp_vec.clear();
                                    }
                                }
                            }
                        }
                    }
                    None => {}
                }
                // Regular
                for b in ws.object_natal.clone() {
                    let mut sw = false;
                    for p in pair.clone() {
                        if (p.0 == bodie.object_enum && p.1 == b.object_enum)
                            || (p.0 == b.object_enum
                                && p.1 == bodie.object_enum)
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
                            if (abs_separation - asp as f32).abs() <= orb as f32
                            {
                                asp_vec.push(record_asp.clone());
                                let draw = ws_draw.draw_aspect(
                                    ws.get_bodie_longitude(
                                        bodie.object_enum,
                                        false, // no transit
                                    ),
                                    ws.get_bodie_longitude(
                                        b.object_enum,
                                        false,
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
                                self.vec_aspects.push(format!(
                                    "{:?} N-{:?} N-{:?} {:?}",
                                    asp_vec.clone(),
                                    bodie.object_enum,
                                    b.object_enum,
                                    record_asp.clone()
                                ));
                                asp_vec.clear();
                            }
                        }
                    }
                }
                match self.transit_data {
                    Some(_) => {
                        // Transit
                        for i in 0..12 {
                            if i == 0 || i == 9 {
                                // Only Asc et Mc
                                separation = ws.get_closest_distance(
                                    bodie.longitude as f32,
                                    ws.house.clone()[i].longitude as f32,
                                );
                                abs_separation = separation.abs();
                                // TODO
                                /*
                                .filter(|x| x == &Aspects::Trine || x ==
                                      &Aspects::Square || x == &Aspects::Sextile || x == &Aspects::Opposition ||
                                      x == &Aspects::Conjunction)
                                   */

                                for record_asp in Aspects::iter() {
                                    asp = record_asp.angle().0;
                                    orb = record_asp.angle().1;
                                    if (abs_separation - asp as f32).abs()
                                        <= orb as f32
                                    {
                                        asp_vec.push(record_asp.clone());
                                        /*
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
                                        // Unactivated now TODO option
                                        res.push(DataObjectSvg {
                                            svg: draw.svg,
                                            object_type: DataObjectType::Aspect,
                                            size_x: draw.size_x as f32,
                                            size_y: draw.size_y as f32,
                                            pos_x: draw.pos_x as f32,
                                            pos_y: draw.pos_y as f32,
                                            aspects: asp_vec.clone(),
                                        });*/
                                        asp_vec.clear();
                                    }
                                }
                            }
                        }
                    }
                    None => {}
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
                        // TODO
                        // important change
                        /*
                        .filter(|x| x == &Aspects::Trine || x ==
                            &Aspects::Square || x == &Aspects::Sextile || x == &Aspects::Opposition ||
                            x == &Aspects::Conjunction)
                         */

                        for record_asp in Aspects::iter() {
                            asp = record_asp.angle().0;
                            orb = record_asp.angle().1;
                            if (abs_separation - asp as f32).abs() <= orb as f32
                            {
                                asp_vec.push(record_asp.clone());
                                /*
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
                                // TODO unactivated now anngle
                                res.push(DataObjectSvg {
                                    svg: draw.svg,
                                    object_type: DataObjectType::Aspect,
                                    size_x: draw.size_x as f32,
                                    size_y: draw.size_y as f32,
                                    pos_x: draw.pos_x as f32,
                                    pos_y: draw.pos_y as f32,
                                    aspects: asp_vec.clone(),
                                });
                                 */
                                asp_vec.clear();
                            }
                        }
                    }
                }
            }
        }
        let t = match &self.transit_data {
            Some(x) => x.clone(),
            None => &DataChartNatal {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                min: 0,
                sec: 0.0,
                lat: 0.0,
                lng: 0.0,
                time_zone: 0.0,
            },
        };
        let transit_data = DataChartNatal {
            year: t.year,
            month: t.month,
            day: t.day,
            hour: t.hour,
            min: t.min,
            sec: t.sec,
            lat: t.lat,
            lng: t.lng,
            time_zone: t.time_zone,
        };
        Chart {
            theme: self.theme,
            svg_max_size: self.svg_max_size,
            natal_data: DataChartNatal {
                year: self.natal_data.year,
                month: self.natal_data.month,
                day: self.natal_data.day,
                hour: self.natal_data.hour,
                min: self.natal_data.min,
                sec: self.natal_data.sec,
                lat: self.natal_data.lat,
                lng: self.natal_data.lng,
                time_zone: self.natal_data.time_zone,
            },
            transit_data: Some(transit_data),
            path: self.path.clone(),
            lang: self.lang,
            data_object_svg: res,
            vec_aspects: self.vec_aspects.iter().map(|x| x.clone()).collect(),
        }
    }
}

/// Svg natal chart
pub fn chart_svg(
    max_size: Number,
    data: DataChartNatal,
    path: &str,
    _lang: Language,
    aspects: AspectsFilter,
) -> (String, Vec<String>) {
    let mut chart = Chart::new_natal(
        max_size,
        DataChartNatal {
            year: data.year,
            month: data.month,
            day: data.day,
            hour: data.hour,
            min: data.min,
            sec: data.sec,
            lat: data.lat,
            lng: data.lng,
            time_zone: data.time_zone,
        },
        path.to_string(),
    );
    // TODO remove mut
    let chart = chart.get_vec_data_object_svg_natal();
    let res_svg = chart.parse_svg(aspects);
    (res_svg, chart.vec_aspects)
}

/// Svg transit chart
pub fn chart_svg_with_transit(
    max_size: Number,
    n: DataChartNatal,
    t: DataChartNatal,
    path: &str,
    _lang: Language,
    aspects: AspectsFilter,
) -> (String, Vec<String>) {
    let mut chart = Chart::new_transit(
        max_size,
        DataChartNatal {
            year: n.year,
            month: n.month,
            day: n.day,
            hour: n.hour,
            min: n.min,
            sec: n.sec,
            lat: n.lat,
            lng: n.lng,
            time_zone: n.time_zone,
        },
        DataChartNatal {
            year: t.year,
            month: t.month,
            day: t.day,
            hour: t.hour,
            min: t.min,
            sec: t.sec,
            lat: t.lat,
            lng: t.lng,
            time_zone: t.time_zone,
        },
        path.to_string(),
    );
    // TODO remove mut
    let chart = chart.get_vec_data_object_svg_transit();
    let res_svg = chart.parse_svg(aspects);
    (res_svg, chart.vec_aspects)
}

/// DataObjectAspectSvg of Aspects with svg + text
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

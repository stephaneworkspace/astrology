/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane Bressani (s.bressani@bluewin.ch)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an MIT License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to use astro_compute_swisseph in your commercial
 * projects, you must adhere to the GPL license or buy a Swiss Ephemeris
 * commercial license.
 */
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;

use strum::{AsStaticRef, IntoEnumIterator};
extern crate astrology_draw_svg;
extern crate ferris_says;
use libswe_sys::sweconst::{
    Bodies, Calandar, House, Object, ObjectType, OptionalFlag,
};
use libswe_sys::swerust::{
    handler_swe02,
    handler_swe03,
    handler_swe07,
    handler_swe08,
    handler_swe14,
    //    handler_swe14::HousesResult,
};
use serde::Deserialize;
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::io::{stdout, BufWriter};
use std::os::raw::{c_char, c_double};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Data {
    year: i32,
    month: i32,
    day: i32,
    hourf64: f64,
    hour: i32,
    min: i32,
    sec: f64,
    lat: f64,
    lng: f64,
}

/// Simple write in console
#[no_mangle]
pub extern "C" fn intro() {
    let phrase = b"Welcome to astro_compute_swisseph";
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(phrase, 40, &mut writer).unwrap();
}

/// Return version of api
#[no_mangle]
pub extern "C" fn sweversion() -> *const c_char {
    //CString::new(get_version()).unwrap().into_raw()
    CString::new(handler_swe02::version()).unwrap().into_raw()
}

// For yew front end
#[no_mangle]
pub extern "C" fn simple_svg(max_size: c_double) -> *const c_char {
    CString::new(astrology_draw_svg::chart(max_size as f64, ""))
        .unwrap()
        .into_raw()
}

pub fn intern_svg(max_size: f64, path: &str) {
    let ephe_path: &str = "/Users/stephanebressani/Code/Rust/astro_compute_swisseph/lib/libswe-sys/src/swisseph/sweph/";
    handler_swe02::set_ephe_path(&ephe_path);

    const PATH: &str = "examples/data.json";
    let mut s = String::new();
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push(PATH);
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let data: Data = serde_json::from_str(&s).unwrap();
    println!("Data: {:?}", data);
    let julday: f64 = handler_swe08::julday(
        data.year,
        data.month,
        data.day,
        data.hourf64,
        Calandar::Gregorian,
    );
    println!("Get julday: {:?}", julday);

    let mut object: Vec<Object> = Vec::new();
    let mut calc: handler_swe03::CalcUtResult;
    for bodies in Bodies::iter() {
        if bodies.clone().object_type() == ObjectType::PlanetOrStar {
            calc = handler_swe03::calc_ut(
                julday,
                bodies.clone(),
                OptionalFlag::Speed as i32,
            );
            object.push(Object::new(
                bodies.clone().as_static(),
                bodies.clone().object_type(),
                calc.longitude,
                calc.latitude,
            ));
        }
    }

    for o in object {
        println!("{:?}", o);
    }

    let pheno_ut: handler_swe07::PhenoUtResult = handler_swe07::pheno_ut(
        julday,
        Bodies::Sun,
        OptionalFlag::Speed as i32,
    );
    println!("PhenoUt: {:?}", pheno_ut);

    // let hsys = HouseSystem::Placidus;
    let name = handler_swe14::house_name('P');
    println!("Hsys: {}", name);

    let utc_time_zone: handler_swe08::UtcTimeZoneResult =
        handler_swe08::utc_time_zone(
            data.year, data.month, data.day, data.hour, data.min, data.sec, 2.0,
        );
    println!("utc_time_zone: {:?}", utc_time_zone);

    let utc_to_jd: handler_swe08::UtcToJdResult = handler_swe08::utc_to_jd(
        utc_time_zone.year[0],
        utc_time_zone.month[0],
        utc_time_zone.day[0],
        utc_time_zone.hour[0],
        utc_time_zone.min[0],
        utc_time_zone.sec[0],
        /*utc_time_zone.year[1],
        utc_time_zone.month[1],
        utc_time_zone.day[1],
        utc_time_zone.hour[1],
        utc_time_zone.min[1],
        utc_time_zone.sec[1],*/
        Calandar::Gregorian,
    );
    println!("utc_to_jd: {:?}", utc_to_jd);

    // To do struct for frontend (for draw canvas/svg)
    let result =
        handler_swe14::houses(utc_to_jd.julian_day_ut, data.lat, data.lng, 'P');
    //println!("House object: {:?}", result);
    let mut house: Vec<House> = Vec::new();
    for (i, res) in result.clone().cusps.iter().enumerate() {
        if i > 0 {
            house.push(House::new(i as i32, res.clone()));
            if i + 1 > 12 {
                break;
            }
        }
    }

    for h in house {
        println!("{:?}", h);
    }
    println!("House: {:?}", result.clone());

    astrology_draw_svg::chart(max_size, path);
    handler_swe02::close();
}

/// Unit test
#[cfg(test)]
mod tests {
    use libswe_sys::swerust::handler_swe02;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn version() {
        assert_eq!(handler_swe02::version(), "2.08");
    }
}

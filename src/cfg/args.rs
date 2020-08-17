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
use super::parse::{
    parse_date, parse_date_from_str, parse_time, parse_time_from_str,
};
use super::validator::{
    validator_parse_date, validator_parse_latlng, validator_parse_path,
    validator_parse_size, validator_parse_time,
};
use chrono::{Datelike, NaiveDate, NaiveTime, Timelike, Utc};
use clap::{App, Arg};
use std::env;
use std::format;
use std::str::FromStr;

#[derive(Debug)]
pub struct AstrologyConfig {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub lat: f32,
    pub lng: f32,
    pub path_and_file: String,
    pub path_ephem_files: String,
    pub size: u32,
}

const DATE: &str = "date";
const TIME: &str = "time";
const LAT: &str = "lat";
const LNG: &str = "lng";
const PATH: &str = "path_and_file";
const PATH_EPHEM: &str = "path_ephem";
const SIZE: &str = "size";

/// Parse args for clap
pub fn parse_args() -> AstrologyConfig {
    let now = Utc::now();
    let date = parse_date(now.day(), now.month(), now.year()).unwrap();
    let default_value_date = format!(
        "{}.{}.{}",
        date.day().to_string(),
        date.month().to_string(),
        date.year().to_string()
    )
    .to_string();
    //let time = parse_time(now.hour(), now.minute(), now.second()).unwrap();
    let time = parse_time(0, 0, 0).unwrap();
    let default_value_time =
        format!("{}:{}", time.hour().to_string(), time.minute().to_string(),)
            .to_string();
    let default_value_path = format!(
        "{}/natal_chart.svg",
        env::current_dir().unwrap().as_path().display()
    );
    let default_value_square = "1000";
    let matches = App::new("Astrology")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Stéphane Bressani <stephane@astrologie-traditionnelle.net)")
        .about("Create svg natal chart using swissephem c library by Astrodienst AG by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an CC License, which allows it to be used
also on commercial projects. This software uses the swiss ephemeris which is
licensed GPL.

Therefore, if you want to use astro_compute_swisseph in your commercial
projects, you must adhere to the GPL license or buy a Swiss Ephemeris
commercial license.")
        .arg(
            Arg::with_name(DATE)
                .short("d")
                .value_name("DATE_CHART")
                .default_value(&default_value_date)
                .multiple(false)
                .help("Date of birth in format: dd.mm.yyyy")
                .validator(validator_parse_date)
                .required(true),
        )
        .arg(
            Arg::with_name(TIME)
                .short("t")
                .value_name("TIME_CHART")
                .default_value(&default_value_time)
                .multiple(false)
                .help("Time of birth in format: hh:mm:ss or hh:mm")
                .validator(validator_parse_time)
                .required(true),
        )
        .arg(
            Arg::with_name(LAT)
                .value_name("LAT_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_latlng)
                .help("Latitude of birth in float format: 99.99"),
        )
        .arg(
            Arg::with_name(LNG)
                .value_name("LNG_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_latlng)
                .help("Longitude of birth in float format: 99.99"),
        )
        .arg(
            Arg::with_name(PATH)
                .long("path_export")
                .value_name("PATH_AND_FILE_CHART")
                .default_value(&default_value_path)
                .help("Path for svg draw on the disk")
                .multiple(false)
                .required(true),
        )
        .arg(
            Arg::with_name(PATH_EPHEM)
                .long("path_ephem")
                .value_name("PATH_SWISS_EPHEM_FILES")
                .help("Path of swiss ephem files")
                .multiple(false)
                .validator(validator_parse_path)
                .required(true),
        )
        .arg(
            Arg::with_name(SIZE)
                .short("s")
                .value_name("SIZE_SQUARE_IN_PX")
                .default_value(&default_value_square)
                .help("Size of the square")
                .multiple(false)
                .validator(validator_parse_size)
                .required(true),
        )
        .get_matches();
    let date_final = parse_date_from_str(
        matches.value_of(DATE).unwrap_or(&default_value_date),
    )
    .unwrap();
    let time_final =
        parse_time_from_str(matches.value_of(TIME).unwrap()).unwrap();
    let mut ephem_final = matches.values_of(PATH_EPHEM).unwrap();
    let mut size_final = matches.values_of(SIZE).unwrap();
    let size_final_string: String =
        size_final.next().as_deref().unwrap().to_string();
    AstrologyConfig {
        date: date_final,
        time: time_final,
        lat: f32::from_str(matches.value_of(LAT).unwrap()).unwrap(),
        lng: f32::from_str(matches.value_of(LNG).unwrap()).unwrap(),
        path_and_file: matches
            .value_of(PATH)
            .unwrap_or(&default_value_path)
            .to_string(),
        path_ephem_files: ephem_final.next().as_deref().unwrap().to_string(),
        size: size_final_string.parse::<u32>().unwrap(),
    }
}

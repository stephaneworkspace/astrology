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
    validator_parse_aspect, validator_parse_date, validator_parse_latlng,
    validator_parse_path, validator_parse_size, validator_parse_time,
    validator_parse_timezone,
};
use chrono::{Datelike, NaiveDate, NaiveTime, Timelike, Utc};
use clap::{App, AppSettings, Arg};
use std::env;
use std::format;
use std::str::FromStr;

#[derive(Debug)]
pub struct AstrologyConfig {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub lat: f32,
    pub lng: f32,
    pub time_zone: f32,
    pub path_and_file: String,
    pub path_ephem_files: String,
    pub size: u32,
    pub aspect: u32,
}

#[derive(Debug)]
pub struct AstrologyTransitConfig {
    pub date_n: NaiveDate,
    pub time_n: NaiveTime,
    pub lat_n: f32,
    pub lng_n: f32,
    pub time_zone_n: f32,
    pub date_t: NaiveDate,
    pub time_t: NaiveTime,
    pub lat_t: f32,
    pub lng_t: f32,
    pub time_zone_t: f32,
    pub path_and_file: String,
    pub path_ephem_files: String,
    pub size: u32,
    pub aspect: u32,
}

const AUTHOR: &str =
    "Stéphane Bressani <stephane@astrologie-traditionnelle.net)";

const DATE: &str = "date";
const TIME: &str = "time";
const LAT: &str = "lat";
const LNG: &str = "lng";
const TIME_ZONE: &str = "time_zone";
const ASPECT: &str = "aspect";
const PATH: &str = "path_and_file";
const PATH_EPHEM: &str = "path_ephem";
const SIZE: &str = "size";

const DATE_N: &str = "date_natal";
const TIME_N: &str = "time_natal";
const LAT_N: &str = "lat_natal";
const LNG_N: &str = "lng_natal";
const TIME_ZONE_N: &str = "time_zone_natal";
const DATE_T: &str = "date_transit";
const TIME_T: &str = "time_transt";
const LAT_T: &str = "lat_transit";
const LNG_T: &str = "lng_transit";
const TIME_ZONE_T: &str = "time_zone_transit";

/// Parse args chart natal (exemple -> svg) for clap
pub fn parse_args_natal() -> AstrologyConfig {
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
        .author(AUTHOR)
        .about("Create svg natal chart using swissephem c library by Astrodienst AG by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an CC License, which allows it to be used
also on commercial projects. This software uses the swiss ephemeris which is
licensed GPL.

Therefore, if you want to use astro_compute_swisseph in your commercial
projects, you must adhere to the GPL license or buy a Swiss Ephemeris
commercial license.")
        .setting(AppSettings::AllowLeadingHyphen)
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
            Arg::with_name(TIME_ZONE)
                .value_name("TIME_ZONE_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_timezone)
                .help("Time zone of birth in numeric format"),
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
        .arg(
            Arg::with_name(ASPECT)
                .short("a")
                .value_name("ASPECT_CODE")
                .default_value("0")
                .help("Code of aspect :
    All aspects = 0
    All majors aspects = 1
    Conjunction = 2
    Opposition = 3
    Trine = 4
    Square = 5
    Sextile = 6
    All minors aspect = 7
    Inconjunction = 8
    Sesquisquare = 9
    Semisquare = 10
    Semisextile = 11
    No aspects = 12")
                .multiple(false)
                .validator(validator_parse_aspect)
                .required(false),
        )
        .get_matches();
    let date_final =
        parse_date_from_str(matches.value_of(DATE).unwrap()).unwrap();
    let time_final =
        parse_time_from_str(matches.value_of(TIME).unwrap()).unwrap();
    let mut ephem_final = matches.values_of(PATH_EPHEM).unwrap();
    let mut size_final = matches.values_of(SIZE).unwrap();
    let size_final_string: String =
        size_final.next().as_deref().unwrap().to_string();
    let mut aspect_final = matches.values_of(ASPECT).unwrap();
    let aspect_final_string: String =
        aspect_final.next().as_deref().unwrap().to_string();
    AstrologyConfig {
        date: date_final,
        time: time_final,
        lat: f32::from_str(matches.value_of(LAT).unwrap()).unwrap(),
        lng: f32::from_str(matches.value_of(LNG).unwrap()).unwrap(),
        time_zone: f32::from_str(matches.value_of(TIME_ZONE).unwrap()).unwrap(),
        path_and_file: matches
            .value_of(PATH)
            .unwrap_or(&default_value_path)
            .to_string(),
        path_ephem_files: ephem_final.next().as_deref().unwrap().to_string(),
        size: size_final_string.parse::<u32>().unwrap(),
        aspect: aspect_final_string.parse::<u32>().unwrap(),
    }
}

/// Parse args chart transit (exemple -> svg_transit) for clap
pub fn parse_args_transit() -> AstrologyTransitConfig {
    let default_value_path = format!(
        "{}/transit_chart.svg",
        env::current_dir().unwrap().as_path().display()
    );
    let default_value_square = "1000";
    let matches = App::new("Astrology")
        .version(env!("CARGO_PKG_VERSION"))
        .author(AUTHOR)
        .about("Create svg natal + transit chart using swissephem c library by Astrodienst AG by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an CC License, which allows it to be used
also on commercial projects. This software uses the swiss ephemeris which is
licensed GPL.

Therefore, if you want to use astro_compute_swisseph in your commercial
projects, you must adhere to the GPL license or buy a Swiss Ephemeris
commercial license.")
        .arg(
            Arg::with_name(DATE_N)
                .long("natal_date")
                .value_name("DATE_NATAL_CHART")
                .multiple(false)
                .help("Date of birth in format: dd.mm.yyyy")
                .validator(validator_parse_date)
                .required(true),
        )
        .arg(
            Arg::with_name(DATE_T)
                .long("transit_date")
                .value_name("DATE_NATAL_CHART")
                .multiple(false)
                .help("Date of transit in format: dd.mm.yyyy")
                .validator(validator_parse_date)
                .required(true),
        )
        .arg(
            Arg::with_name(TIME_N)
                .long("natal_time")
                .value_name("TIME_NATAL_CHART")
                .multiple(false)
                .help("Time of birth in format: hh:mm:ss or hh:mm")
                .validator(validator_parse_time)
                .required(true),
        )
        .arg(
            Arg::with_name(TIME_T)
                .long("transit_time")
                .value_name("TIME_TRANSIT_CHART")
                .multiple(false)
                .help("Time of transit in format: hh:mm:ss or hh:mm")
                .validator(validator_parse_time)
                .required(true),
        )
        .arg(
            Arg::with_name(LAT_N)
                .long("natal_lat")
                .value_name("LAT_NATAL_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_latlng)
                .help("Latitude of birth in float format: 99.99"),
        )
        .arg(
            Arg::with_name(LAT_T)
                .long("transit_lat")
                .value_name("LAT_TRANSIT_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_latlng)
                .help("Latitude of transit in float format: 99.99"),
        )
        .arg(
            Arg::with_name(LNG_N)
                .long("natal_lng")
                .value_name("LNG_NATAL_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_latlng)
                .help("Longitude of birth in float format: 99.99"),
        )
        .arg(
            Arg::with_name(LNG_T)
                .long("transit_lng")
                .value_name("LNG_TRANSIT_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_latlng)
                .help("Longitude of transit in float format: 99.99"),
        )
        .arg(
            Arg::with_name(TIME_ZONE_N)
                .long("natal_time_zone")
                .value_name("TIME_ZONE_NATAL_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_timezone)
                .help("Time zone of birth in numeric format"),
        )
        .arg(
            Arg::with_name(TIME_ZONE_T)
                .long("transit_time_zone")
                .value_name("TIME_ZONE_TRANSIT_CHART")
                .required(true)
                .multiple(false)
                .validator(validator_parse_timezone)
                .help("Time zone of transit in numeric format"),
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
        .arg(
            Arg::with_name(ASPECT)
                .short("a")
                .value_name("ASPECT_CODE")
                .default_value("0")
                .help("Code of aspect :
    All aspects = 0
    All majors aspects = 1
    Conjunction = 2
    Opposition = 3
    Trine = 4
    Square = 5
    Sextile = 6
    All minors aspect = 7
    Inconjunction = 8
    Sesquisquare = 9
    Semisquare = 10
    Semisextile = 11
    No aspects = 12")
                .multiple(false)
                .validator(validator_parse_aspect)
                .required(false),
        )
        .get_matches();
    let date_n_final =
        parse_date_from_str(matches.value_of(DATE_N).unwrap()).unwrap();
    let date_t_final =
        parse_date_from_str(matches.value_of(DATE_T).unwrap()).unwrap();
    let time_n_final =
        parse_time_from_str(matches.value_of(TIME_N).unwrap()).unwrap();
    let time_t_final =
        parse_time_from_str(matches.value_of(TIME_T).unwrap()).unwrap();
    let mut ephem_final = matches.values_of(PATH_EPHEM).unwrap();
    let mut size_final = matches.values_of(SIZE).unwrap();
    let size_final_string: String =
        size_final.next().as_deref().unwrap().to_string();
    let mut aspect_final = matches.values_of(ASPECT).unwrap();
    let aspect_final_string: String =
        aspect_final.next().as_deref().unwrap().to_string();
    AstrologyTransitConfig {
        date_n: date_n_final,
        time_n: time_n_final,
        lat_n: f32::from_str(matches.value_of(LAT_N).unwrap()).unwrap(),
        lng_n: f32::from_str(matches.value_of(LNG_N).unwrap()).unwrap(),
        time_zone_n: f32::from_str(matches.value_of(TIME_ZONE_N).unwrap())
            .unwrap(),
        date_t: date_t_final,
        time_t: time_t_final,
        lat_t: f32::from_str(matches.value_of(LAT_T).unwrap()).unwrap(),
        lng_t: f32::from_str(matches.value_of(LNG_T).unwrap()).unwrap(),
        time_zone_t: f32::from_str(matches.value_of(TIME_ZONE_T).unwrap())
            .unwrap(),
        path_and_file: matches
            .value_of(PATH)
            .unwrap_or(&default_value_path)
            .to_string(),
        path_ephem_files: ephem_final.next().as_deref().unwrap().to_string(),
        size: size_final_string.parse::<u32>().unwrap(),
        aspect: aspect_final_string.parse::<u32>().unwrap(),
    }
}

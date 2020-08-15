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
use chrono::format::ParseError;
use chrono::{Datelike, NaiveDate, NaiveTime, Timelike, Utc};
use clap::{App, Arg};
use std::format;
use std::str::FromStr;

#[derive(Debug)]
pub struct AstrologyConfig {
    date: String,
    time: String,
    lat: f32,
    lng: f32,
    path_and_file: String,
}

const DATE: &str = "date";
const TIME: &str = "time";
const LAT: &str = "lat";
const LNG: &str = "lng";
const PATH_AND_FILE: &str = "path_and_file";

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
    let default_value_time = format!(
        "{}:{}:{}",
        time.hour().to_string(),
        time.minute().to_string(),
        time.second().to_string()
    )
    .to_string();
    let matches = App::new("Astrology")
        .version("0.2.1") //TODO: get it from metadata
        .author("Stéphane Bressani <stephane@astrologie-traditionnelle.net)")
        .about("Create svg natal chart using swissephem lib")
        .arg(
            Arg::with_name(DATE)
                // .short("d")
                // .long("date")
                .value_name("DATE_CHART")
                .default_value(&default_value_date)
                .help("Date of birth in format: dd.mm.yyyy")
                //.validator(validate_ports)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name(TIME)
                // .short("t")
                // .long("time")
                .value_name("TIME_CHART")
                .default_value(&default_value_time)
                .required(true)
                .help("Time of birth in format: hh:mm:ss"),
        )
        .arg(
            Arg::with_name(LAT)
                // .long("latitude")
                .value_name("LAT_CHART")
                .required(true)
                .help("Latitude of birth in float format: 99.99"),
        )
        .arg(
            Arg::with_name(LNG)
                // .long("longitude")
                .value_name("LNG_CHART")
                .required(true)
                .help("Longitude of birth in float format: (example) 99.99"),
        )
        .arg(
            Arg::with_name(PATH_AND_FILE)
                .short("p")
                .value_name("PATH_AND_FILE_CHART")
                .required(false)
                .help("Path for svg draw on the disk (default:  ~/natal_chart.svg)"),
        )
        //.validator(validat_ports)
        .get_matches();
    AstrologyConfig {
        date: matches
            .value_of("DATE_CHART")
            .unwrap_or(&default_value_date)
            .to_string(),
        time: matches
            .value_of("TIME_CHART")
            .unwrap_or(&default_value_time)
            .to_string(),
        lat: f32::from_str(matches.value_of("LAT_CHART").unwrap()).unwrap(),
        lng: f32::from_str(matches.value_of("LNG_CHART").unwrap()).unwrap(),
        path_and_file: matches
            .value_of("PATH_AND_FILE_CHART")
            .unwrap_or("~/natal_chart.svg")
            .to_string(),
    }
}

fn parse_date(
    day: u32,
    month: u32,
    year: i32,
) -> Result<NaiveDate, ParseError> {
    let f = format!("{}.{}.{}", day, month, year).to_string();
    let date = NaiveDate::parse_from_str(&f, "%d.%m.%Y")?;
    Ok(date)
}

fn parse_time(hour: i32, min: i32, sec: i32) -> Result<NaiveTime, ParseError> {
    let f = format!("{}:{}:{}", hour, min, sec).to_string();
    let time = NaiveTime::parse_from_str(&f, "%H:%M:%S")?;
    Ok(time)
}

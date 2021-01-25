/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (https://github.com/stephaneworkspace)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * This software uses the swiss ephemeris which is licensed GPL.
 *
 * Therefore, if you want to this source in your commercial projects, you must
 * adhere to the GPL license or buy a Swiss Ephemeris commercial license.
 */
use chrono::format::ParseError;
use chrono::{NaiveDate, NaiveTime};
use std::format;

/// Parse date from value integer to NaiveDate
pub fn parse_date(
    day: u32,
    month: u32,
    year: i32,
) -> Result<NaiveDate, ParseError> {
    let f = format!("{}.{}.{}", day, month, year).to_string();
    let date = NaiveDate::parse_from_str(&f, "%d.%m.%Y")?;
    Ok(date)
}

/// Parse date from value &str to NaiveDate
pub fn parse_date_from_str(date: &str) -> Result<NaiveDate, ParseError> {
    let d = NaiveDate::parse_from_str(date, "%d.%m.%Y")?;
    Ok(d)
}

/// Parse time from value integer to NaiveTime
pub fn parse_time(
    hour: i32,
    min: i32,
    sec: i32,
) -> Result<NaiveTime, ParseError> {
    let f = format!("{}:{}:{}", hour, min, sec).to_string();
    let time = NaiveTime::parse_from_str(&f, "%H:%M:%S")?;
    Ok(time)
}

/// Parse time from value &str to NaiveTime
pub fn parse_time_from_str(time: &str) -> Result<NaiveTime, ParseError> {
    let items: Vec<_> = time.clone().split(&[':'][..]).collect();
    let mut time_string: String = "".to_string();
    for (i, item) in items.iter().enumerate() {
        if i > 2 {
            break;
        }
        if i == 0 {
            time_string = format!("{}", item);
        } else {
            time_string = format!("{}:{}", time_string, item);
        }
    }
    match items.len() {
        1 => {
            time_string = format!("{}:0:0", time_string);
        },
        2 => {
            time_string = format!("{}:0", time_string);
        },
        _ => {},
    }
    let t = NaiveTime::parse_from_str(time_string.as_str(), "%H:%M:%S")?;
    Ok(t)
}

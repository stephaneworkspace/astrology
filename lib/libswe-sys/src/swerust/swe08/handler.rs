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
use crate::raw;
use crate::sweconst::Calandar;
use std::ffi::{CStr, CString};
// use std::os::raw::c_char;

/*
 * 8. Date and time conversion functions
 */
pub fn julday(
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
    calandar: Calandar,
) -> f64 {
    let result: f64 =
        unsafe { raw::swe_julday(year, month, day, hour, calandar as i32) };
    result
}

/// [0 -> jday / 1 -> utc]
#[derive(Debug, Clone)]
pub struct UtcTimeZoneResult {
    pub year: [i32; 2],
    pub month: [i32; 2],
    pub day: [i32; 2],
    pub hour: [i32; 2],
    pub min: [i32; 2],
    pub sec: [f64; 2],
}

pub fn utc_time_zone(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    sec: f64,
    timezone: f64,
) -> UtcTimeZoneResult {
    let mut year_out = [0; 2];
    let mut month_out = [0; 2];
    let mut day_out = [0; 2];
    let mut hour_out = [0; 2];
    let mut min_out = [0; 2];
    let mut sec_out = [0.0; 2];
    unsafe {
        let p_year_out = year_out.as_mut_ptr();
        let p_month_out = month_out.as_mut_ptr();
        let p_day_out = day_out.as_mut_ptr();
        let p_hour_out = hour_out.as_mut_ptr();
        let p_min_out = min_out.as_mut_ptr();
        let p_sec_out = sec_out.as_mut_ptr();
        raw::swe_utc_time_zone(
            year,
            month,
            day,
            hour,
            min,
            sec,
            timezone,
            p_year_out,
            p_month_out,
            p_day_out,
            p_hour_out,
            p_min_out,
            p_sec_out,
        );
        UtcTimeZoneResult {
            year: year_out,
            month: month_out,
            day: day_out,
            hour: hour_out,
            min: min_out,
            sec: sec_out,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UtcToJdResult {
    pub julian_day_et: f64,
    pub julian_day_ut: f64,
    pub err: String, // To do in other file same struct
    pub result: i32,
}
pub fn utc_to_jd(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    sec: f64,
    calandar: Calandar,
) -> UtcToJdResult {
    let mut dret = [0.0; 2];
    let mut serr = [0; 255];
    unsafe {
        let p_dret = dret.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        let result = raw::swe_utc_to_jd(
            year,
            month,
            day,
            hour,
            min,
            sec,
            calandar as i32,
            p_dret,
            p_serr,
        );
        let s_serr = CString::from(CStr::from_ptr(p_serr))
            .to_str()
            .unwrap()
            .to_string();
        UtcToJdResult {
            julian_day_et: dret[0],
            julian_day_ut: dret[1],
            err: s_serr,
            result: result,
        }
    }
}

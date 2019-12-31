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
extern crate math;
use crate::raw;
use crate::sweconst::Signs;
use math::round;
use strum::IntoEnumIterator;
// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;

/*
 * 17. Auxilliary functions
 */
pub fn degnorm(x: f64) -> f64 {
    unsafe { raw::swe_degnorm(x) }
}

pub fn radnorm(x: f64) -> f64 {
    unsafe { raw::swe_radnorm(x) }
}

#[derive(Debug)]
pub struct SplitDegResult {
    print: String,
    deg: i32,
    min: i32,
    sec: i32,
    cdegfr: f64,
    //isgn: i32,
    sign: Signs,
    result: f64,
}

/// float to deg
/// isgn return me always 0 ? I compute this value manualy
pub fn split_deg(ddeg: f64, roundflag: i32) -> SplitDegResult {
    // Convert deg to sign 30°
    let sign_calc = round::floor(ddeg / 30.0, 0) as usize;
    let house_calc = round::floor(ddeg / 30.0, 0);
    let long_30 = (house_calc as f64 * 30.0) - ddeg;
    // Call c library
    let mut deg = [0; 1];
    let mut min = [0; 1];
    let mut sec = [0; 1];
    let mut cdegfr = [0.0; 1];
    let mut isgn = [0; 1];
    let result: f64 = unsafe {
        let p_deg = deg.as_mut_ptr();
        let p_min = min.as_mut_ptr();
        let p_sec = sec.as_mut_ptr();
        let p_cdegfr = cdegfr.as_mut_ptr();
        let p_isgn = isgn.as_mut_ptr();
        raw::swe_split_deg(
            long_30, roundflag, p_deg, p_min, p_sec, p_cdegfr, p_isgn,
        )
    };
    let print = format!(
        "{}{}{:02}{}{:02}",
        i32::abs(deg[0]),
        "°",
        min[0],
        "'",
        sec[0],
    );
    SplitDegResult {
        print: print,
        deg: deg[0],
        min: min[0],
        sec: sec[0],
        cdegfr: cdegfr[0],
        // isgn: isgn[0],
        sign: Signs::iter().nth(sign_calc).unwrap(),
        result: result,
    }
}

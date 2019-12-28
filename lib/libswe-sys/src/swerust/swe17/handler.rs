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
// use crate::sweconst::Calandar;
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
    string_result: String,
    deg: i32,
    min: i32,
    sec: i32,
    cdegfr: f64,
    isgn: i32,
    result: f64,
}

pub fn split_deg(ddeg: f64, roundflag: i32) -> SplitDegResult {
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
            ddeg, roundflag, p_deg, p_min, p_sec, p_cdegfr, p_isgn,
        )
    };
    let string_result = format!(
        "{}{}{:02}{}{:02}",
        i32::abs(deg[0]),
        "°",
        min[0],
        "'",
        sec[0],
    );
    SplitDegResult {
        string_result: string_result,
        deg: deg[0],
        min: min[0],
        sec: sec[0],
        cdegfr: cdegfr[0],
        isgn: isgn[0],
        result: result,
    }
}

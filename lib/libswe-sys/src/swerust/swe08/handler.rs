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
//use std::ffi::{CStr, CString};
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

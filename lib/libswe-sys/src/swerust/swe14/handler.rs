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
// use crate::sweconst::HouseSystem;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

/*
 * 14. House cusp calculation
 */

pub fn house_name(hsys: char) -> String {
    unsafe {
        CString::from(CStr::from_ptr(raw::swe_house_name(hsys as c_int)))
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct HousesResult {
    // cusps: [f64; 37], // Limtation to 32 ->
    // /* array for 13 (or 37 for system G) doubles */
    pub cusps: Vec<f64>,
    pub ascmc: [f64; 10],
    pub result: i32,
}

pub fn houses(
    tjd_ut: f64,
    geolat: f64,
    geolong: f64,
    hsys: char,
) -> HousesResult {
    let mut cusps = [0.0; 37];
    let mut ascmc = [0.0; 10];
    let result: i32 = unsafe {
        let p_cuspsw = cusps.as_mut_ptr();
        let p_ascmc = ascmc.as_mut_ptr();
        raw::swe_houses_ex(
            tjd_ut,
            0, // 64 | (64 * 1024),
            geolat,
            geolong,
            hsys as c_int,
            p_cuspsw,
            p_ascmc,
        )
    };
    HousesResult {
        cusps: cusps.to_vec(),
        ascmc: ascmc,
        result: result,
    }
}

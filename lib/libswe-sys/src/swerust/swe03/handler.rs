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
use crate::sweconst::Bodies;
use std::ffi::{CStr, CString};

/*
 * 3. The functions swe_calc_ut() and swe_calc()
 *
 * Before calling one of these functions or any other Swiss Ephemeris function,
 * it is strongly recommended to call the function swe_set_ephe_path(). Even if
 * you don’t want to set an ephemeris path and use the Moshier ephemeris, it is
 * nevertheless recommended to call swe_set_ephe_path(NULL), because this
 * function makes important initializations. If you don’t do that, the Swiss
 * Ephemeris may work but the results may be not 100% consistent.
 */

/*
 * xx
 *
 * Ecliptic position               Equatorial position (SEFLG_EQUATORIAL)
 * Longitude                       right ascension
 * Latitude                        declination
 * Distance in AU                  distance in AU
 * Speed in longitude (deg/day)    speed in right ascension (deg/day)
 * Speed in latitude (deg/day)     speed in declination (deg/day)
 * Speed in distance (AU/day)      speed in distance (AU/day)
 */
#[derive(Debug)]
pub struct CalcUtResult {
    pub longitude: f64,
    pub latitude: f64,
    pub distance_au: f64,
    pub speed_longitude: f64,
    pub speed_latitude: f64,
    pub speed_distance_au: f64,
    pub status: i32,
    pub serr: String,
}

pub fn calc_ut(tjd_ut: f64, ipl: Bodies, iflag: i32) -> CalcUtResult {
    let mut xx: [f64; 6] = [0.0; 6];
    let mut serr = [0; 255];
    let result = unsafe {
        let p_xx = xx.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        let status = raw::swe_calc_ut(tjd_ut, ipl as i32, iflag, p_xx, p_serr);
        let s_serr = CString::from(CStr::from_ptr(p_serr))
            .to_str()
            .unwrap()
            .to_string();
        CalcUtResult {
            longitude: xx[0],
            latitude: xx[1],
            distance_au: xx[2],
            speed_longitude: xx[3],
            speed_latitude: xx[4],
            speed_distance_au: xx[5],
            serr: s_serr,
            status: status,
        }
    };
    result
}

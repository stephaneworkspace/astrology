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
 * 7. Eclipses, risings, settings, meridian transits, planetary phenomena
 */
#[derive(Debug)]
pub struct PhenoUtResult {
    pub phase_angle: f64,
    pub phase_illuminated: f64,
    pub elongation_of_planet: f64,
    pub apparent_dimaeter_of_disc: f64,
    pub apparent_magnitude: f64,
    pub status: i32,
    pub serr: String,
}

pub fn pheno_ut(tjd_ut: f64, ipl: Bodies, iflag: i32) -> PhenoUtResult {
    let mut attr: [f64; 20] = [0.0; 20];
    let mut serr = [0; 255];
    let result = unsafe {
        let p_attr = attr.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        let status =
            raw::swe_pheno_ut(tjd_ut, ipl as i32, iflag, p_attr, p_serr);
        let s_serr = CString::from(CStr::from_ptr(p_serr))
            .to_str()
            .unwrap()
            .to_string();
        PhenoUtResult {
            phase_angle: attr[0],
            phase_illuminated: attr[1],
            elongation_of_planet: attr[2],
            apparent_dimaeter_of_disc: attr[3],
            apparent_magnitude: attr[4],
            serr: s_serr,
            status: status,
        }
    };
    result
}

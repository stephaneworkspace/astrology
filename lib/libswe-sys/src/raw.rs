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
/*#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)
*/

use std::os::raw::c_char;

#[link(name = "swe")]
extern "C" {
    // pub fn swe_test(path: *c_char); // swe_test try
    pub fn swe_set_ephe_path(path: *const c_char);
    // Set location jpl
    // pub fn swe_set_jpl_file(fname: *const c_char);
    /// Version
    pub fn swe_version(s_version: *mut c_char) -> *mut c_char;
    /// Free memory
    pub fn swe_close();
}

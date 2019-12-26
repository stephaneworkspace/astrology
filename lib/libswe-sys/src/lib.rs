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
use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
mod raw;
/// Get version
pub fn get_version() -> CString {
    // Get the version
    let mut version = [0; 255];
    let v = unsafe {
        let p = version.as_mut_ptr();
        raw::swe_version(p);
        CStr::from_ptr(p)
    };
    // Memory clean
    unsafe {
        raw::swe_close();
    }
    CString::from(v)
}

pub fn test_lib() -> String {
    // Initial path swissephem
    let path_begin = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_full = path_begin + "/lib/libswe-sys/src/swisseph/sweph";
    //let path_final = b"./swisseph/sweph".as_ptr() as *const c_char;
    let path_final = path_full.as_bytes().as_ptr() as *const c_char;
    unsafe {
        raw::swe_set_ephe_path(path_final);
    }

    // Get the version
    let mut version = [0; 255];
    let v = unsafe {
        let p = version.as_mut_ptr();
        raw::swe_version(p);
        CStr::from_ptr(p)
    };

    // Memory clean
    unsafe {
        raw::swe_close();
    }

    CString::from(v).to_str().unwrap().to_string()
}

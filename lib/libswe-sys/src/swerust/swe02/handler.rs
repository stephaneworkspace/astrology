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
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/*
 * 2. The Ephemeris file related functions
 */

/// Set the path of ephemeris
pub fn set_ephe_path(path: &str) {
    if path.len() > 255 {
        panic!("swe 2.1 -> set_ephe_path -> path to long");
    }
    let c_str = CString::new(path).unwrap();
    let path_final: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        raw::swe_set_ephe_path(path_final);
    }
}

/// Close swiss ephemeris, free memory
pub fn close() {
    unsafe { raw::swe_close() }
}

/// Set the path of ephemeris for working with JPL file
pub fn set_jpl_file(fname: &str) {
    if fname.len() > 255 {
        panic!("swe 2.3 -> set_jpl_file -> fname to long");
    }
    let c_str = CString::new(fname).unwrap();
    let fname_final: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        raw::swe_set_jpl_file(fname_final);
    }
}

/// Get version of swiss ephemeris
pub fn version() -> String {
    // Get the version
    let mut version = [0; 255];
    let v = unsafe {
        let p = version.as_mut_ptr();
        raw::swe_version(p);
        CStr::from_ptr(p)
    };
    CString::from(v).to_str().unwrap().to_string()
}

/// Get librarx path dll
pub fn get_library_path() -> String {
    // Get dll path
    let mut dll_path = [0; 255];
    let dll = unsafe {
        let p = dll_path.as_mut_ptr();
        raw::swe_get_library_path(p);
        CStr::from_ptr(p)
    };
    CString::from(dll).to_str().unwrap().to_string()
}

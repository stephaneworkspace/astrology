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
extern crate astrology_draw_svg;
extern crate ferris_says;

use libswe_sys::swerust::handler_swe02;
use std::ffi::CString;
use std::io::{stdout, BufWriter};
use std::os::raw::c_char;

/// Simple write in console
#[no_mangle]
pub extern "C" fn intro() {
    let phrase = b"Welcome to astro_compute_swisseph";
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(phrase, 40, &mut writer).unwrap();
}

/// Return version of api
#[no_mangle]
pub extern "C" fn sweversion() -> *const c_char {
    //CString::new(get_version()).unwrap().into_raw()
    CString::new(handler_swe02::version()).unwrap().into_raw()
}

// For yew front end
#[no_mangle]
pub extern "C" fn simple_svg() -> *const c_char {
    CString::new(astrology_draw_svg::write())
        .unwrap()
        .into_raw()
}

/// Unit test
#[cfg(test)]
mod tests {
    use libswe_sys::swerust::handler_swe02;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn version() {
        assert_eq!(handler_swe02::version(), "2.08");
    }
}

/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (https://github.com/stephaneworkspace)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an CC License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to this source in your commercial projects, you must
 * adhere to the GPL license or buy a Swiss Ephemeris commercial license.
 */
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
//#[macro_use]
extern crate strum_macros;

use libswe_sys::swerust::handler_swe02;
use serde::Deserialize;
use std::ffi::{CStr, CString};
pub mod astrology_draw_svg;
pub use self::astrology_draw_svg::DataChartNatal;
pub use std::os::raw::{c_char, c_double, c_int};
extern crate libc;
pub use self::astrology_draw_svg::{DataObjectSvg, DataObjectType};
pub use libc::size_t;

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hourf32: f64,
    pub hour: i32,
    pub min: i32,
    pub sec: f64,
    pub lat: f64,
    pub lng: f64,
}

#[no_mangle]
pub struct DataChartNatalC {
    pub year: c_int,
    pub month: c_int,
    pub day: c_int,
    pub hourf32: c_double,
    pub hour: c_int,
    pub min: c_int,
    pub sec: c_double,
    pub lat: c_double,
    pub lng: c_double,
}

/// Return version of api
#[no_mangle]
pub extern "C" fn sweversion() -> *const c_char {
    //CString::new(get_version()).unwrap().into_raw()
    CString::new(handler_swe02::version()).unwrap().into_raw()
}

// C -> Rust -> C
// This is the first try
#[repr(C)]
pub enum ObjectType {
    Chart,
    House,
    Zodiac,
}

#[repr(C)]
pub struct ObjectCanvas {
    size_x: c_double,
    size_y: c_double,
    pos_x: c_double,
    pos_y: c_double,
}

#[no_mangle]
pub extern "C" fn compute(
    year: c_int,
    month: c_int,
    day: c_int,
    hourf32: c_double,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    max_size: c_double,
    path: *const c_char,
) -> *const c_char {
    let d = DataChartNatalC {
        year: year,
        month: month,
        day: day,
        hourf32: hourf32,
        hour: hour,
        min: min,
        sec: sec,
        lat: lat,
        lng: lng,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data = astrology_draw_svg::chart(max_size as f32, d, &path_str);
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn compute_transit(
    year: c_int,
    month: c_int,
    day: c_int,
    hourf32: c_double,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    year_transit: c_int,
    month_transit: c_int,
    day_transit: c_int,
    hourf32_transit: c_double,
    hour_transit: c_int,
    min_transit: c_int,
    sec_transit: c_double,
    lat_transit: c_double,
    lng_transit: c_double,
    max_size: c_double,
    path: *const c_char,
) -> *const c_char {
    let d = DataChartNatalC {
        year: year,
        month: month,
        day: day,
        hourf32: hourf32,
        hour: hour,
        min: min,
        sec: sec,
        lat: lat,
        lng: lng,
    };
    let d_t = DataChartNatalC {
        year: year_transit,
        month: month_transit,
        day: day_transit,
        hourf32: hourf32_transit,
        hour: hour_transit,
        min: min_transit,
        sec: sec_transit,
        lat: lat_transit,
        lng: lng_transit,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data = astrology_draw_svg::chart_with_transit(
        max_size as f32,
        d,
        d_t,
        &path_str,
    );
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn aspects() -> *const c_char {
    let data = astrology_draw_svg::all_aspects();
    CString::new(serde_json::to_string(&data).unwrap())
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

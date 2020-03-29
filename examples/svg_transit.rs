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
use astrology::compute_transit;
use astrology::Data;
use std::env;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
fn main() {
    const PATH_TRANSIT: &str = "examples/transit.json";
    let mut s_transit = String::new();
    let mut file_path_transit = PathBuf::new();
    file_path_transit.push(env::current_dir().unwrap().as_path());
    file_path_transit.push(PATH_TRANSIT);
    File::open(file_path_transit.as_path())
        .unwrap()
        .read_to_string(&mut s_transit)
        .unwrap();
    let data_transit: Data = serde_json::from_str(&s_transit).unwrap();
    println!("Data: {:?}", data_transit);
    const PATH: &str = "examples/data.json";
    let mut s = String::new();
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push(PATH);
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let data: Data = serde_json::from_str(&s).unwrap();
    println!("Data: {:?}", data);
    let path = CString::new(
        "/Users/stephanebressani/Code/Flutter/astro/ios/EphemFiles/",
    )
    .expect("CString::new failled");
    let data_c_str = unsafe {
        CStr::from_ptr(compute_transit(
            data.year,
            data.month,
            data.day,
            data.hourf32,
            data.hour,
            data.min,
            data.sec,
            data.lat,
            data.lng,
            data_transit.year,
            data_transit.month,
            data_transit.day,
            data_transit.hourf32,
            data_transit.hour,
            data_transit.min,
            data_transit.sec,
            data_transit.lat,
            data_transit.lng,
            550.0,
            path.as_ptr(),
        ))
    };
    let _data_str: &str = data_c_str.to_str().unwrap();
    println!("{}", &_data_str);
}

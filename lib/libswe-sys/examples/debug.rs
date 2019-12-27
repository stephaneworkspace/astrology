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
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use libswe_sys::sweconst::Calandar;
use libswe_sys::swerust::{handler_swe02, handler_swe08};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Data {
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
}

fn main() {
    println!("Swissephem C -> Rust");
    let swe02_path_final = "src/swisseph/sweph";
    let swe02_path: String =
        env::var("CARGO_MANIFEST_DIR").unwrap() + swe02_path_final;
    println!("Set the path of ephemeris to: {}", swe02_path);
    handler_swe02::set_ephe_path(&swe02_path);
    println!("Version swephem: {}", handler_swe02::version());
    println!("Get path of library: {}", handler_swe02::get_library_path());

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
    println!(
        "Get julday: {:?}",
        handler_swe08::julday(
            data.year,
            data.month,
            data.day,
            data.hour,
            Calandar::Julian
        )
    );
    println!("Exit and free memory swephem");
    handler_swe02::close()
}

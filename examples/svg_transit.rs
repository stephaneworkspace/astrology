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
use astrology::cfg::parse_args_transit;
use astrology::svg_draw::{chart_svg_with_transit, DataChartNatal};
use chrono::{Datelike, Timelike};
use libswe_sys::sweconst::Language;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;

/// Write svg natal + transit chart
fn main() {
    let cfg = parse_args_transit();
    let d_n = DataChartNatal {
        year: cfg.date_n.year(),
        month: cfg.date_n.month(),
        day: cfg.date_n.day(),
        hour: cfg.time_n.hour(),
        min: cfg.time_n.minute(),
        sec: cfg.time_n.second() as f32,
        lat: cfg.lat_n,
        lng: cfg.lng_n,
        time_zone: cfg.time_zone_n,
    };
    let d_t = DataChartNatal {
        year: cfg.date_t.year(),
        month: cfg.date_t.month(),
        day: cfg.date_t.day(),
        hour: cfg.time_t.hour(),
        min: cfg.time_t.minute(),
        sec: cfg.time_t.second() as f32,
        lat: cfg.lat_t,
        lng: cfg.lng_t,
        time_zone: cfg.time_zone_t,
    };
    let mut file_export = File::create(&cfg.path_and_file).unwrap();
    let path = CString::new(cfg.path_ephem_files.as_str())
        .expect("CString::new failled");
    let path_c_str = unsafe { CStr::from_ptr(path.as_ptr()) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let svg: String =
        chart_svg_with_transit(1000.0, d_n, d_t, &path_str, Language::English);
    file_export.write_all(svg.as_bytes()).unwrap();
    println!("File exported to: {}", cfg.path_and_file);
}

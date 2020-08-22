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
use astrology::cfg::parse_args_natal;
use astrology::svg_draw::{chart_svg, DataChartNatal};
use chrono::{Datelike, Timelike};
use libswe_sys::sweconst::{AspectsFilter, Language};
use num_traits::FromPrimitive;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;

/// Write svg natal chart
fn main() {
    let cfg = parse_args_natal();
    let d = DataChartNatal {
        year: cfg.date.year(),
        month: cfg.date.month(),
        day: cfg.date.day(),
        hour: cfg.time.hour(),
        min: cfg.time.minute(),
        sec: cfg.time.second() as f32,
        lat: cfg.lat,
        lng: cfg.lng,
        time_zone: cfg.time_zone,
    };
    let mut file_export = File::create(&cfg.path_and_file).unwrap();
    let path = CString::new(cfg.path_ephem_files.as_str())
        .expect("CString::new failled");
    let path_c_str = unsafe { CStr::from_ptr(path.as_ptr()) };
    let path_str: &str = path_c_str.to_str().unwrap();
    println!("{}", &path_str);
    let aspect: AspectsFilter = match FromPrimitive::from_u32(cfg.aspect) {
        Some(a) => a,
        None => AspectsFilter::NoAspects,
    };

    let svg: String =
        chart_svg(cfg.size as f32, d, &path_str, Language::English, aspect);
    file_export.write_all(svg.as_bytes()).unwrap();
    println!("File exported to: {}", cfg.path_and_file);
}

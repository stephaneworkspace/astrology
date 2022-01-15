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

    let (svg, vec_aspect) =
        chart_svg(cfg.size as f32, d, &path_str, Language::English, aspect);
    for x in vec_aspect {
        println!("{}", x);
    }
    file_export.write_all(svg.as_bytes()).unwrap();
    println!("File exported to: {}", cfg.path_and_file);
}

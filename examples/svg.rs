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
use astrology::cfg::parse_args;
use astrology::svg_draw::{
    chart, DataChartNatal, DataObjectSvg, DataObjectType,
};
use base64::encode;
use chrono::{Datelike, Timelike};
use libswe_sys::sweconst::Language;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;

/// Write chart to PATH_EXPORT (change it for you)
fn main() {
    let cfg = parse_args();
    println!("Configuration: {:?}", cfg);
    let d = DataChartNatal {
        year: cfg.date.year(),
        month: cfg.date.month() as i32,
        day: cfg.date.day() as i32,
        hour: cfg.time.hour() as i32,
        min: cfg.time.minute() as i32,
        sec: cfg.time.second() as f32,
        lat: cfg.lat,
        lng: cfg.lng,
    };
    let mut file_export = File::create(&cfg.path_and_file).unwrap();
    let path = CString::new(cfg.path_ephem_files.as_str())
        .expect("CString::new failled");
    let path_c_str = unsafe { CStr::from_ptr(path.as_ptr()) };
    let path_str: &str = path_c_str.to_str().unwrap();
    println!("{}", &path_str);

    let res: Vec<DataObjectSvg> =
        chart(1000.0, d, &path_str, Language::English);
    let mut svg_res: String = "".to_string();
    for r in res.clone() {
        if r.object_type == DataObjectType::Chart {
            svg_res = r.svg;
        }
    }
    if svg_res != "" {
        svg_res = svg_res.replace("</svg>", "");
        for r in res {
            if r.object_type != DataObjectType::Chart {
                // to do better inside after for real use
                svg_res = format!("{}<image width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" href=\"data:image/svg+xml;base64,{}\"/>", svg_res, r.size_x, r.size_y, r.pos_x, r.pos_y, encode(r.svg.as_str()));
            }
        }
    } else {
        svg_res = "<svg>".to_string();
    }
    svg_res = format!("{}</svg>", svg_res);
    file_export.write_all(svg_res.as_bytes()).unwrap();
    println!("File exported to: {}", cfg.path_and_file);
    //let res: Vec<DataObjectSvg>;

    //let _data_str: &str = data_c_str.to_str().unwrap();
    //println!("{}", &_data_str);
}

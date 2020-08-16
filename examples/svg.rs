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
use chrono::{Datelike, NaiveDate, NaiveTime, Timelike, Utc};
use libswe_sys::sweconst::Language;
use std::env;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

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

    // const PATH: &str = "examples/data.json";
    // const PATH_EXPORT: &str = "/Users/stephanebressani/Svg/chart.svg";
    // let mut file_export = File::create(PATH_EXPORT).unwrap();
    // let mut s = String::new();
    // let mut file_path = PathBuf::new();
    //TODO -> default path windows and unix
    // file_path.push(env::current_dir().unwrap().as_path());
    // file_path.push(PATH);
    // File::open(file_path.as_path())
    //    .unwrap()
    //    .read_to_string(&mut s)
    //    .unwrap();
    // let data: DataChartNatal = serde_json::from_str(&s).unwrap();
    // println!("Data: {:?}", data);
    let path = CString::new(
        "/Users/stephanebressani/Code/Flutter/astro/ios/EphemFiles/",
    )
    .expect("CString::new failled");
    // let d = DataChartNatal {
    //    year: data.year,
    //    month: data.month,
    //    day: data.day,
    //    hour: data.hour,
    //    min: data.min,
    //    sec: data.sec as f32,
    //    lat: data.lat as f32,
    //    lng: data.lng as f32,
    // };
    let path_c_str = unsafe { CStr::from_ptr(path.as_ptr()) };
    let path_str: &str = path_c_str.to_str().unwrap();
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
    //println!("File exported to: {}", PATH_EXPORT);
    //let res: Vec<DataObjectSvg>;

    //let _data_str: &str = data_c_str.to_str().unwrap();
    //println!("{}", &_data_str);
}

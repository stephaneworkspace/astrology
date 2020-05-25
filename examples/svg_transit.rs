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
use astrology::svg_draw::{chart_with_transit, DataChartNatal, DataObjectType};
use base64::encode;
use libswe_sys::sweconst::Language;
use std::env;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Write svg transit to PATH_EXPORT (change it for you)
fn main() {
    const PATH_EXPORT: &str = "/Users/stephanebressani/Svg/chart.svg";
    let mut file_export = File::create(PATH_EXPORT).unwrap();

    const PATH_TRANSIT: &str = "examples/transit.json";
    let mut s_transit = String::new();
    let mut file_path_transit = PathBuf::new();
    file_path_transit.push(env::current_dir().unwrap().as_path());
    file_path_transit.push(PATH_TRANSIT);
    File::open(file_path_transit.as_path())
        .unwrap()
        .read_to_string(&mut s_transit)
        .unwrap();
    let data_transit: DataChartNatal =
        serde_json::from_str(&s_transit).unwrap();
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
    let data: DataChartNatal = serde_json::from_str(&s).unwrap();
    println!("Data: {:?}", data);
    let path = CString::new(
        "/Users/stephanebressani/Code/Flutter/astro/ios/EphemFiles/",
    )
    .expect("CString::new failled");

    let lang = Language::English;
    let d = DataChartNatal {
        year: data.year,
        month: data.month,
        day: data.day,
        hour: data.hour,
        min: data.min,
        sec: data.sec as f32,
        lat: data.lat as f32,
        lng: data.lng as f32,
    };
    let d_t = DataChartNatal {
        year: data_transit.year,
        month: data_transit.month,
        day: data_transit.day,
        hour: data_transit.hour,
        min: data_transit.min,
        sec: data_transit.sec as f32,
        lat: data_transit.lat as f32,
        lng: data_transit.lng as f32,
    };
    let path_c_str = unsafe { CStr::from_ptr(path.as_ptr()) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let res = chart_with_transit(1000.0, d, d_t, &path_str, lang);
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
    println!("File exported to: {}", PATH_EXPORT);
}

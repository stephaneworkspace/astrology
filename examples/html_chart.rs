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
use astrology::export_chart_html;
use astrology::DataChartNatal;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
fn main() {
    const PATH_CHART_NATAL: &str = "examples/data.json";
    let mut s = String::new();
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push(PATH_CHART_NATAL);
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let data: DataChartNatal = serde_json::from_str(&s).unwrap();

    println!("Read: {:?}", data);

    export_chart_html(550.0, "/Users/stephanebressani/Svg/index.html", data)
        .expect("Error generate svg html");
}

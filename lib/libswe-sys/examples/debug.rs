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
use libswe_sys::swerust::handler_swe02;
use libswe_sys::test_lib;
use std::env;

fn main() {
    println!("Swissephem C -> Rust");
    println!("");
    println!("------------------------------");
    println!("");

    // swe02
    // Setting the path
    let swe02_path_final = "src/swisseph/sweph";
    let swe02_path: String =
        env::var("CARGO_MANIFEST_DIR").unwrap() + swe02_path_final;
    println!("Set the path of ephemeris to: {}", swe02_path);
    handler_swe02::set_ephe_path(&swe02_path);

    println!("");
    println!("------------------------------");
    println!("");

    println!("{}", test_lib());
}

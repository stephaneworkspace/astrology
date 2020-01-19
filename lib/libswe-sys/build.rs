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
use std::env;
use std::path::Path;

fn main() {
    /*
     * Old Way
     * Not work with cargo
     *
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir)
                .join("src/swisseph/2.08/src/build")
                .display()
        );
    */
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cc::Build::new()
        .flag("-g")
        //        .flag("-09")
        .flag("-Wall")
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swecl.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swedate.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swehel.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swehouse.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swejpl.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swemmoon.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swemplan.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swepcalc.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/sweph.c"))
        .file(Path::new(&dir).join("src/swisseph/2.08/src/swephlib.c"))
        .compile("swe")
}

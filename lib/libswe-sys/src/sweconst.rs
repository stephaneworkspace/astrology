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
#![allow(non_camel_case_types)]
pub enum Bodies {
    SE_ECL_NUT = -1,
    SE_SUN = 0,
    SE_MOON = 1,
    SE_MERCURY = 2,
    SE_VENUS = 3,
    SE_MARS = 4,
    SE_JUPITER = 5,
    SE_SATURN = 6,
    SE_URANUS = 7,
    SE_NEPTUNE = 8,
    SE_PLUTO = 9,
    SE_MEAN_NODE = 10,
    SE_TRUE_NODE = 11,
    SE_MEAN_APOG = 12,
    SE_OSCU_APOG = 13,
    SE_EARTH = 14,
    SE_CHIRON = 15,
    SE_PHOLUS = 16,
    SE_CERES = 17,
    SE_PALLAS = 18,
    SE_JUNO = 19,
    SE_VESTA = 20,
    SE_INTP_APOG = 21,
    SE_INTP_PERG = 22,
    SE_NPLANETS = 23,
    // SE_FICT_OFFSET = 40,
    // SE_NFICT_ELEM = 15,
    SE_AST_OFFSET = 10000,
    /* Hamburger or Uranian "planets" */
    SE_CUPIDO = 40,
    SE_HADES = 41,
    SE_ZEUS = 42,
    SE_KRONOS = 43,
    SE_APOLLON = 44,
    SE_ADMETOS = 45,
    SE_VULKANUS = 46,
    SE_POSEIDON = 47,
    /* other fictitious bodies */
    SE_ISIS = 48,
    SE_NIBIRU = 49,
    SE_HARRINGTON = 50,
    SE_NEPTUNE_LEVERRIER = 51,
    SE_NEPTUNE_ADAMS = 52,
    SE_PLUTO_LOWELL = 53,
    SE_PLUTO_PICKERING = 54,
}

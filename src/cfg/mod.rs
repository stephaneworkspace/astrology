/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (https://github.com/stephaneworkspace)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * This software uses the swiss ephemeris which is licensed GPL.
 *
 * Therefore, if you want to this source in your commercial projects, you must
 * adhere to the GPL license or buy a Swiss Ephemeris commercial license.
 */
mod args;
mod parse;
mod validator;
pub use self::args::{
    parse_args_natal, parse_args_transit, AstrologyConfig,
    AstrologyTransitConfig,
};

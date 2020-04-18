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
/*extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
*/
//#[macro_use]
//extern crate strum_macros;
pub mod svg_draw;
//pub use self::svg_draw::DataChartNatal;
//pub use self::svg_draw::{DataObjectSvg, DataObjectType};

/*pub enum ObjectType {
    Chart,
    House,
    Zodiac,
}

pub struct ObjectCanvas {
    size_x: c_double,
    size_y: c_double,
    pos_x: c_double,
    pos_y: c_double,
}
*/

/// Unit test
#[cfg(test)]
mod tests {
    use libswe_sys::swerust::handler_swe02;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn version() {
        assert_eq!(handler_swe02::version(), "2.08");
    }
}

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
pub mod angles;
pub mod aspects;
pub mod bodies;
pub mod compute_chart;
pub mod houses;
pub mod numbers;
pub mod svg_draw;
pub mod zodiacs;
pub use self::compute_chart::{
    chart, chart_svg, chart_svg_with_transit, chart_with_transit,
    DataChartNatal, DataObjectAspectSvg, DataObjectSvg, DataObjectType,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

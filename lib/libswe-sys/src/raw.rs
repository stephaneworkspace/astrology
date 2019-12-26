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

use std::os::raw::c_char;
/// Interface https://www.astro.com/ftp/swisseph/doc/swephprg.htm#_Toc19111156
#[link(name = "swe")]
extern "C" {
    /*
     * 2. The Ephemeris file related functions
     */

    /// 2.1
    /// void swe_set_ephe_path(
    ///     char *path);
    pub fn swe_set_ephe_path(path: *const c_char);

    /// 2.2
    /// /* close Swiss Ephemeris */
    /// void swe_close(
    ///     void);
    pub fn swe_close();

    /// 2.3
    /// /* set name of JPL ephemeris file */
    /// void swe_set_jpl_file(
    ///     char *fname);
    pub fn swe_set_jpl_file(fname: *const c_char);

    /// 2.4
    /// /* find out version number of your Swiss Ephemeris version */
    /// char *swe_version(
    ///     char *svers);
    /// /* svers is a string variable with sufficient space to contain the
    /// version number (255 char) */
    pub fn swe_version(s_version: *mut c_char) -> *mut c_char;
}

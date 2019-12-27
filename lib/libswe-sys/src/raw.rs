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
use std::os::raw::{c_char, c_double, c_int};
/// Interface https://www.astro.com/ftp/swisseph/doc/swephprg.htm#_Toc19111156
#[link(name = "swe")]
extern "C" {
    /*
     * 2. The Ephemeris file related functions
     */

    /// void swe_set_ephe_path(
    ///     char *path);
    pub fn swe_set_ephe_path(path: *const c_char);

    /// /* close Swiss Ephemeris */
    /// void swe_close(
    ///     void);
    pub fn swe_close();

    /// /* set name of JPL ephemeris file */
    /// void swe_set_jpl_file(
    ///     char *fname);
    pub fn swe_set_jpl_file(fname: *const c_char);

    /// /* find out version number of your Swiss Ephemeris version */
    /// char *swe_version(
    ///     char *svers);
    /// /* svers is a string variable with sufficient space to contain the
    /// version number (255 char) */
    pub fn swe_version(s_version: *mut c_char) -> *mut c_char;

    /// /* find out the library path of the DLL or executable */
    /// char *swe_get_library_path(
    ///     char *spath);
    /// /* spath is a string variable with sufficient space to contain the
    /// library path (255 char) */
    pub fn swe_get_library_path(spath: *mut c_char) -> *mut c_char;

    /*
     * 3. The functions swe_calc_ut() and swe_calc()
     * Before calling one of these functions or any other Swiss Ephemeris
     * function, it is strongly recommended to call the function
     * swe_set_ephe_path(). Even if you don’t want to set an ephemeris path and
     * use the Moshier ephemeris, it is nevertheless recommended to call
     * swe_set_ephe_path(NULL), because this function makes important
     * initializations. If you don’t do that, the Swiss Ephemeris may work but
     * the results may be not 100% consistent.
     */

    /// int swe_calc_ut(
    ///     double tjd_ut,
    ///     int ipl,
    ///     int iflag,
    ///     double* xx,
    ///     char* serr);
    ///
    /// tjd_ut    = Julian day, Universal Time
    /// ipl       = body number
    /// iflag     = a 32 bit integer containing bit flags that indicate what
    ///             kind of computation is wanted
    /// xx        = array of 6 doubles for longitude, latitude, distance, speed
    ///             in long., speed in lat., and speed in dist.
    /// serr[256] = character string to return error messages in case of error.
    pub fn swe_calc_ut(
        tjd_ut: c_double,
        ipl: c_int,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;
}

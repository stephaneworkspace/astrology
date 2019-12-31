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

    /*
     * 7. Eclipses, risings, settings, meridian transits, planetary phenomena
     */

    /// int32 swe_pheno_ut(
    ///     double tjd_ut,       /* time Jul. Day UT */
    ///     int32 ipl,           /* planet number */
    ///     int32 iflag,         /* ephemeris flag */
    ///     double *attr,        /* return array, 20 doubles, see below */
    ///     char *serr);         /* return error string */
    pub fn swe_pheno_ut(
        tjd_ut: c_double,
        ipl: c_int,
        iflag: c_int,
        atr: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    /*
     * 8. Date and time conversion functions
     */

    /// double swe_julday(
    ///     int year,
    ///     int month,
    ///     int day,
    ///     double hour,
    ///     int gregflag);
    pub fn swe_julday(
        year: c_int,
        month: c_int,
        day: c_int,
        hour: c_double,
        gregflag: c_int,
    ) -> c_double;

    /// void swe_utc_time_zone(
    ///     int32 iyear, int32 imonth, int32 iday,
    ///     int32 ihour, int32 imin, double dsec,
    ///     double d_timezone,
    ///     int32 *iyear_out, int32 *imonth_out, int32 *iday_out,
    ///     int32 *ihour_out, int32 *imin_out, double *dsec_out);
    pub fn swe_utc_time_zone(
        iyear: c_int,
        imonth: c_int,
        iday: c_int,
        ihour: c_int,
        imin: c_int,
        dsec: c_double,
        d_timezone: c_double,
        iyear_out: *mut c_int,
        imonth_out: *mut c_int,
        iday_out: *mut c_int,
        ihour_out: *mut c_int,
        imin_out: *mut c_int,
        dsec_out: *mut c_double,
    );

    /// int32 swe_utc_to_jd(
    /// int32 iyear, int32 imonth, int32 iday,
    /// int32 ihour, int32 imin, double dsec,          /* NOTE: second is a decimal */
    /// gregflag,            /* Gregorian calendar: 1, Julian calendar: 0 */
    /// dret                 /* return array, two doubles:
    ///                       * dret[0] = Julian day in ET (TT)
    ///                       * dret[1] = Julian day in UT (UT1) */
    /// serr);               /* error string */
    pub fn swe_utc_to_jd(
        iyear: c_int,
        imonth: c_int,
        iday: c_int,
        ihour: c_int,
        imin: c_int,
        dsec: c_double,
        gregflag: c_int,
        dret: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    /*
     * 14. House cups calculation
     */

    /// int swe_house_names(
    ///     int hsys);
    pub fn swe_house_name(hsys: c_int) -> *mut c_char;

    /// int swe_houses_ex(
    ///     double tjd_ut,
    ///     int32 iflag,
    ///     double geolat,
    ///     double geolon,
    ///     int hsys,
    ///     double *cusps,
    ///     double *ascmc);
    pub fn swe_houses_ex(
        tjd_ut: c_double,
        iflag: c_int,
        geolat: c_double,
        geolon: c_double,
        hsys: c_int,
        cusps: *mut c_double,
        ascmc: *mut c_double,
    ) -> c_int;

    /*
     * 17. Auxiliary functions
     */

    /// double swe_degnorm(double x);
    pub fn swe_degnorm(x: c_double) -> c_double;

    /// double swe_radnorm(double x);
    pub fn swe_radnorm(x: c_double) -> c_double;

    /// double swe_split_deg(
    ///     double ddeg,
    ///     int32 roundflag,
    ///     int32 *ideg,
    ///     int32 *imin,
    ///     int32 *isec,
    ///     double *dsecfr,
    ///     int32 *isgn);
    pub fn swe_split_deg(
        ddeg: c_double,
        roundflag: c_int,
        ideg: *mut c_int,
        imin: *mut c_int,
        isec: *mut c_int,
        cdegfr: *mut c_double,
        isgn: *mut c_int,
    ) -> c_double;
}

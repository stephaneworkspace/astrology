/*#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)
*/

use std::os::raw::c_char;

#[link(name = "swe")]
extern "C" {
    // pub fn swe_test(path: *c_char); // swe_test try
    pub fn swe_set_ephe_path(path: *const c_char);
    // Set location jpl
    // pub fn swe_set_jpl_file(fname: *const c_char);
    /// Version
    pub fn swe_version(s_version: *mut c_char) -> *mut c_char;
    /// Free memory
    pub fn swe_close();
}

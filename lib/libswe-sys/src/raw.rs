/*#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)
*/

use std::os::raw::c_uchar;

#[link(name = "swe")]
extern "C" {
    // pub fn swe_test(path: *const c_char); // swe_test try
    pub fn swe_set_ephe_path(path: *const c_uchar);
    /// Version
    pub fn swe_version(s_version: &*mut c_uchar) -> *mut c_uchar;
    /// Free memory
    pub fn swe_close();
}

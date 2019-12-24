use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
mod raw;
pub fn test_lib() -> String {
    // Initial path swissephem
    let path_begin = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_full = path_begin + "/lib/libswe-sys/src/swisseph/sweph";
    //let path_final = b"./swisseph/sweph".as_ptr() as *const c_char;
    let path_final = path_full.as_bytes().as_ptr() as *const c_char;
    unsafe {
        raw::swe_set_ephe_path(path_final);
    }

    // Get the version
    let mut version = [0; 255];
    let v = unsafe {
        let p = version.as_mut_ptr();
        raw::swe_version(p);
        CStr::from_ptr(p)
    };

    // Memory clean
    unsafe {
        raw::swe_close();
    }

    CString::from(v).to_str().unwrap().to_string()
}

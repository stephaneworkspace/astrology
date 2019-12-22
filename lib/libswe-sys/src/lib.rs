use std::env;
//use std::ffi::CString;
use std::os::raw::c_uchar;
use std::ptr;
mod raw;
pub fn test_lib() -> String {
    // Initial path swissephem
    let path_begin = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_full = path_begin + "/lib/libswe-sys/src/swisseph/sweph";
    //let path_final = b"./swisseph/sweph".as_ptr() as *const c_uchar;
    let path_final = path_full.as_bytes().as_ptr() as *const c_uchar;
    let version = ptr::null_mut() as *mut c_uchar;
    //let version: *mut c_uchar = "\0".as_bytes().as_ptr() as *mut c_uchar;
    //let version_for_ferry: Vec<u8>;
    let version = ptr::null_mut() as *mut c_uchar;
    unsafe {
        raw::swe_set_ephe_path(path_final);
        // Get the version
        raw::swe_version(&version);
        // let test_unsafe = CString::from_raw(version as *mut i8).as_bytes();
        //version_for_ferry = CString::from_raw(version as *mut i8);
        // Free memory
        raw::swe_close();
    }
    // Return to rusty
    // path_full
    path_full
}

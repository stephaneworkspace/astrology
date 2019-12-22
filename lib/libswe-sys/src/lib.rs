use std::env;
use std::os::raw::c_char;

mod raw;
pub fn test_lib() -> String {
    // Initial path swissephem
    let path_begin = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_full = path_begin + "/lib/libswe-sys/src/swisseph/sweph";
    //let path_final = b"./swisseph/sweph".as_ptr() as *const c_char;
    let path_final = path_full.as_bytes().as_ptr() as *const c_char;
    let version: *mut c_char = "\0".as_bytes().as_ptr() as *mut c_char;
    unsafe {
        raw::swe_set_ephe_path(path_final);
        // Get the version
        raw::swe_version(version);
        // Free memory
        raw::swe_close();
    }
    // Return to rusty
    path_full
}

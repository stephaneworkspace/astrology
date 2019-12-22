use std::os::raw::c_char;

mod raw;
pub fn test_lib() -> String {
    // Initial path swissephem
    let path = b"./swisseph/sweph".as_ptr() as *const c_char;
    unsafe {
        raw::swe_test(path);
    }
    // Return to rusty
    "Path set swe_set_ephe_path".to_string()
}

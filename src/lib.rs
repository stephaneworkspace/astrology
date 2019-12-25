extern crate ferris_says;

//use libswe_sys::get_version;
use std::ffi::CString;
use std::io::{stdout, BufWriter};
use std::os::raw::c_char;

/// Simple write in console
#[no_mangle]
pub extern "C" fn example_intro() {
    let phrase = b"Welcome to astro_compute_swisseph";
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(phrase, 40, &mut writer).unwrap();
}

/// Return version of api
#[no_mangle]
pub extern "C" fn sweversion() -> *mut c_char {
    // get_version().as_ptr()
    // CString::new(get_version()).unwrap().into_raw()
    CString::new("Hello world").unwrap().into_raw()
}
/*
/// This code is not detect by lipo
/// I think &[u8] is not valid
#[no_mangle]
pub extern "C" fn example_from_lib(phrase: &[u8]) {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(phrase, 40, &mut writer).unwrap();
}
*/
/// Unit test
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

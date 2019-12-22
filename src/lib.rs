extern crate ferris_says;

use std::io::{stdout, BufWriter};

/// Simple write in console
#[no_mangle]
pub extern "C" fn example_intro() {
    let phrase = b"Welcome to astro_compute_swisseph";
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(phrase, 40, &mut writer).unwrap();
}

#[no_mangle]
pub extern "C" fn example_from_lib(phrase: &[u8]) {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(phrase, 40, &mut writer).unwrap();
}

/// Unit test
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

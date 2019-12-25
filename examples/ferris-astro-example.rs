extern crate libswe_sys;

use astro::*;
use libswe_sys::test_lib;

fn main() {
    example_from_lib(test_lib().as_bytes());
}

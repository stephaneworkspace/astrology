use std::env;
use std::path::Path;

fn main() {
    // println!("cargo:rustc-link-lib=static=swe");
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir)
            .join("src/swisseph/2.08/src/build")
            .display()
    );
}

# Traditional astrology for rust

Rust library by Stéphane Bressani (s.bressani@bluewin.ch)

Using swissephem c library by Astrodienst AG
by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an CC License, which allows it to be used 
also on commercial projects. This software uses the swiss ephemeris which is
licensed GPL.

Therefore, if you want to use astro_compute_swisseph in your commercial
projects, you must adhere to the GPL license or buy a Swiss Ephemeris
commercial license.

# Use
Actuallay the version is like a hello world (0.1), just this method is available:

```
pub extern "C" fn sweversion() -> *const c_char {
    CString::new(handler_swe02::version()).unwrap().into_raw()
}
```

# Version
0.1.23
* Update library libswe-sys c 0.1.5 -> 0.1.6 (add standard lib path for compile
  wasm in yew)

0.1.22
* Update library libswe-sys c 0.1.4 -> 0.1.5 (add math.h for compile wasm in
  yew)

0.1.21
* Add 20° svg
* Add 21° svg
* Add 22° svg
* Add 23° svg
* Add 24° svg
* Add 25° svg
* Add 26° svg
* Add 27° svg
* Add 28° svg
* Add 29° svg
* Add 30° svg

0.1.20
* Add 13° svg
* Add 14° svg
* Add 15° svg
* Add 16° svg
* Add 17° svg
* Add 18° svg
* Add 19° svg

0.1.19
* Update library swissephem in Cargo.toml (not OK, see :
  https://stackoverflow.com/questions/60188673/rust-ffi-wasm-yew-cargo-web-start-fatal-error-math-h-file-not-found)

0.1.18
* Trait for yew export for this project https://github.com/stephaneworkspace/yewastrology
* Only Circle chart and Bodies, without x and y, just svg for yew export

0.1.17
* Add 6° svg
* Add 7° svg
* Add 8° svg
* Add 9° svg
* Add 10° svg
* Add 11° svg
* Add 12° svg

0.1.16
* Add 2° svg
* Add 3° svg
* Add 4° svg
* Add 5° svg

0.1.15
* Add 1° svg

0.1.14
* Add file for degres ° and minutes '
* Add 0° svg

0.1.13
* Change license to Creative Commons

0.1.12
* Add Pisces svg

0.1.11
* Add Aquarius svg

0.1.10
* Add Capricorn svg

0.1.9
* Add Sagittarus svg

0.1.8
* Add Scorpio svg

0.1.7
* Add Libra svg

0.1.6
* Add Virgo svg

0.1.5
* Add Leo svg

0.1.4
* Add Cancer svg

0.1.3
* Add Gemini svg

0.1.2
* Add Taurus svg

0.1.1
* Add Aries svg

0.1.0
* Simple swiss ephemeris call based on version 2.8 of swiss ephemeris

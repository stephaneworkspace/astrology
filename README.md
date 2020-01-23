# Traditional astrology for rust

Rust library by Stéphane Bressani (s.bressani@bluewin.ch)

Using swissephem c library by Astrodienst AG
by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an MIT License, which allows it to be used
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
0.1.5
Add Leo svg

0.1.4
Add Cancer svg

0.1.3
Add Gemini svg

0.1.2
Add Taurus svg

0.1.1
Add Aries svg

0.1.0
Simple swiss ephemeris call based on version 2.8 of swiss ephemeris

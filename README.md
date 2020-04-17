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

# Example

![Example](https://i.ibb.co/zRm7fsW/theme30avril2007.png)

# Use of example
Create a data.json, modify example/svg.rs for the path of swissephem files and 
execute svg in example

# Use of library

For get the version as a pointer const c_char :

```
pub extern "C" fn sweversion() -> *const c_char {
    CString::new(handler_swe02::version()).unwrap().into_raw()
}
```

For get a json with all svg and position inside as a pointer const c_char:
```
pub extern "C" fn compute(
    year: c_int,
    month: c_int,
    day: c_int,
    hourf32: c_double,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    max_size: c_double,
    path: *const c_char,
) -> *const c_char {
    let d = DataChartNatalC {
        year: year,
        month: month,
        day: day,
        hourf32: hourf32,
        hour: hour,
        min: min,
        sec: sec,
        lat: lat,
        lng: lng,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data = astrology_draw_svg::chart(max_size as f32, d, &path_str);
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}
```

For all aspects (only major at this moment)

```
#[no_mangle]
pub extern "C" fn aspects() -> *const c_char {
    let data = astrology_draw_svg::all_aspects();
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}

```

For transit

```
#[no_mangle]
pub extern "C" fn compute_transit(
    year: c_int,
    month: c_int,
    day: c_int,
    hourf32: c_double,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    year_transit: c_int,
    month_transit: c_int,
    day_transit: c_int,
    hourf32_transit: c_double,
    hour_transit: c_int,
    min_transit: c_int,
    sec_transit: c_double,
    lat_transit: c_double,
    lng_transit: c_double,
    max_size: c_double,
    path: *const c_char,
) -> *const c_char {
    let d = DataChartNatalC {
        year: year,
        month: month,
        day: day,
        hourf32: hourf32,
        hour: hour,
        min: min,
        sec: sec,
        lat: lat,
        lng: lng,
    };
    let d_t = DataChartNatalC {
        year: year_transit,
        month: month_transit,
        day: day_transit,
        hourf32: hourf32_transit,
        hour: hour_transit,
        min: min_transit,
        sec: sec_transit,
        lat: lat_transit,
        lng: lng_transit,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data = astrology_draw_svg::chart_with_transit(
        max_size as f32,
        d,
        d_t,
        &path_str,
    );
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}
```

# Version
0.1.71
* Add some text for aspects on french

0.1.70
* Translate on french some text for aspect in svg, now svg for english and
  french

0.1.69
* Svg in exemple create svg picture

0.1.68
* Add another path /usr/local/opt/llvm/include for wasm (libsew_sys)

0.1.66
* Add another path for /usr/include for the last vrate libswe_sys for
  compatibiliy with osx /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include

0.1.65
* Support wasm on mac with last crate libswe_sys with /usr/include for c
  headers

0.1.64
* Support wasm on mac os with last crate libswe-sys using llvm this time (bew
  install llvm)

0.1.63
* Support wasm on mac os with the last crate libswe_sys with path
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin

0.1.62
* Updated the crate libswe_sys for support wasm on mac os

0.1.61
* Optimization code and file naming

0.1.60
* Fix bug documentation docs.rs

0.1.59
* Optimization of svg_draw.rs with polymorphism, the code is clean now

0.1.58
* Compile with crate 0.1.48 libswe_sys (begining of support theme colors)

0.1.57
* Supress unused functions

0.1.56
* Add transit aspects

0.1.55
* Add begining of transit, aspect in transit is not implemented in this version

0.1.54
* Add fortuna part

0.1.53
* Minors aspects drawed to the natal chart

0.1.52
* Majors aspects drawed to the natal chart

0.1.51
* Create a function for have all aspects (major at this moment only)

0.1.50
* Correct lilith value (true, not mean)

0.1.49
* Add majors aspects svg

0.1.48
* Add Lilith mean to chart

0.1.47
* Add correct Ceres svg

0.1.46
* Add south node
* Add bash script compilation for ffi export

0.1.45
* Add zodiac color fix some bugs

0.1.44
* Add zodiac color
* Change README for call c extern

0.1.43
* Add collision bodies/angle detection for write the natal chart

0.1.42
* Add Asc svg
* Add Fc Svg
* Add Desc svg
* Add Mc Svg
* Draw all angles in render svg chart

0.1.41
* Implement retrograde function
* Draw house numero on chart

0.1.40
* Change interface for c -> rust -> c with chart data in params

0.1.39
* Add planets svg to chart

0.1.38
* New interface c -> rust -> c with a pointer array

0.1.37
* Fix one error in c -> rust -> c for draw svg

0.1.36
* Simple function in c-rust-c with the chart in svg into "ptr const string" 

0.1.35
* Add house 1 svg
* Add house 2 svg
* Add house 3 svg
* Add house 4 svg
* Add house 5 svg
* Add house 6 svg
* Add house 7 svg
* Add house 8 svg
* Add house 9 svg
* Add house 10 svg
* Add house 11 svg
* Add house 12 svg

0.1.34
* Add angle pointer in svg chart

0.1.33
* Add pointer to all houses in svg chart

0.1.32
* Add rules for all 12 zodiac signs with examples/data.json values for
  examples/html_chart.rs
* Some tests code for c -> rust -> c are temporary deleted

0.1.31
* Add rules for zodiac without using libswe_sys, just compute with Aries =
  0°0'0""

0.1.30
* Add 10' svg
* Add 11' svg
* Add 12' svg
* Add 13' svg
* Add 14' svg
* Add 15' svg
* Add 16' svg
* Add 17' svg
* Add 18' svg
* Add 19' svg
* Add 20' svg
* Add 21' svg
* Add 22' svg
* Add 23' svg
* Add 24' svg
* Add 25' svg
* Add 26' svg
* Add 27' svg
* Add 28' svg
* Add 29' svg
* Add 30' svg
* Add 31' svg
* Add 32' svg
* Add 33' svg
* Add 34' svg
* Add 35' svg
* Add 36' svg
* Add 37' svg
* Add 38' svg
* Add 39' svg
* Add 40' svg
* Add 41' svg
* Add 42' svg
* Add 43' svg
* Add 44' svg
* Add 45' svg
* Add 46' svg
* Add 47' svg
* Add 48' svg
* Add 49' svg
* Add 50' svg
* Add 51' svg
* Add 52' svg
* Add 53' svg
* Add 54' svg
* Add 55' svg
* Add 56' svg
* Add 57' svg
* Add 58' svg
* Add 59' svg

0.1.29
* Add 0' svg
* Add 1' svg
* Add 2' svg
* Add 3' svg
* Add 4' svg
* Add 5' svg
* Add 6' svg
* Add 7' svg
* Add 8' svg
* Add 9' svg

0.1.28
* Update library libswe-sy to 0.1.13

0.1.27
* Update library libswe-sys to 0.1.11

0.1.26
* Update library libswe-sys 0.1.7 -> 0.1.9

0.1.25
* Update library libswe-sys 0.1.6 -> 0.1.7

0.1.24
* Forgot update library libswe-sys 0.1.5 -> 0.1.6

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

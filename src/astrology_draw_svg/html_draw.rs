/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (https://github.com/stephaneworkspace)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an CC License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to this source in your commercial projects, you must
 * adhere to the GPL license or buy a Swiss Ephemeris commercial license.
 */
pub const HTML_HEAD: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>Astrology</title>
        <style>
            .svg-base {
                background-repeat: no-repeat;
            }
            .element {
                position: absolute; 
                width: 100%; 
                height: 100%; 
                display: flex; 
                justify-content: center; 
            }
        </style>
    </head>
"#;

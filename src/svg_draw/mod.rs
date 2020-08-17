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
extern crate base64;
extern crate libswe_sys;
extern crate serde;
extern crate strum;
use libswe_sys::sweconst::{Aspects, Language, Theme};
pub mod angles;
pub mod aspects;
pub mod bodies;
pub mod houses;
pub mod numbers;
pub mod zodiacs;
use aspects::{
    aspects_all_aspects, aspects_draw, aspects_maj_aspects,
    aspects_min_aspects, aspects_no_aspect,
};
pub mod compute_chart;
pub mod svg_draw;
pub use self::compute_chart::{
    chart, chart_svg, chart_with_transit, DataChartNatal, DataObjectAspectSvg,
    DataObjectSvg, DataObjectType,
};
pub use self::svg_draw::{
    WorkingStorageDrawPolyMorphNatal, WorkingStorageDrawPolyMorphTransit,
    WorkingStoragePolyMorphNatal, WorkingStoragePolyMorphTransit,
};
use strum::IntoEnumIterator;
pub use svg_draw::*;

pub fn all_aspects(lang: Language) -> Vec<DataObjectAspectSvg> {
    let mut res: Vec<DataObjectAspectSvg> = Vec::new();
    // No aspect
    let va_no_aspect: Vec<Aspects> = Vec::new();
    let mut t;
    t = match lang {
        // To do const
        Language::English => "No aspect".to_string(),
        Language::French => "Pas d'aspect".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_no_aspect(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_no_aspect,
    });

    // Maj aspects
    let mut va_maj_aspects: Vec<Aspects> = Vec::new();
    for a in Aspects::iter() {
        if a.maj() {
            va_maj_aspects.push(a.clone());
        }
    }
    t = match lang {
        // To do const
        Language::English => "Majors aspects".to_string(),
        Language::French => "Aspects majeurs".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_maj_aspects(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_maj_aspects,
    });

    // Single Maj aspects
    for a in Aspects::iter() {
        if a.maj() {
            let mut va: Vec<Aspects> = Vec::new();
            va.push(a as Aspects);
            res.push(DataObjectAspectSvg {
                svg: aspects_draw(a, Theme::Light, lang).to_string(),
                text: a.text(lang),
                aspects: va.clone(),
            });
            va.clear()
        }
    }

    // Min aspects
    let mut va_min_aspects: Vec<Aspects> = Vec::new();
    for a in Aspects::iter() {
        if !a.maj() {
            va_min_aspects.push(a.clone());
        }
    }
    t = match lang {
        // To do const
        Language::English => "Minors aspects".to_string(),
        Language::French => "Aspects mineurs".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_min_aspects(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_min_aspects,
    });

    // Single Min aspects
    for a in Aspects::iter() {
        if !a.maj() {
            let mut va: Vec<Aspects> = Vec::new();
            va.push(a as Aspects);
            res.push(DataObjectAspectSvg {
                svg: aspects_draw(a, Theme::Light, lang).to_string(),
                text: a.text(lang),
                aspects: va.clone(),
            });
            va.clear()
        }
    }

    // All aspects
    let mut va_all_aspects: Vec<Aspects> = Vec::new();
    for a in Aspects::iter() {
        va_all_aspects.push(a.clone());
    }
    t = match lang {
        // To do const
        Language::English => "All aspects".to_string(),
        Language::French => "Tous les aspects".to_string(),
    };
    res.push(DataObjectAspectSvg {
        svg: aspects_all_aspects(Theme::Light, lang).to_string(),
        text: t,
        aspects: va_all_aspects,
    });

    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
extern crate strum;
use crate::svg_draw::svg_draw::{
    WorkingStorageDrawPolyMorphNatal, WorkingStorageDrawPolyMorphTransit,
};
use libswe_sys::sweconst::{Bodies, Theme};
use svg::node::element::path::{Data, Number};
use svg::node::element::{Group, Path};
use svg::Document;
pub const ANGLE_SIZE: Number = 50.0;

impl WorkingStorageDrawPolyMorphNatal {
    pub fn angles_draw_asc(&self) -> Document {
        draw_asc(self.ws.theme)
    }
    pub fn angles_draw_fc(&self) -> Document {
        draw_fc(self.ws.theme)
    }
    pub fn angles_draw_desc(&self) -> Document {
        draw_desc(self.ws.theme)
    }
    pub fn angles_draw_mc(&self) -> Document {
        draw_mc(self.ws.theme)
    }
}

impl WorkingStorageDrawPolyMorphTransit {
    pub fn angles_draw_asc(&self) -> Document {
        draw_asc(self.ws.theme)
    }
    pub fn angles_draw_fc(&self) -> Document {
        draw_fc(self.ws.theme)
    }
    pub fn angles_draw_desc(&self) -> Document {
        draw_desc(self.ws.theme)
    }
    pub fn angles_draw_mc(&self) -> Document {
        draw_mc(self.ws.theme)
    }
}

/// Draw the Ascendant text (AS in english)
fn draw_asc(theme: Theme) -> Document {
    let size: (Number, Number) = (ANGLE_SIZE, ANGLE_SIZE);
    let document: Document;
    // To do use Primary/Secondary/Etc... color for that
    let color: String =
        format!("#{:06X}", Bodies::EclNut.object_color(theme) as i32);
    let data1 = Data::new()
        .move_to((9.2, 28.6)) // M
        .line_by((-2.5, 7.6)) // l
        .horizontal_line_to(3.4) // H
        .line_by((8.2, -24.3)) // l
        .horizontal_line_by(3.8) // h
        .line_by((8.3, 24.3)) // l
        .horizontal_line_by(-3.3) // h
        .line_by((-2.6, -7.6)) // l
        .horizontal_line_to(9.2) // H
        .close() // z
        .move_to((17.1, 26.1)) // M
        .line_by((-2.4, -7.0)) // l
        .cubic_curve_by((-0.5, -1.6, -0.9, -3.0, -1.3, -4.4)) // c
        .horizontal_line_by(-0.1) // h
        .cubic_curve_by((-0.4, 1.4, -0.8, 2.9, -1.2, 4.4)) // c
        .line_by((-2.4, 7.0)) // l
        .horizontal_line_to(17.1) // H
        .close(); // z
    let data2 = Data::new()
        .move_to((43.7, 35.4)) // M
        .cubic_curve_by((-1.2, 0.6, -3.5, 1.2, -6.4, 1.2)) // c
        .cubic_curve_by((-6.8, 0.0, -12.0, -4.3, -12.0, -12.3)) // c
        .cubic_curve_by((0.0, -7.6, 5.1, -12.7, 12.7, -12.7)) // c
        .cubic_curve_by((3.0, 0.0, 4.9, 0.6, 5.8, 1.1)) // c
        .line_to((43.0, 15.2)) // L
        .cubic_curve_by((-1.2, -0.6, -2.9, -1.0, -4.9, -1.0)) // c
        .cubic_curve_by((-5.7, 0.0, -9.5, 3.6, -9.5, 10.0)) // c
        .cubic_curve_by((0.0, 5.9, 3.4, 9.8, 9.3, 9.8)) // c
        .cubic_curve_by((1.9, 0.0, 3.9, -0.4, 5.1, -1.0)) // c
        .line_to((43.7, 35.4)) // L
        .close(); // z
    let path1 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data1);
    let path2 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data2);
    let group = Group::new().add(path1).add(path2);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

/// Draw the Fc text (FC in english)
fn draw_fc(theme: Theme) -> Document {
    let size: (Number, Number) = (ANGLE_SIZE, ANGLE_SIZE);
    let document: Document;
    let color: String =
        format!("#{:06X}", Bodies::EclNut.object_color(theme) as i32);
    let data1 = Data::new()
        .move_to((7.2, 11.9)) // M
        .horizontal_line_by(13.1) // h
        .vertical_line_by(2.6) // v
        .horizontal_line_by(-9.9) // h
        .vertical_line_by(8.1) // v
        .horizontal_line_by(9.2) // h
        .vertical_line_by(2.6) // v
        .horizontal_line_by(-9.2) // h
        .vertical_line_by(11.0) // v
        .horizontal_line_to(7.2) // H
        .vertical_line_to(11.9) // V
        .close(); // z
    let data2 = Data::new()
        .move_to((41.7, 35.4)) // M
        .cubic_curve_by((-1.2, 0.6, -3.5, 1.2, -6.4, 1.2)) // c
        .cubic_curve_by((-6.8, 0.0, -12.0, -4.3, -12.0, -12.3)) // c
        .cubic_curve_by((0.0, -7.6, 5.1, -12.7, 12.7, -12.7)) // c
        .cubic_curve_by((3.0, 0.0, 4.9, 0.6, 5.8, 1.1)) // c
        .line_to((41.0, 15.2)) // L
        .cubic_curve_by((-1.2, -0.6, -2.9, -1.0, -4.9, -1.0)) // c
        .cubic_curve_by((-5.7, 0.0, -9.5, 3.6, -9.5, 10.0)) // c
        .cubic_curve_by((0.0, 5.9, 3.4, 9.8, 9.3, 9.8)) // c
        .cubic_curve_by((1.9, 0.0, 3.9, -0.4, 5.1, -1.0)) // c
        .line_to((41.7, 35.4)) // L
        .close(); // z
    let path1 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data1);
    let path2 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data2);
    let group = Group::new().add(path1).add(path2);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

/// Draw the Desc text (DC in english)
fn draw_desc(theme: Theme) -> Document {
    let size: (Number, Number) = (ANGLE_SIZE, ANGLE_SIZE);
    let document: Document;
    let color: String =
        format!("#{:06X}", Bodies::EclNut.object_color(theme) as i32);
    let data1 = Data::new()
        .move_to((4.0, 12.3)) // M
        .cubic_curve_by((1.9, -0.3, 4.2, -0.5, 6.7, -0.5)) // c
        .cubic_curve_by((4.5, 0.0, 7.7, 1.0, 9.8, 3.0)) // c
        .cubic_curve_by((2.2, 2.0, 3.4, 4.8, 3.4, 8.7)) // c
        .cubic_curve_by((0.0, 4.0, -1.2, 7.2, -3.5, 9.4)) // c
        .cubic_curve_by((-2.3, 2.3, -6.0, 3.5, -10.7, 3.5)) // c
        .cubic_curve_by((-2.2, 0.0, -4.1, -0.1, -5.7, -0.3)) // c
        .vertical_line_to(12.3) // V
        .close() // z
        .move_to((7.1, 33.7)) // M
        .cubic_curve_by((0.8, 0.1, 1.9, 0.2, 3.2, 0.2)) // c
        .cubic_curve_by((6.7, 0.0, 10.3, -3.7, 10.3, -10.3)) // c
        .cubic_curve_by((0.0, -5.7, -3.2, -9.4, -9.8, -9.4)) // c
        .cubic_curve_by((-1.6, 0.0, -2.8, 0.1, -3.7, 0.3)) // c
        .vertical_line_to(33.7) // V
        .close(); // z
    let data2 = Data::new()
        .move_to((44.9, 35.4)) // M
        .cubic_curve_by((-1.2, 0.6, -3.5, 1.2, -6.4, 1.2)) // c
        .cubic_curve_by((-6.8, 0.0, -12.0, -4.3, -12.0, -12.3)) // c
        .cubic_curve_by((0.0, -7.6, 5.1, -12.7, 12.7, -12.7)) // c
        .cubic_curve_by((3.0, 0.0, 4.9, 0.6, 5.8, 1.1)) // c
        .line_by((-0.8, 2.6)) // l
        .cubic_curve_by((-1.2, -0.6, -2.9, -1.0, -4.9, -1.0)) // c
        .cubic_curve_by((-5.7, 0.0, -9.5, 3.6, -9.5, 10.0)) // c
        .cubic_curve_by((0.0, 5.9, 3.4, 9.8, 9.3, 9.8)) // c
        .cubic_curve_by((1.9, 0.0, 3.9, -0.4, 5.1, -1.0)) // c
        .line_to((44.9, 35.4)) // L
        .close(); // z
    let path1 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data1);
    let path2 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data2);
    let group = Group::new().add(path1).add(path2);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

/// Draw the Mc text (CC in english)
fn draw_mc(theme: Theme) -> Document {
    let size: (Number, Number) = (ANGLE_SIZE, ANGLE_SIZE);
    let document: Document;
    let color: String =
        format!("#{:06X}", Bodies::EclNut.object_color(theme) as i32);
    let data1 = Data::new()
        .move_to((22.0, 25.5)) // M
        .cubic_curve_by((-0.2, -3.4, -0.4, -7.5, -0.4, -10.5)) // c
        .horizontal_line_by(-0.1) // h
        .cubic_curve_by((-0.8, 2.8, -1.8, 5.9, -3.1, 9.2)) // c
        .line_by((-4.3, 11.8)) // l
        .horizontal_line_by(-2.4) // h
        .line_to((7.8, 24.5)) // L
        .cubic_curve_by((-1.2, -3.4, -2.1, -6.6, -2.8, -9.4)) // c
        .horizontal_line_to(5.0) // H
        .cubic_curve_by((-0.1, 3.0, -0.3, 7.1, -0.5, 10.7)) // c
        .line_to((3.8, 36.2)) // L
        .horizontal_line_by(-3.0) // h
        .line_by((1.7, -24.3)) // l
        .horizontal_line_by(4.0) // h
        .line_by((4.1, 11.7)) // l
        .cubic_curve_by((1.0, 3.0, 1.8, 5.7, 2.4, 8.2)) // c
        .horizontal_line_by(0.1) // h
        .cubic_curve_by((0.6, -2.4, 1.5, -5.1, 2.6, -8.2)) // c
        .line_by((4.3, -11.7)) // l
        .horizontal_line_by(4.0) // h
        .line_by((1.5, 24.3)) // l
        .horizontal_line_by(-3.1) // h
        .line_to((22.0, 25.5)) // L
        .close(); // z
    let data2 = Data::new()
        .move_to((47.4, 35.4)) // M
        .cubic_curve_to((46.3, 36.0, 44.0, 36.6, 41.0, 36.6)) // C
        .cubic_curve_by((-6.8, 0.0, -12.0, -4.3, -12.0, -12.3)) // c
        .cubic_curve_by((0.0, -7.6, 5.1, -12.7, 12.7, -12.7)) // c
        .cubic_curve_by((3.0, 0.0, 4.9, 0.6, 5.8, 1.1)) // c
        .line_by((-0.8, 2.6)) // l
        .cubic_curve_by((-1.2, -0.6, -2.9, -1.0, -4.9, -1.0)) // c
        .cubic_curve_by((-5.7, 0.0, -9.5, 3.6, -9.5, 10.0)) // c
        .cubic_curve_by((0.0, 5.9, 3.4, 9.8, 9.3, 9.8)) // c
        .cubic_curve_by((1.9, 0.0, 3.9, -0.4, 5.1, -1.0)) // c
        .line_to((47.4, 35.4)) // L
        .close(); // z
    let path1 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data1);
    let path2 = Path::new()
        .set("fill", color.clone())
        .set("stroke", color.clone())
        .set("stroke-width", 0)
        .set("d", data2);
    let group = Group::new().add(path1).add(path2);
    document = Document::new()
        .set("viewBox", (0, 0, size.0, size.1))
        .add(group);
    document
}

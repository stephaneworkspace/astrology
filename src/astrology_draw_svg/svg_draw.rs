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
extern crate libswe_sys;
extern crate strum;
//use strum::AsStaticRef;
use libswe_sys::sweconst::Signs;
use std::f32;
use strum::IntoEnumIterator; // Enum for loop
use svg::node::element::path::Number;
use svg::node::element::{Circle, Line};
use svg::Document;

// Working Storage - CONST

// Const size in %
// circle 360°
const CIRCLE: Number = 360.0;

// tuple (visible/value)
const CIRCLE_SIZE: [(bool, Number); 7] = [
    (true, 35.0),  // 0
    (true, 62.0),  // 1
    (true, 67.0),  // 2
    (false, 75.0), // 3
    (false, 80.0), // 4
    (false, 87.0), // 5
    (false, 94.0), // 6
];

// Working Storage - Enums
// #[derive(Debug, Clone, PartialEq, Display)]
pub enum LargerDrawLine {
    Small,
    Large,
}

// Working Storage - Struct
#[derive(Debug, Clone)]
pub struct WorkingStorage {
    pub max_size: Number,
}

#[derive(Debug, Clone)]
pub struct WorkingStorageDraw {
    ws: WorkingStorage,
}

// Struct
#[derive(Debug, Clone)]
pub struct Offset {
    pub x: Number,
    pub y: Number,
}

// Interfaces

pub trait Draw {
    fn draw_base(&self) -> Document;
}

pub trait CalcDraw {
    fn get_radius_total(&self) -> Number;
    fn get_radius_circle(&self, occurs: usize) -> (Number, bool);
    fn get_center_equal(&self, max_size: Number) -> Offset;
    fn get_center(&self) -> Offset;
    fn get_line_trigo(
        &self,
        angular: Number,
        radius_circle_begin: Number,
        radius_circle_end: Number,
    ) -> [Offset; 2];
}

// Methods - Constructors

impl WorkingStorage {
    pub fn new(max_size: Number) -> WorkingStorage {
        WorkingStorage { max_size: max_size }
    }
}

impl WorkingStorageDraw {
    pub fn new(ws: WorkingStorage) -> WorkingStorageDraw {
        WorkingStorageDraw { ws: ws }
    }
}

// Methods

impl Draw for WorkingStorageDraw {
    fn draw_base(&self) -> Document {
        let calc_draw = self.ws.clone();
        let center: (Number, Number) = (
            calc_draw.get_radius_total() as Number,
            calc_draw.get_radius_total() as Number,
        );

        let mut circle = Vec::new();
        for (i, ele) in CIRCLE_SIZE.iter().enumerate() {
            if ele.0 {
                circle.push(
                    Circle::new()
                        .set("fill", "none")
                        .set("cx", center.0)
                        .set("cy", center.1)
                        .set("r", calc_draw.get_radius_circle(i).0)
                        .set("stroke", "black")
                        .set("stroke-width", 1),
                );
            }
        }

        // Draw zodiac simple with begin at Aries 0°0'0"
        // https://github.com/stephaneworkspace/astrologie/blob/master/lib/draw_astro.dart
        let mut larger_draw_line = LargerDrawLine::Small;
        let mut line_degre = Vec::new();
        for i in Signs::iter() {
            let mut sign = i as i32;
            // 0°
            // temporary Aries 0°0'0"
            let pos = (sign as f32 - 1.0) * 30.0;
            // let pos = sign_pos_circle;
            let a_xy: [Offset; 2] = self.ws.get_line_trigo(
                pos,
                self.ws.get_radius_circle(1).0,
                self.ws.get_radius_circle(0).0,
            );
            line_degre.push(
                Line::new()
                    .set("x1", a_xy[0].x)
                    .set("x2", a_xy[0].y)
                    .set("y1", a_xy[1].x)
                    .set("y2", a_xy[1].y),
            );
            // 1° to 29°
            for j in 1..15 {
                if j == 5 || j == 10 || j == 15 {
                    larger_draw_line = LargerDrawLine::Large;
                } else {
                    larger_draw_line = LargerDrawLine::Small;
                }
            }
        }
        //for i in 1..12 {}

        // pub enum LargerDrawLine {

        let document = Document::new()
            //.set("baseProfile", "full")
            //.set("version", "1.1")
            //.set("xmlns:xlink", "http://www.w3.org/1999/xlink")
            .set(
                "viewBox",
                (0, 0, self.ws.max_size as i32, self.ws.max_size as i32),
            )
            .add(circle[0].clone())
            .add(circle[1].clone())
            .add(circle[2].clone());
        document
    }
}

impl CalcDraw for WorkingStorage {
    fn get_radius_total(&self) -> Number {
        self.max_size / 2.0
    }

    fn get_radius_circle(&self, occurs: usize) -> (Number, bool) {
        if occurs > CIRCLE_SIZE.len() {
            panic!("Out of range in circle occurs: {}", occurs);
        }
        (
            (self.get_radius_total() * CIRCLE_SIZE[occurs].1) / 100.0,
            CIRCLE_SIZE[occurs].0,
        )
    }

    fn get_center_equal(&self, max_size: Number) -> Offset {
        let result = max_size / 2.0;
        Offset {
            x: result,
            y: result,
        }
    }

    fn get_center(&self) -> Offset {
        Offset {
            x: self.get_radius_total(),
            y: self.get_radius_total(),
        }
    }

    fn get_line_trigo(
        &self,
        angular: Number,
        radius_circle_begin: Number,
        radius_circle_end: Number,
    ) -> [Offset; 2] {
        let dx1: Number = self.get_center().x
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_begin as f32;
        let dx2: Number = self.get_center().y
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * -1.0
                * radius_circle_begin as f32;
        let dy1: Number = self.get_center().x
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_end as f32;
        let dy2: Number = self.get_center().y
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * -1.0
                * radius_circle_end as f32;
        [Offset { x: dx1, y: dx2 }, Offset { x: dy1, y: dy2 }]
    }
}

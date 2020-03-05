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
use libswe_sys::sweconst::{Angle, House}; // , Signs};
use libswe_sys::swerust::handler_swe14::HousesResult;
use std::f32;
// use strum::IntoEnumIterator; // Enum for loop
use svg::node::element::path::{Data, Number};
use svg::node::element::{Circle, Group, Line, Path};
use svg::Document;

// Working Storage - CONST

// Const size in %
// circle 360°
const CIRCLE: Number = 360.0;

const LARGER_DRAW_LINE_RULES_SMALL: Number = 0.1;
const LARGER_DRAW_LINE_RULES_LARGE: Number = 0.2;

// tuple (visible/value)
const CIRCLE_SIZE: [(Number, bool); 7] = [
    (35.0, true),  // 0
    (62.0, true),  // 1
    (67.0, true),  // 2
    (75.0, false), // 3
    (80.0, false), // 4
    (87.0, false), // 5
    (94.0, false), // 6
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
    pub house: Vec<House>,
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
    fn get_radius_rules_inside_circle(
        &self,
        larger_draw_line: LargerDrawLine,
    ) -> Number;
    fn get_radius_rules_inside_circle_house_for_pointer_bottom(&self)
        -> Number;
    fn get_radius_rules_inside_circle_house_for_pointer_top(&self) -> Number;
    fn get_center_equal(&self, max_size: Number) -> Offset;
    fn get_center(&self) -> Offset;
    fn get_line_trigo(
        &self,
        angular: Number,
        radius_circle_begin: Number,
        radius_circle_end: Number,
    ) -> [Offset; 2];
    fn get_triangle_path(
        &self,
        angular: Number,
        angular_pointer: Number,
        radius_circle_begin: Number,
        radius_circle_end: Number,
    ) -> [Offset; 3];
}

// Methods - Constructors

impl WorkingStorage {
    pub fn new(max_size: Number, house: HousesResult) -> WorkingStorage {
        let mut h: Vec<House> = Vec::new();
        for (i, res) in house.clone().cusps.iter().enumerate() {
            if i > 0 {
                // angle
                let angle;
                angle = match i - 1 {
                    1 => Angle::Asc,
                    4 => Angle::Fc,
                    7 => Angle::Desc,
                    10 => Angle::Mc,
                    _ => Angle::Nothing,
                };
                h.push(House::new(i as i32, res.clone(), angle));
                if i + 1 > 12 {
                    break;
                }
            }
        }
        WorkingStorage {
            max_size: max_size,
            house: h,
        }
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
            // ele.0 = size
            // ele.1 = bool if printed
            if ele.1 {
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
        let mut larger_draw_line;

        // Draw rules and sign separation
        let mut line_degre = Vec::new();

        // For all 12 zodiac signs
        for i in 0..12 {
            // Signs::iter() { // for loop like 0..12
            let sign = i as i32;
            // 0°
            // temporary Aries 0°0'0"
            let off_pos_asc: f32 = self.ws.house[0].longitude as f32;
            let mut pos = (sign as f32 - 1.0) * 30.0 + &off_pos_asc;
            if pos > 360.0 {
                pos = pos - 360.0;
            }
            // let pos = sign_pos_circle;
            let a_xy: [Offset; 2] = self.ws.get_line_trigo(
                pos,
                self.ws.get_radius_circle(1).0,
                self.ws.get_radius_circle(0).0,
            );
            line_degre.push(
                Line::new()
                    .set("x1", a_xy[0].x)
                    .set("y1", a_xy[0].y)
                    .set("x2", a_xy[1].x)
                    .set("y2", a_xy[1].y)
                    .set("stroke", "black")
                    .set("stroke-width", 1),
            );
            // 1° to 29°
            for j in 1..15 {
                if j == 5 || j == 10 || j == 15 {
                    larger_draw_line = LargerDrawLine::Large;
                } else {
                    larger_draw_line = LargerDrawLine::Small;
                }
                // temporary Aries 0°0'0"
                pos = ((sign as f32 - 1.0) * 30.0)
                    + (j as f32 * 2.0)
                    + &off_pos_asc;
                // to do
                // pos = sign_pos_circle + (j as f32 * 2.0);
                if pos > 360.0 {
                    pos = pos - 360.0;
                }
                let a_xy_line: [Offset; 2] = self.ws.get_line_trigo(
                    pos,
                    self.ws.get_radius_circle(1).0,
                    self.ws.get_radius_rules_inside_circle(larger_draw_line),
                );
                line_degre.push(
                    Line::new()
                        .set("x1", a_xy_line[0].x)
                        .set("y1", a_xy_line[0].y)
                        .set("x2", a_xy_line[1].x)
                        .set("y2", a_xy_line[1].y)
                        .set("stroke", "black")
                        .set("stroke-width", 1),
                );
            }
        }

        // Debug
        // for (l, i) in (&line_degre).iter().enumerate() {
        //    println!("{}{}", i, l.to_string());
        // }

        let mut group_degre: Group = Group::new();
        for i in 0..line_degre.len() {
            group_degre = group_degre.add(line_degre[i].clone());
        }

        // Draw house separation with pointer -> (path triangle draw in svg)
        let mut line_house = Vec::new();
        let mut triangle_house = Vec::new();
        // For all 12 house delimiter
        for i in 0..12 {
            let house_pos: f32 = self.ws.house[i].longitude as f32;
            let angular_pointer = 1.0; // Todo angular pointer varying
                                       // iphone/ipad, need to be a CONST
            let a_xy_tria: [Offset; 3];
            let a_xy_line: [Offset; 2];
            if self.ws.house[i].angle == Angle::Nothing {
                a_xy_tria = self.ws.get_triangle_path(
                  house_pos,
                  angular_pointer,
                  self.ws
                      .get_radius_rules_inside_circle_house_for_pointer_bottom(),
                  self.ws
                      .get_radius_rules_inside_circle_house_for_pointer_top(),
              );
                // Line for small pointer (only if angle is Angle::Nothing)
                a_xy_line = self.ws.get_line_trigo(
                    house_pos,
                    self.ws.get_radius_circle(2).0,
                    self.ws.get_radius_circle(1).0,
                );
                line_house.push(
                    Line::new()
                        .set("x1", a_xy_line[0].x)
                        .set("y1", a_xy_line[0].y)
                        .set("x2", a_xy_line[1].x)
                        .set("y2", a_xy_line[1].y)
                        .set("stroke", "black")
                        .set("stroke-width", 1),
                );
            } else {
                a_xy_tria = self.ws.get_triangle_path(
                    house_pos,
                    angular_pointer,
                    self.ws.get_radius_circle(2).0,
                    self.ws.get_radius_circle(1).0,
                );
            }
            triangle_house.push(
                Path::new()
                    .set("fill", "black")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set(
                        "d",
                        Data::new()
                            .move_to((a_xy_tria[2].x, a_xy_tria[2].y)) // M
                            .line_to((a_xy_tria[0].x, a_xy_tria[0].y)) // L
                            .line_to((a_xy_tria[1].x, a_xy_tria[1].y)) // L
                            .close(), // z
                    ),
            );
        }

        let mut group_house: Group = Group::new();
        for i in 0..line_house.len() {
            group_house = group_house.add(line_house[i].clone());
        }
        for i in 0..triangle_house.len() {
            group_house = group_house.add(triangle_house[i].clone());
        }

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
            .add(circle[2].clone())
            .add(group_degre)
            .add(group_house);
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
            (self.get_radius_total() * CIRCLE_SIZE[occurs].0) / 100.0,
            CIRCLE_SIZE[occurs].1,
        )
    }

    fn get_radius_rules_inside_circle(
        &self,
        larger_draw_line: LargerDrawLine,
    ) -> Number {
        let size = match larger_draw_line {
            LargerDrawLine::Small => 1.0 + LARGER_DRAW_LINE_RULES_SMALL,
            LargerDrawLine::Large => 1.0 + LARGER_DRAW_LINE_RULES_LARGE,
        };
        self.get_radius_total()
            * (((CIRCLE_SIZE[1].0 - CIRCLE_SIZE[0].0) / size)
                + CIRCLE_SIZE[0].0)
            / 100.0
    }

    /// Find position bottom of pointer
    ///     ..
    ///    /  \
    ///     ||
    ///     ||
    ///    HERE
    fn get_radius_rules_inside_circle_house_for_pointer_bottom(
        &self,
    ) -> Number {
        let div_trait_pointer = 1.5; // Todo CONST varying iphone/ipad
        (self.get_radius_total()
            * (((CIRCLE_SIZE[2].0 - CIRCLE_SIZE[1].0) / div_trait_pointer)
                - CIRCLE_SIZE[2].0))
            / 100.0
    }

    /// Top of pointer
    ///     HERE
    ///     /  \
    ///      ||
    ///      ||
    fn get_radius_rules_inside_circle_house_for_pointer_top(&self) -> Number {
        (self.get_radius_total()
            * ((CIRCLE_SIZE[2].0 - CIRCLE_SIZE[1].0) - CIRCLE_SIZE[2].0))
            / 100.0
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
    fn get_triangle_path(
        &self,
        angular: Number,
        angular_pointer: Number,
        radius_circle_begin: Number,
        radius_circle_end: Number,
    ) -> [Offset; 3] {
        let mut angular1 = angular as f32 - angular_pointer as f32;
        if angular1 > 360.0 {
            angular1 = angular1 - 360.0;
        }
        let mut angular2 = angular as f32 + angular_pointer as f32;
        if angular2 > 360.0 {
            angular2 = angular2 - 360.0;
        }
        let dx1: Number = self.get_center().x
            + (angular1 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_begin as f32;
        let dy1: Number = self.get_center().y
            + (angular1 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * -1.0
                * radius_circle_begin as f32;
        let dx2: Number = self.get_center().x
            + (angular2 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_begin as f32;
        let dy2: Number = self.get_center().y
            + (angular2 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * -1.0
                * radius_circle_begin as f32;
        let dx3: Number = self.get_center().x
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_end as f32;
        let dy3: Number = self.get_center().y
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * -1.0
                * radius_circle_end as f32;
        [
            Offset { x: dx1, y: dy1 },
            Offset { x: dx2, y: dy2 },
            Offset { x: dx3, y: dy3 },
        ]
    }
}

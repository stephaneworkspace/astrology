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
use crate::astrology_draw_svg::svg_draw_angle::{
    draw_asc as svg_draw_asc, draw_desc as svg_draw_desc,
    draw_fc as svg_draw_fc, draw_mc as svg_draw_mc, ANGLE_SIZE,
};
use crate::astrology_draw_svg::svg_draw_bodies::draw_bodie as svg_draw_bodie;
use crate::astrology_draw_svg::svg_draw_bodies::BODIE_SIZE;
use crate::astrology_draw_svg::svg_draw_house::draw_house as svg_draw_house;
use crate::astrology_draw_svg::svg_draw_house::HOUSE_SIZE;
use crate::astrology_draw_svg::svg_draw_numbers::draw_degre as svg_draw_degre;
use crate::astrology_draw_svg::svg_draw_numbers::draw_minute as svg_draw_minute;
use crate::astrology_draw_svg::svg_draw_numbers::{DEG_SIZE, MIN_SIZE};
use crate::astrology_draw_svg::svg_draw_zodiac::draw_zodiac as svg_draw_zodiac;
use crate::astrology_draw_svg::svg_draw_zodiac::ZODIAC_SIZE;
use libswe_sys::sweconst::{Angle, Bodies, House, Object, ObjectPos, Signs};
use libswe_sys::swerust::handler_swe14::HousesResult;
use std::f32;
use strum::IntoEnumIterator;
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
const CIRCLE_SIZE: [(Number, bool); 9] = [
    (35.0, true),  // 0
    (62.0, true),  // 1
    (67.0, true),  // 2
    (76.0, false), // 3
    (78.0, false), // 4
    (88.0, false), // 5
    (95.0, false), // 6
    (70.0, false), // 7 between 2 and 3
    (71.0, false), // 8 correction planet between 2 and 3
];

const BODIE_DISTANCE: Number = 5.0;

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
    pub object: Vec<Object>,
    pub temp_position_bodies: Vec<TempPositionBodies>,
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

#[derive(Debug, Clone)]
pub struct SvgObject {
    pub svg: String,
    pub size_x: Number,
    pub size_y: Number,
    pub pos_x: Number,
    pub pos_y: Number,
}

#[derive(Debug, Clone)]
pub struct SvgObjectBodie {
    pub svg: String,
    pub size_x: Number,
    pub size_y: Number,
    pub pos_x: Number,
    pub pos_y: Number,
    pub deg_svg: String,
    pub deg_size_x: Number,
    pub deg_size_y: Number,
    pub deg_pos_x: Number,
    pub deg_pos_y: Number,
    pub min_svg: String,
    pub min_size_x: Number,
    pub min_size_y: Number,
    pub min_pos_x: Number,
    pub min_pos_y: Number,
    pub trait_svg: String,
    pub trait_size_x: Number, // max_size
    pub trait_size_y: Number, // max_size
    pub trait_pos_x: Number,  // 0.0
    pub trait_pos_y: Number,  // 0.0
}

#[derive(Debug, Clone, Copy)]
pub struct TempPositionBodies {
    pub init_index: i16,
    pub index: i16,
    pub sw_reserve: bool,
    pub sw_bodie: bool,
    pub bodie_enum: Bodies,
    pub longitude: Number,
    pub space_left: Number,
    pub space_right: Number,
    pub longitude_fix: Number,
}

// Interfaces

pub trait Draw {
    fn draw_base(&self) -> Document;
    fn draw_zodiac(&self, sign: Signs) -> SvgObject;
    fn draw_house(&self, numero: i16) -> SvgObject;
    fn draw_angle(&self, angle: Angle) -> SvgObjectBodie;
    fn draw_bodie(&self, bodie: Bodies) -> SvgObjectBodie;
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
    fn get_radius_circle_zodiac(&self) -> Number;
    fn get_radius_circle_house(&self) -> Number;
    fn get_center_equal(&self, max_size: Number) -> Offset;
    fn get_center(&self) -> Offset;
    fn get_center_item(&self, size: Number, offset: Offset) -> Offset;
    fn get_distance_between_2_offset(
        &self,
        offset_1: Offset,
        offset_2: Offset,
    ) -> Number;
    fn get_line_trigo(
        &self,
        angular: Number,
        radius_circle_begin: Number,
        radius_circle_end: Number,
    ) -> [Offset; 2];
    fn get_pos_trigo(&self, angular: Number, radius_circle: Number) -> Offset;
    fn get_triangle_path(
        &self,
        angular: Number,
        angular_pointer: Number,
        radius_circle_begin: Number,
        radius_circle_end: Number,
    ) -> [Offset; 3];
    fn get_fix_pos(&self, pos: Number) -> Number;
    fn get_angle_is_on_chart(&self, angle: Angle) -> bool;
    fn get_bodie_is_on_chart(&self, bodie: Bodies) -> bool;
    fn get_angle_longitude(&self, angle: Angle) -> Number;
    fn get_bodie_longitude(&self, bodie: Bodies) -> Number;
    fn get_angle_fix_longitude(&self, angle: Angle) -> Number;
    fn get_bodie_fix_longitude(&self, bodie: Bodies) -> Number;
    fn set_fix_compute(&mut self);
}

// Methods - Constructors

impl WorkingStorage {
    pub fn new(
        max_size: Number,
        house: HousesResult,
        object: Vec<Object>,
    ) -> WorkingStorage {
        let mut h: Vec<House> = Vec::new();
        for (i, res) in house.clone().cusps.iter().enumerate() {
            if i > 0 {
                // angle compatible Placidus
                let angle;
                angle = match i - 1 {
                    0 => Angle::Asc,
                    3 => Angle::Fc,
                    6 => Angle::Desc,
                    9 => Angle::Mc,
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
            object: object,
            temp_position_bodies: Vec::new(),
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
            let off_pos_asc: f32 = 360.0 - self.ws.house[0].longitude as f32;
            let mut pos = sign as f32 * 30.0 + &off_pos_asc;
            pos = self.ws.get_fix_pos(pos);
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
                pos = (sign as f32 * 30.0) + (j as f32 * 2.0) + &off_pos_asc;
                pos = self.ws.get_fix_pos(pos);
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
            let offset_house: f32 = 360.0 - self.ws.house[0].longitude as f32;
            let mut house_pos: f32 =
                offset_house + self.ws.house[i].longitude as f32;
            house_pos = self.ws.get_fix_pos(house_pos);
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

    fn draw_zodiac(&self, sign: Signs) -> SvgObject {
        // x = left-right
        // y = top-bottom
        let zodiac_ratio: Number = 10.0; // To do a const
        let zodiac_size =
            (((ZODIAC_SIZE * zodiac_ratio) / 100.0) * self.ws.max_size) / 100.0;

        let off_pos_asc: f32 = 360.0 - self.ws.house[0].longitude as f32;
        let mut pos =
            ((sign.clone() as u64 - 1) as f32 * 30.0) + 15.0 + &off_pos_asc;
        pos = self.ws.get_fix_pos(pos);
        let offset: Offset = self.ws.get_center_item(
            zodiac_size,
            self.ws
                .get_pos_trigo(pos, self.ws.get_radius_circle_zodiac()),
        );
        let svg = svg_draw_zodiac(sign);
        let svg_object: SvgObject = SvgObject {
            svg: svg.to_string(),
            size_x: zodiac_size,
            size_y: zodiac_size,
            pos_x: offset.x,
            pos_y: offset.y,
        };
        svg_object
    }

    /// draw_house
    /// numero 1 to 12
    fn draw_house(&self, numero: i16) -> SvgObject {
        let house_ratio: Number = 5.0; // To do a const
        let house_size =
            (((HOUSE_SIZE * house_ratio) / 100.0) * self.ws.max_size) / 100.0;
        let off_pos_asc: f32 = 360.0 - self.ws.house[0].longitude as f32;
        let pos_next: Number;
        if numero > 11 {
            pos_next = self.ws.house[0].longitude as f32 + &off_pos_asc;
        } else {
            pos_next =
                self.ws.house[numero as usize].longitude as f32 + &off_pos_asc;
        }
        let pos_now: Number =
            self.ws.house[numero as usize - 1].longitude as f32 + &off_pos_asc;
        let mut pos: Number;
        if pos_now > pos_next {
            pos = pos_now + ((pos_next - pos_now - 360.0) / 2.0);
        } else {
            pos = pos_now + ((pos_next - pos_now) / 2.0);
        }
        pos = self.ws.get_fix_pos(pos);
        let offset: Offset = self.ws.get_center_item(
            house_size,
            self.ws
                .get_pos_trigo(pos, self.ws.get_radius_circle_house()),
        );
        let svg = svg_draw_house(numero);
        let svg_object: SvgObject = SvgObject {
            svg: svg.to_string(),
            size_x: house_size,
            size_y: house_size,
            pos_x: offset.x,
            pos_y: offset.y,
        };
        svg_object
    }

    fn draw_angle(&self, angle: Angle) -> SvgObjectBodie {
        let angle_ratio: Number = 12.0; // To do a const
        let angle_size =
            (((ANGLE_SIZE * angle_ratio) / 100.0) * self.ws.max_size) / 100.0;
        let deg_ratio: Number = 6.0; // To do a const
        let deg_size =
            (((DEG_SIZE * deg_ratio) / 100.0) * self.ws.max_size) / 100.0;
        let min_ratio: Number = 5.5; // To do a const
        let min_size =
            (((MIN_SIZE * min_ratio) / 100.0) * self.ws.max_size) / 100.0;

        // Tested with placidus
        let svg_angle = match angle {
            Angle::Asc => svg_draw_asc(),
            Angle::Fc => svg_draw_fc(),
            Angle::Desc => svg_draw_desc(),
            Angle::Mc => svg_draw_mc(),
            _ => Document::new(),
        };

        let mut svg_deg = svg_draw_degre(0);
        let mut svg_min = svg_draw_minute(0);
        let pos: Number = self.ws.get_angle_longitude(angle.clone());
        let pos_fix: Number = self.ws.get_angle_fix_longitude(angle.clone());

        for h in self.ws.house.clone() {
            if h.angle.clone() == angle {
                svg_deg = svg_draw_degre(h.split.deg as i16);
                svg_min = svg_draw_minute(h.split.min as i16);
                break;
            }
        }

        let offset_angle: Offset = self.ws.get_center_item(
            angle_size,
            self.ws.get_pos_trigo(pos, self.ws.get_radius_circle(4).0),
        );
        let offset_deg: Offset = self.ws.get_center_item(
            deg_size,
            self.ws.get_pos_trigo(pos, self.ws.get_radius_circle(5).0),
        );
        let offset_min: Offset = self.ws.get_center_item(
            min_size,
            self.ws.get_pos_trigo(pos, self.ws.get_radius_circle(6).0),
        );

        // Trait
        let t_xy_begin: [Offset; 2] = self.ws.get_line_trigo(
            pos,
            self.ws.get_radius_circle(2).0,
            self.ws.get_radius_circle(7).0, // should be 3
        );
        let line_1 = Line::new()
            .set("x1", t_xy_begin[0].x)
            .set("y1", t_xy_begin[0].y)
            .set("x2", t_xy_begin[1].x)
            .set("y2", t_xy_begin[1].y)
            .set("stroke", "black")
            .set("stroke-width", 1);
        let t_xy_end: [Offset; 2] = self.ws.get_line_trigo(
            pos_fix,
            self.ws.get_radius_circle(7).0,
            self.ws.get_radius_circle(8).0,
        );
        let line_2 = Line::new()
            .set("x1", t_xy_begin[1].x)
            .set("y1", t_xy_begin[1].y)
            .set("x2", t_xy_end[1].x)
            .set("y2", t_xy_end[1].y)
            .set("stroke", "black")
            .set("stroke-width", 1);
        let document_trait = Document::new()
            .set(
                "viewBox",
                (0, 0, self.ws.max_size as i32, self.ws.max_size as i32),
            )
            .add(line_1)
            .add(line_2);

        let svg_object_bodie: SvgObjectBodie = SvgObjectBodie {
            svg: svg_angle.to_string(),
            size_x: angle_size,
            size_y: angle_size,
            pos_x: offset_angle.x,
            pos_y: offset_angle.y,
            deg_svg: svg_deg.to_string(),
            deg_size_x: deg_size,
            deg_size_y: deg_size,
            deg_pos_x: offset_deg.x,
            deg_pos_y: offset_deg.y,
            min_svg: svg_min.to_string(),
            min_size_x: min_size,
            min_size_y: min_size,
            min_pos_x: offset_min.x,
            min_pos_y: offset_min.y,
            trait_svg: document_trait.to_string(),
            trait_size_x: self.ws.max_size,
            trait_size_y: self.ws.max_size,
            trait_pos_x: 0.0,
            trait_pos_y: 0.0,
        };
        svg_object_bodie
    }

    fn draw_bodie(&self, bodie: Bodies) -> SvgObjectBodie {
        let planet_ratio: Number = 12.0; // To do a const
        let planet_size =
            (((BODIE_SIZE * planet_ratio) / 100.0) * self.ws.max_size) / 100.0;
        let deg_ratio: Number = 6.0; // To do a const
        let deg_size =
            (((DEG_SIZE * deg_ratio) / 100.0) * self.ws.max_size) / 100.0;
        let min_ratio: Number = 5.5; // To do a const
        let min_size =
            (((MIN_SIZE * min_ratio) / 100.0) * self.ws.max_size) / 100.0;

        let mut sw_retrograde = false;
        for o in self.ws.object.clone() {
            if o.object_enum == bodie.clone() {
                if o.object_pos == ObjectPos::Retrograde {
                    sw_retrograde = true;
                }
                break;
            }
        }

        let svg_planet = svg_draw_bodie(bodie.clone(), sw_retrograde);
        let mut svg_deg = svg_draw_degre(0);
        let mut svg_min = svg_draw_minute(0);
        let pos: Number = self.ws.get_bodie_longitude(bodie.clone());
        let pos_fix: Number = self.ws.get_bodie_fix_longitude(bodie.clone());
        for b in self.ws.object.clone() {
            if b.object_enum.clone() == bodie {
                svg_deg = svg_draw_degre(b.split.deg as i16);
                svg_min = svg_draw_minute(b.split.min as i16);
                break;
            }
        }
        let offset_planet: Offset = self.ws.get_center_item(
            planet_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(4).0),
        );
        let offset_deg: Offset = self.ws.get_center_item(
            deg_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(5).0),
        );
        let offset_min: Offset = self.ws.get_center_item(
            min_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(6).0),
        );

        // Trait
        let t_xy_begin: [Offset; 2] = self.ws.get_line_trigo(
            pos,
            self.ws.get_radius_circle(2).0,
            self.ws.get_radius_circle(7).0, // should be 3
        );
        let line_1 = Line::new()
            .set("x1", t_xy_begin[0].x)
            .set("y1", t_xy_begin[0].y)
            .set("x2", t_xy_begin[1].x)
            .set("y2", t_xy_begin[1].y)
            .set("stroke", "black")
            .set("stroke-width", 1);
        let t_xy_end: [Offset; 2] = self.ws.get_line_trigo(
            pos_fix,
            self.ws.get_radius_circle(7).0,
            self.ws.get_radius_circle(8).0,
        );
        let line_2 = Line::new()
            .set("x1", t_xy_begin[1].x)
            .set("y1", t_xy_begin[1].y)
            .set("x2", t_xy_end[1].x)
            .set("y2", t_xy_end[1].y)
            .set("stroke", "black")
            .set("stroke-width", 1);
        let document_trait = Document::new()
            .set(
                "viewBox",
                (0, 0, self.ws.max_size as i32, self.ws.max_size as i32),
            )
            .add(line_1)
            .add(line_2);

        let svg_object_bodie: SvgObjectBodie = SvgObjectBodie {
            svg: svg_planet.to_string(),
            size_x: planet_size,
            size_y: planet_size,
            pos_x: offset_planet.x,
            pos_y: offset_planet.y,
            deg_svg: svg_deg.to_string(),
            deg_size_x: deg_size,
            deg_size_y: deg_size,
            deg_pos_x: offset_deg.x,
            deg_pos_y: offset_deg.y,
            min_svg: svg_min.to_string(),
            min_size_x: min_size,
            min_size_y: min_size,
            min_pos_x: offset_min.x,
            min_pos_y: offset_min.y,
            trait_svg: document_trait.to_string(),
            trait_size_x: self.ws.max_size,
            trait_size_y: self.ws.max_size,
            trait_pos_x: 0.0,
            trait_pos_y: 0.0,
        };
        svg_object_bodie
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

    fn get_radius_circle_zodiac(&self) -> Number {
        let div_trait_big = 0.2;
        (self.get_radius_total()
            * (((CIRCLE_SIZE[1].0 - CIRCLE_SIZE[0].0) / (2.0 + div_trait_big))
                + CIRCLE_SIZE[0].0))
            / 100.0
    }

    fn get_radius_circle_house(&self) -> Number {
        (self.get_radius_total()
            * (((CIRCLE_SIZE[2].0 - CIRCLE_SIZE[1].0) / 2.0)
                + CIRCLE_SIZE[1].0))
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

    fn get_center_item(&self, size: Number, offset: Offset) -> Offset {
        Offset {
            x: offset.x - (size / 2.0),
            y: offset.y - (size / 2.0),
        }
    }

    fn get_distance_between_2_offset(
        &self,
        offset_1: Offset,
        offset_2: Offset,
    ) -> Number {
        let a = offset_1.x - offset_2.x;
        let b = offset_2.y - offset_2.y;
        ((a * a) + (b * b)).sqrt()
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
                * radius_circle_begin as f32;
        let dy1: Number = self.get_center().x
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_end as f32;
        let dy2: Number = self.get_center().y
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
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
        angular1 = self.get_fix_pos(angular1);
        let mut angular2 = angular as f32 + angular_pointer as f32;
        angular2 = self.get_fix_pos(angular2);
        let dx1: Number = self.get_center().x
            + (angular1 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_begin as f32;
        let dy1: Number = self.get_center().y
            + (angular1 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * radius_circle_begin as f32;
        let dx2: Number = self.get_center().x
            + (angular2 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_begin as f32;
        let dy2: Number = self.get_center().y
            + (angular2 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * radius_circle_begin as f32;
        let dx3: Number = self.get_center().x
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).cos()
                * -1.0
                * radius_circle_end as f32;
        let dy3: Number = self.get_center().y
            + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI).sin()
                * radius_circle_end as f32;
        [
            Offset { x: dx1, y: dy1 },
            Offset { x: dx2, y: dy2 },
            Offset { x: dx3, y: dy3 },
        ]
    }

    fn get_pos_trigo(&self, angular: Number, radius_circle: Number) -> Offset {
        Offset {
            x: self.get_center().x
                + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI)
                    .cos()
                    * -1.0
                    * radius_circle as f32,
            y: self.get_center().y
                + (angular as f32 / CIRCLE as f32 * 2.0 * f32::consts::PI)
                    .sin()
                    * radius_circle as f32,
        }
    }

    fn get_fix_pos(&self, mut pos: Number) -> Number {
        let mut done = false;
        while !done {
            if pos >= 360.0 {
                pos = pos - 360.0;
            }
            if pos >= 360.0 {
                //
            } else {
                done = true;
            }
        }
        pos
    }

    /// Warning you can't remove Angle::Asc of if without rewrite
    /// the get_bodie_fix_longitude function
    fn get_angle_is_on_chart(&self, angle: Angle) -> bool {
        if angle == Angle::Asc || angle == Angle::Mc {
            true
        } else {
            false
        }
    }
    fn get_bodie_is_on_chart(&self, bodie: Bodies) -> bool {
        if bodie == Bodies::Sun
            || bodie == Bodies::Moon
            || bodie == Bodies::Mars
            || bodie == Bodies::Mercury
            || bodie == Bodies::Venus
            || bodie == Bodies::Jupiter
            || bodie == Bodies::Saturn
            || bodie == Bodies::Uranus
            || bodie == Bodies::Neptune
            || bodie == Bodies::Pluto
        {
            true
        } else {
            false
        }
    }

    fn get_angle_longitude(&self, angle: Angle) -> Number {
        let mut pos: Number = 0.0;
        for h in self.house.clone() {
            if h.angle.clone() == angle {
                pos =
                    360.0 - self.house[0].longitude as f32 + h.longitude as f32;
                break;
            }
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    fn get_bodie_longitude(&self, bodie: Bodies) -> Number {
        let mut pos: Number = 0.0;
        for b in self.object.clone() {
            if b.object_enum.clone() == bodie {
                pos =
                    360.0 - self.house[0].longitude as f32 + b.longitude as f32;
                break;
            }
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    fn get_angle_fix_longitude(&self, angle: Angle) -> Number {
        let mut pos: Number = self.get_angle_longitude(angle.clone());
        if angle == Angle::Mc {
            pos = pos - BODIE_DISTANCE;
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    fn get_bodie_fix_longitude(&self, bodie: Bodies) -> Number {
        let mut pos: Number = self.get_bodie_longitude(bodie.clone());
        if bodie == Bodies::Sun {
            pos = pos + BODIE_DISTANCE;
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    fn set_fix_compute(&mut self) {
        let mut temp_no_order: Vec<TempPositionBodies> = Vec::new();
        let mut i: i16 = 0;
        for a in Angle::iter() {
            if self.get_angle_is_on_chart(a.clone()) {
                if a.clone() == Angle::Asc {
                    i = i + 1;
                    let longitude = self.get_angle_longitude(a.clone());
                    temp_no_order.push(TempPositionBodies {
                        init_index: i,
                        index: 1,
                        sw_reserve: true,
                        sw_bodie: false,
                        bodie_enum: Bodies::EclNut, // -1 Nothing, this variable
                        // is not used
                        longitude: longitude,
                        space_left: 0.0,
                        space_right: 0.0,
                        longitude_fix: 0.0,
                    });
                } else {
                    i = i + 1;
                    let longitude = self.get_angle_longitude(a.clone());
                    temp_no_order.push(TempPositionBodies {
                        init_index: i,
                        index: 0,
                        sw_reserve: false,
                        sw_bodie: false,
                        bodie_enum: Bodies::EclNut, // -1 Nothing, this variable
                        // is not used
                        longitude: longitude,
                        space_left: 0.0,
                        space_right: 0.0,
                        longitude_fix: 0.0,
                    });
                }
            }
        }
        for b in Bodies::iter() {
            if self.get_bodie_is_on_chart(b) {
                i = i + 1;
                let longitude = self.get_bodie_longitude(b);
                temp_no_order.push(TempPositionBodies {
                    init_index: i,
                    index: 0,
                    sw_reserve: false,
                    sw_bodie: true,
                    bodie_enum: b,
                    longitude: longitude,
                    space_left: 0.0,
                    space_right: 0.0,
                    longitude_fix: 0.0,
                });
            }
        }
        // Order by pos
        let mut done = false;
        let mut old_lng = 0.0; // Value ASC forced
        let mut next_lng;
        let mut old_i;
        let mut next_index = 1;
        let mut temp_order: Vec<TempPositionBodies> = Vec::new();

        let mut done_main = false;
        while !done_main {
            old_i = 0;
            next_lng = 0.0;
            for t in temp_no_order.clone() {
                if t.index.clone() == 0 {
                    if done {
                        if t.longitude < next_lng && t.longitude >= old_lng {
                            // not egal here
                            next_lng = t.longitude;
                            old_i = t.init_index;
                        }
                    } else {
                        if t.longitude >= old_lng {
                            done = true;
                            next_lng = t.longitude;
                            old_i = t.init_index;
                        }
                    }
                }
            }
            // Next - Detect of Not
            done = false;
            if old_i > 0 {
                old_lng = next_lng;
                next_index = next_index + 1;
                temp_order.clear();
                for t in temp_no_order.clone() {
                    if t.init_index == old_i {
                        temp_order.push(TempPositionBodies {
                            init_index: t.init_index,
                            index: next_index,
                            sw_reserve: t.sw_reserve,
                            sw_bodie: t.sw_bodie,
                            bodie_enum: t.bodie_enum,
                            longitude: t.longitude,
                            space_left: t.space_left,
                            space_right: t.space_right,
                            longitude_fix: t.longitude_fix,
                        });
                    } else {
                        temp_order.push(t.clone());
                    }
                }
                temp_no_order.clear();
                for t in temp_order.clone() {
                    temp_no_order.push(t);
                }
            } else {
                // Nothing found
                done_main = true;
            }
        }
        // Next order
        temp_order.clear();
        for t in temp_no_order.clone() {
            println!(
                "i_index: {} index: {} bodie: {} longitude: {}",
                t.init_index, t.index, t.bodie_enum, t.longitude
            );
        }
        self.temp_position_bodies = temp_no_order;
    }
}

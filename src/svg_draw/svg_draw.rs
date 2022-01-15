extern crate libswe_sys;
extern crate strum;
//use strum::AsStaticRef;
use crate::svg_draw::angles::ANGLE_SIZE;
use crate::svg_draw::bodies::BODIE_SIZE;
use crate::svg_draw::houses::HOUSE_SIZE;
use crate::svg_draw::numbers::{DEG_SIZE, MIN_SIZE};
use crate::svg_draw::zodiacs::ZODIAC_SIZE;
use libswe_sys::sweconst::{
    Angle, Aspects, Bodies, House, Language, Object, ObjectPos, Signs, Theme,
};
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

/// Natal circle
/// (Position, Bool visible)
const CIRCLE_SIZE_NATAL: [(Number, bool); 9] = [
    (35.0, true),  // 0
    (62.0, true),  // 1
    (67.0, true),  // 2
    (77.0, false), // 3
    (80.0, false), // 4
    (89.0, false), // 5
    (96.0, false), // 6
    (69.0, false), // 7 between 2 and 3
    (71.0, false), // 8 correction planet between 2 and 3
];

/// Transit circle
/// (Position, Bool visible)
const CIRCLE_SIZE_TRANSIT: [(Number, bool); 12] = [
    (45.0, true),  // 0 CIRCLE ASPECT
    (59.0, true),  // 1 CIRCLE TRANSIT
    (75.0, true),  // 2 CIRCLE ZODIAC END
    (80.0, true),  // 3 CIRCLE HOUSE
    (92.0, false), // 4 CIRCLE INVISIBLE -
    (92.0, false), // 5 CIRCLE INVISIBLE PLANET
    //    (0.0, false), // 5
    (0.0, false),  // 6
    (82.0, false), // 7 between 2 and 3
    (85.0, false), // 8 correction planet between 2 and 3
    (49.0, false), // 9 Planet pos transit
    (57.5, false), // 10 - 7 transit
    (54.5, false), // 11 - 8 transit
];

// For draw min/sec (color for angle) / Bodies::EclNut = -1 not used
const ANGLE_BODIE: Bodies = Bodies::EclNut;

const BODIE_DISTANCE_NATAL: Number = 8.0;
const BODIE_DISTANCE_OFFSET_NATAL: Number = 0.5;
const BODIE_DISTANCE_TRANSIT: Number = 13.5;
const BODIE_DISTANCE_OFFSET_TRANSIT: Number = 0.5;

// Working Storage - Enums
// #[derive(Debug, Clone, PartialEq, Display)]
pub enum LargerDrawLine {
    Small,
    Large,
}

/// Working storage for natal chart
/// Polymorph on traits "Draw" and "CalcDraw"
#[derive(Debug, Clone)]
pub struct WorkingStoragePolyMorphNatal {
    pub max_size: Number,
    pub theme: Theme,
    pub lang: Language,
    pub house: Vec<House>,
    pub object: Vec<Object>,
    pub temp_position_bodies: Vec<TempPositionBodies>,
}

/// Working storage for transit chart
/// Polymorph on traits "Draw" and "CalcDraw"
#[derive(Debug, Clone)]
pub struct WorkingStoragePolyMorphTransit {
    pub max_size: Number,
    pub theme: Theme,
    pub lang: Language,
    pub house: Vec<House>,
    pub object_natal: Vec<Object>,
    pub object_transit: Vec<Object>,
    pub temp_position_bodies_natal: Vec<TempPositionBodies>,
    pub temp_position_bodies_transit: Vec<TempPositionBodies>,
}

/// Working storage for natal chart
/// Polymorph on trait "Draw"
#[derive(Debug, Clone)]
pub struct WorkingStorageDrawPolyMorphNatal {
    pub ws: WorkingStoragePolyMorphNatal,
}

/// Working storage for transit chart
/// Polymorph on traiit "Draw"
#[derive(Debug, Clone)]
pub struct WorkingStorageDrawPolyMorphTransit {
    pub ws: WorkingStoragePolyMorphTransit,
}

/// Offset position x and y
/// Type "Number" for working with "svg" crate
#[derive(Debug, Clone)]
pub struct Offset {
    pub x: Number,
    pub y: Number,
}

/// Object svg
/// Type "Number" for working with "svg" crate
#[derive(Debug, Clone)]
pub struct SvgObject {
    pub svg: String,
    pub size_x: Number,
    pub size_y: Number,
    pub pos_x: Number,
    pub pos_y: Number,
}

/// Bodie object svg
/// Type "Number" for working with "svg" crate
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

/// Temporary struct for compute offset between object/angles
#[derive(Debug, Clone, Copy)]
pub struct TempPositionBodies {
    pub init_index: i16,
    pub index: i16,
    pub sw_reserve: bool,
    pub sw_bodie: bool,
    pub bodie_enum: Bodies,
    pub angle_enum: Angle,
    pub longitude: Number,
    pub space_left: Number,
    pub space_right: Number,
    pub fix: Number,
    pub longitude_fix: Number,
}

pub trait Draw {
    fn draw_base(&self) -> Document;
    fn draw_zodiac(&self, sign: Signs) -> SvgObject;
    fn draw_house(&self, numero: i16) -> SvgObject;
    fn draw_angle(&self, angle: Angle) -> SvgObjectBodie;
    fn draw_bodie(&self, bodie: Bodies, sw_transit: bool) -> SvgObjectBodie;
    fn draw_aspect(
        &self,
        lng_1: Number,
        lng_2: Number,
        aspect: Aspects,
    ) -> SvgObject;
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
    fn get_bodie_longitude(&self, bodie: Bodies, sw_transit: bool) -> Number;
    fn get_angle_fix_longitude(&self, angle: Angle) -> Number;
    fn get_bodie_fix_longitude(
        &self,
        bodie: Bodies,
        sw_transit: bool,
    ) -> Number;
    fn set_fix_compute(&mut self, sw_transit: bool);
    fn get_closest_distance(&self, angle1: Number, angle2: Number) -> Number;
    fn get_znorm(&self, angle: Number) -> Number;
}

impl WorkingStoragePolyMorphNatal {
    pub fn new(
        max_size: Number,
        theme: Theme,
        lang: Language,
        house: HousesResult,
        object: Vec<Object>,
    ) -> WorkingStoragePolyMorphNatal {
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
        WorkingStoragePolyMorphNatal {
            max_size: max_size,
            theme: theme,
            lang: lang,
            house: h,
            object: object,
            temp_position_bodies: Vec::new(),
        }
    }
}

impl WorkingStoragePolyMorphTransit {
    pub fn new(
        max_size: Number,
        theme: Theme,
        lang: Language,
        house: HousesResult,
        object_natal: Vec<Object>,
        object_transit: Vec<Object>,
    ) -> WorkingStoragePolyMorphTransit {
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
        WorkingStoragePolyMorphTransit {
            max_size: max_size,
            theme: theme,
            lang: lang,
            house: h,
            object_natal: object_natal,
            object_transit: object_transit,
            temp_position_bodies_natal: Vec::new(),
            temp_position_bodies_transit: Vec::new(),
        }
    }
}

impl WorkingStorageDrawPolyMorphNatal {
    pub fn new(
        ws: WorkingStoragePolyMorphNatal,
    ) -> WorkingStorageDrawPolyMorphNatal {
        WorkingStorageDrawPolyMorphNatal { ws: ws }
    }
}

impl WorkingStorageDrawPolyMorphTransit {
    pub fn new(
        ws: WorkingStoragePolyMorphTransit,
    ) -> WorkingStorageDrawPolyMorphTransit {
        WorkingStorageDrawPolyMorphTransit { ws: ws }
    }
}

impl Draw for WorkingStorageDrawPolyMorphNatal {
    fn draw_base(&self) -> Document {
        let calc_draw = self.ws.clone();
        let center: (Number, Number) = (
            calc_draw.get_radius_total() as Number,
            calc_draw.get_radius_total() as Number,
        );

        let mut circle = Vec::new();
        for (i, ele) in CIRCLE_SIZE_NATAL.iter().enumerate() {
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
        let svg = self.zodiacs_draw(sign);
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
        let svg = self.houses_draw(numero);
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
            Angle::Asc => self.angles_draw_asc(),
            Angle::Fc => self.angles_draw_fc(),
            Angle::Desc => self.angles_draw_desc(),
            Angle::Mc => self.angles_draw_mc(),
            _ => Document::new(),
        };

        let mut svg_deg = self.numbers_draw_degre(0, ANGLE_BODIE);
        let mut svg_min = self.numbers_draw_minute(0, ANGLE_BODIE);
        let pos: Number = self.ws.get_angle_longitude(angle.clone());
        let pos_fix: Number = self.ws.get_angle_fix_longitude(angle.clone());

        for h in self.ws.house.clone() {
            if h.angle.clone() == angle {
                svg_deg =
                    self.numbers_draw_degre(h.split.deg as i16, ANGLE_BODIE);
                svg_min =
                    self.numbers_draw_minute(h.split.min as i16, ANGLE_BODIE);
                break;
            }
        }

        let offset_angle: Offset = self.ws.get_center_item(
            angle_size,
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
        let color: String =
            format!("#{:06X}", ANGLE_BODIE.object_color(self.ws.theme) as i32);
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
            .set("stroke", color.clone())
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
            .set("stroke", color)
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

    /// Draw svg bodie into struct SvgObjectBodie
    /// - bodies is planet/fiction/asteroid
    /// - _sw_transit is not used here
    fn draw_bodie(&self, bodie: Bodies, _sw_transit: bool) -> SvgObjectBodie {
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

        let svg_planet = self.bodies_draw(bodie.clone(), sw_retrograde);
        let mut svg_deg = self.numbers_draw_degre(0, bodie.clone());
        let mut svg_min = self.numbers_draw_minute(0, bodie.clone());
        let pos: Number = self.ws.get_bodie_longitude(bodie.clone(), false);
        let pos_fix: Number =
            self.ws.get_bodie_fix_longitude(bodie.clone(), false);
        for b in self.ws.object.clone() {
            if b.object_enum.clone() == bodie {
                svg_deg =
                    self.numbers_draw_degre(b.split.deg as i16, bodie.clone());
                svg_min =
                    self.numbers_draw_minute(b.split.min as i16, bodie.clone());
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
        let color: String =
            format!("#{:06X}", bodie.object_color(self.ws.theme) as i32);
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
            .set("stroke", color.clone())
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
            .set("stroke", color)
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

    fn draw_aspect(
        &self,
        lng_1: Number,
        lng_2: Number,
        aspect: Aspects,
    ) -> SvgObject {
        let pos1 = self.ws.get_pos_trigo(lng_1, self.ws.get_radius_circle(0).0);
        let pos2 = self.ws.get_pos_trigo(lng_2, self.ws.get_radius_circle(0).0);
        let line;
        match aspect {
            Aspects::Conjunction => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "yellow")
                    .set("stroke-width", 2);
            },
            Aspects::Opposition => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke-dasharray", "4, 1, 2")
                    .set("stroke", "orange")
                    .set("stroke-width", 1);
            },
            Aspects::Trine => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "blue")
                    .set("stroke-width", 1);
            },
            Aspects::Square => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "red")
                    .set("stroke-width", 1);
            },
            Aspects::Sextile => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "green")
                    .set("stroke-width", 1);
            },
            Aspects::Inconjunction => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "violet")
                    .set("stroke-width", 1);
            },
            Aspects::Sesquisquare => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "brown")
                    .set("stroke-width", 1);
            },
            Aspects::Semisquare => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "brown")
                    .set("stroke-width", 1);
            },
            Aspects::Semisextile => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "violet")
                    .set("stroke-width", 1);
            },
        }
        let document = Document::new()
            .set(
                "viewBox",
                (0, 0, self.ws.max_size as i32, self.ws.max_size as i32),
            )
            .add(line);
        let res = SvgObject {
            svg: document.to_string(),
            size_x: self.ws.max_size,
            size_y: self.ws.max_size,
            pos_x: 0.0,
            pos_y: 0.0,
        };
        res
    }
}

impl Draw for WorkingStorageDrawPolyMorphTransit {
    fn draw_base(&self) -> Document {
        let calc_draw = self.ws.clone();
        let center: (Number, Number) = (
            calc_draw.get_radius_total() as Number,
            calc_draw.get_radius_total() as Number,
        );

        let mut circle = Vec::new();
        for (i, ele) in CIRCLE_SIZE_TRANSIT.iter().enumerate() {
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
                self.ws.get_radius_circle(2).0,
                self.ws.get_radius_circle(1).0,
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
                    self.ws.get_radius_circle(2).0,
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
                    self.ws.get_radius_circle(3).0,
                    self.ws.get_radius_circle(2).0,
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
                    self.ws.get_radius_circle(3).0,
                    self.ws.get_radius_circle(2).0,
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
            .add(circle[3].clone())
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
        let svg = self.zodiacs_draw(sign);
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
        let svg = self.houses_draw(numero);
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
            Angle::Asc => self.angles_draw_asc(),
            Angle::Fc => self.angles_draw_fc(),
            Angle::Desc => self.angles_draw_desc(),
            Angle::Mc => self.angles_draw_mc(),
            _ => Document::new(),
        };

        let mut svg_deg = self.numbers_draw_degre(0, ANGLE_BODIE);
        let mut svg_min = self.numbers_draw_minute(0, ANGLE_BODIE);
        let pos: Number = self.ws.get_angle_longitude(angle.clone());
        let pos_fix: Number = self.ws.get_angle_fix_longitude(angle.clone());

        for h in self.ws.house.clone() {
            if h.angle.clone() == angle {
                svg_deg =
                    self.numbers_draw_degre(h.split.deg as i16, ANGLE_BODIE);
                svg_min =
                    self.numbers_draw_minute(h.split.min as i16, ANGLE_BODIE);
                break;
            }
        }

        let offset_angle: Offset = self.ws.get_center_item(
            angle_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(5).0),
        );
        let offset_deg: Offset = self.ws.get_center_item(
            deg_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(6).0),
        );
        let offset_min: Offset = self.ws.get_center_item(
            min_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(6).0),
        );

        // Trait
        let color: String =
            format!("#{:06X}", ANGLE_BODIE.object_color(self.ws.theme) as i32);
        let t_xy_begin: [Offset; 2] = self.ws.get_line_trigo(
            pos,
            self.ws.get_radius_circle(3).0,
            self.ws.get_radius_circle(7).0, // should be 3
        );
        let line_1 = Line::new()
            .set("x1", t_xy_begin[0].x)
            .set("y1", t_xy_begin[0].y)
            .set("x2", t_xy_begin[1].x)
            .set("y2", t_xy_begin[1].y)
            .set("stroke", color.clone())
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
            .set("stroke", color)
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

    fn draw_bodie(&self, bodie: Bodies, sw_transit: bool) -> SvgObjectBodie {
        let planet_ratio: Number = if sw_transit {
            6.0 // To do a const
        } else {
            12.0 // To do a const
        };
        let planet_size =
            (((BODIE_SIZE * planet_ratio) / 100.0) * self.ws.max_size) / 100.0;
        let deg_ratio: Number = 6.0; // To do a const
        let deg_size =
            (((DEG_SIZE * deg_ratio) / 100.0) * self.ws.max_size) / 100.0;
        let min_ratio: Number = 5.5; // To do a const
        let min_size =
            (((MIN_SIZE * min_ratio) / 100.0) * self.ws.max_size) / 100.0;

        let mut sw_retrograde = false;
        if sw_transit {
            for o in self.ws.object_transit.clone() {
                if o.object_enum == bodie.clone() {
                    if o.object_pos == ObjectPos::Retrograde {
                        sw_retrograde = true;
                    }
                    break;
                }
            }
        } else {
            for o in self.ws.object_natal.clone() {
                if o.object_enum == bodie.clone() {
                    if o.object_pos == ObjectPos::Retrograde {
                        sw_retrograde = true;
                    }
                    break;
                }
            }
        }

        let svg_planet = self.bodies_draw(bodie.clone(), sw_retrograde);
        let mut svg_deg = self.numbers_draw_degre(0, bodie.clone());
        let mut svg_min = self.numbers_draw_minute(0, bodie.clone());
        let pos: Number =
            self.ws.get_bodie_longitude(bodie.clone(), sw_transit);
        let pos_fix: Number =
            self.ws.get_bodie_fix_longitude(bodie.clone(), sw_transit);
        if sw_transit {
            for b in self.ws.object_transit.clone() {
                if b.object_enum.clone() == bodie {
                    svg_deg = self
                        .numbers_draw_degre(b.split.deg as i16, bodie.clone());
                    svg_min = self
                        .numbers_draw_minute(b.split.min as i16, bodie.clone());
                    break;
                }
            }
        } else {
            for b in self.ws.object_natal.clone() {
                if b.object_enum.clone() == bodie {
                    svg_deg = self
                        .numbers_draw_degre(b.split.deg as i16, bodie.clone());
                    svg_min = self
                        .numbers_draw_minute(b.split.min as i16, bodie.clone());
                    break;
                }
            }
        }

        let offset_planet: Offset = if sw_transit {
            self.ws.get_center_item(
                planet_size,
                self.ws
                    .get_pos_trigo(pos_fix, self.ws.get_radius_circle(9).0),
            )
        } else {
            self.ws.get_center_item(
                planet_size,
                self.ws
                    .get_pos_trigo(pos_fix, self.ws.get_radius_circle(5).0),
            )
        };
        let offset_deg: Offset = self.ws.get_center_item(
            deg_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(6).0),
        );
        let offset_min: Offset = self.ws.get_center_item(
            min_size,
            self.ws
                .get_pos_trigo(pos_fix, self.ws.get_radius_circle(6).0),
        );

        // Trait
        let color: String =
            format!("#{:06X}", bodie.object_color(self.ws.theme) as i32);
        let t_xy_begin: [Offset; 2] = if sw_transit {
            self.ws.get_line_trigo(
                pos,
                self.ws.get_radius_circle(1).0,
                self.ws.get_radius_circle(10).0, // should be 3
            )
        } else {
            self.ws.get_line_trigo(
                pos,
                self.ws.get_radius_circle(3).0,
                self.ws.get_radius_circle(7).0, // should be 3
            )
        };
        let line_1 = Line::new()
            .set("x1", t_xy_begin[0].x)
            .set("y1", t_xy_begin[0].y)
            .set("x2", t_xy_begin[1].x)
            .set("y2", t_xy_begin[1].y)
            .set("stroke", color.clone())
            .set("stroke-width", 1);
        let t_xy_end: [Offset; 2] = if sw_transit {
            self.ws.get_line_trigo(
                pos_fix,
                self.ws.get_radius_circle(10).0,
                self.ws.get_radius_circle(11).0,
            )
        } else {
            self.ws.get_line_trigo(
                pos_fix,
                self.ws.get_radius_circle(7).0,
                self.ws.get_radius_circle(8).0,
            )
        };
        let line_2 = Line::new()
            .set("x1", t_xy_begin[1].x)
            .set("y1", t_xy_begin[1].y)
            .set("x2", t_xy_end[1].x)
            .set("y2", t_xy_end[1].y)
            .set("stroke", color)
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

    fn draw_aspect(
        &self,
        lng_1: Number,
        lng_2: Number,
        aspect: Aspects,
    ) -> SvgObject {
        let pos1 = self.ws.get_pos_trigo(lng_1, self.ws.get_radius_circle(0).0);
        let pos2 = self.ws.get_pos_trigo(lng_2, self.ws.get_radius_circle(0).0);
        let line;
        match aspect {
            Aspects::Conjunction => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "yellow")
                    .set("stroke-width", 2);
            },
            Aspects::Opposition => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke-dasharray", "4, 1, 2")
                    .set("stroke", "orange")
                    .set("stroke-width", 1);
            },
            Aspects::Trine => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "blue")
                    .set("stroke-width", 1);
            },
            Aspects::Square => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "red")
                    .set("stroke-width", 1);
            },
            Aspects::Sextile => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "green")
                    .set("stroke-width", 1);
            },
            Aspects::Inconjunction => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "violet")
                    .set("stroke-width", 1);
            },
            Aspects::Sesquisquare => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "brown")
                    .set("stroke-width", 1);
            },
            Aspects::Semisquare => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "brown")
                    .set("stroke-width", 1);
            },
            Aspects::Semisextile => {
                line = Line::new()
                    .set("x1", pos1.x)
                    .set("y1", pos1.y)
                    .set("x2", pos2.x)
                    .set("y2", pos2.y)
                    .set("stroke", "violet")
                    .set("stroke-width", 1);
            },
        }
        let document = Document::new()
            .set(
                "viewBox",
                (0, 0, self.ws.max_size as i32, self.ws.max_size as i32),
            )
            .add(line);
        let res = SvgObject {
            svg: document.to_string(),
            size_x: self.ws.max_size,
            size_y: self.ws.max_size,
            pos_x: 0.0,
            pos_y: 0.0,
        };
        res
    }
}

impl CalcDraw for WorkingStoragePolyMorphNatal {
    fn get_radius_total(&self) -> Number {
        self.max_size / 2.0
    }

    fn get_radius_circle(&self, occurs: usize) -> (Number, bool) {
        if occurs > CIRCLE_SIZE_NATAL.len() {
            panic!("Out of range in circle occurs: {}", occurs);
        }
        (
            (self.get_radius_total() * CIRCLE_SIZE_NATAL[occurs].0) / 100.0,
            CIRCLE_SIZE_NATAL[occurs].1,
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
            * (((CIRCLE_SIZE_NATAL[1].0 - CIRCLE_SIZE_NATAL[0].0) / size)
                + CIRCLE_SIZE_NATAL[0].0)
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
            * (((CIRCLE_SIZE_NATAL[2].0 - CIRCLE_SIZE_NATAL[1].0)
                / div_trait_pointer)
                - CIRCLE_SIZE_NATAL[2].0))
            / 100.0
    }

    /// Top of pointer
    ///     HERE
    ///     /  \
    ///      ||
    ///      ||
    fn get_radius_rules_inside_circle_house_for_pointer_top(&self) -> Number {
        (self.get_radius_total()
            * ((CIRCLE_SIZE_NATAL[2].0 - CIRCLE_SIZE_NATAL[1].0)
                - CIRCLE_SIZE_NATAL[2].0))
            / 100.0
    }

    fn get_radius_circle_zodiac(&self) -> Number {
        let div_trait_big = 0.2;
        (self.get_radius_total()
            * (((CIRCLE_SIZE_NATAL[1].0 - CIRCLE_SIZE_NATAL[0].0)
                / (2.0 + div_trait_big))
                + CIRCLE_SIZE_NATAL[0].0))
            / 100.0
    }

    fn get_radius_circle_house(&self) -> Number {
        (self.get_radius_total()
            * (((CIRCLE_SIZE_NATAL[2].0 - CIRCLE_SIZE_NATAL[1].0) / 2.0)
                + CIRCLE_SIZE_NATAL[1].0))
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
            || bodie == Bodies::TrueNode // North Node true
        //    || bodie == Bodies::Chiron
       //     || bodie == Bodies::MeanApog // AsteroidLilith != Dark moon mean
           || bodie == Bodies::OscuApog // AsteroidLilith != Dark moon true 13-17
       //     || bodie == Bodies::Ceres // 13-17
       //     || bodie == Bodies::SouthNode
          //  || bodie == Bodies::FortunaPart
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

    /// Get the longitude of bodie in "Number" (type for "svg" crate)
    /// - bodie is the planet/fiction/asteroid
    /// - sw_transit is not used here
    fn get_bodie_longitude(&self, bodie: Bodies, _sw_transit: bool) -> Number {
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
        if angle != Angle::Nothing {
            let arr = self.temp_position_bodies.clone();
            for a in arr {
                if a.angle_enum == angle {
                    pos = a.longitude_fix;
                }
            }
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    /// Get the fixed longitude of a bodie
    /// - bodie is the planet/fiction/asteroid
    /// - sw_transit is not used here
    fn get_bodie_fix_longitude(
        &self,
        bodie: Bodies,
        _sw_transit: bool,
    ) -> Number {
        let mut pos: Number = self.get_bodie_longitude(bodie.clone(), false);
        if bodie != Bodies::EclNut {
            let arr = self.temp_position_bodies.clone();
            for a in arr {
                if a.bodie_enum == bodie {
                    pos = a.longitude_fix;
                }
            }
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    /// Set value with a compute of offset between Angle/Bodies(Planet/Fiction/
    /// Asteroid)
    /// - sw_transit is not used here
    fn set_fix_compute(&mut self, _sw_transit: bool) {
        let mut temp_no_order: Vec<TempPositionBodies> = Vec::new();
        let mut i: i16 = 0;
        //println!("A");
        for a in Angle::iter() {
            if self.get_angle_is_on_chart(a) {
                i = i + 1;
                let longitude = self.get_angle_longitude(a);
                temp_no_order.push(TempPositionBodies {
                    init_index: i,
                    index: 0,
                    sw_reserve: false,
                    sw_bodie: false,
                    bodie_enum: Bodies::EclNut, // -1 Nothing, this variable
                    angle_enum: a,
                    longitude: longitude,
                    space_left: 0.0,
                    space_right: 0.0,
                    fix: 0.0,
                    longitude_fix: 0.0,
                });
            }
        }
        //println!("B");
        for b in Bodies::iter() {
            if self.get_bodie_is_on_chart(b) {
                i = i + 1;
                let longitude = self.get_bodie_longitude(b, false);
                temp_no_order.push(TempPositionBodies {
                    init_index: i,
                    index: 0,
                    sw_reserve: false,
                    sw_bodie: true,
                    bodie_enum: b,
                    angle_enum: Angle::Nothing,
                    longitude: longitude,
                    space_left: 0.0,
                    space_right: 0.0,
                    fix: 0.0,
                    longitude_fix: 0.0,
                });
            }
        }
        //println!("C");
        // Order by pos
        let mut done = false;
        let mut old_lng = 0.0; // Value ASC forced
        let mut next_lng;
        let mut old_i;
        let mut next_index = 0;
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
                            angle_enum: t.angle_enum,
                            longitude: t.longitude,
                            space_left: t.space_left,
                            space_right: t.space_right,
                            fix: t.fix,
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
        //println!("D");

        // Order by index
        temp_order.clear();
        i = 1;
        done = false;
        let mut x_no_bug = 0;
        while !done {
            x_no_bug = x_no_bug + 1;
            for t in temp_no_order.clone() {
                if t.index == i {
                    temp_order.push(t);
                    i = i + 1;
                    break;
                }
            }
            if i > temp_no_order.len() as i16 {
                done = true;
            }
            if x_no_bug > 1000 {
                done = true;
            }
        }
        temp_no_order = temp_order.clone();

        // Left <-
        //println!("E");
        temp_order.clear();
        i = 0;
        done = false;
        let mut x_no_bug = 0;
        while !done {
            x_no_bug = x_no_bug + 1;
            let row = &temp_no_order[i as usize];
            let space_left;
            if i == 0 {
                let r_left = &temp_no_order[temp_no_order.len() - 1];
                space_left = self.get_fix_pos(
                    row.longitude.clone() - r_left.longitude + 360.0,
                );
            } else {
                let r_left = &temp_no_order[i as usize - 1];
                space_left =
                    self.get_fix_pos(row.longitude.clone() - r_left.longitude);
            };
            temp_order.push(TempPositionBodies {
                init_index: row.init_index,
                index: row.index,
                sw_reserve: row.sw_reserve,
                sw_bodie: row.sw_bodie,
                bodie_enum: row.bodie_enum,
                angle_enum: row.angle_enum,
                longitude: row.longitude,
                space_left: space_left,
                space_right: row.space_right,
                fix: row.fix,
                longitude_fix: row.longitude_fix,
            });
            i = i + 1;
            if (i > temp_no_order.len() as i16 - 1) || (x_no_bug > 1000) {
                done = true;
            }
        }
        temp_no_order = temp_order.clone();

        // Right ->
        //println!("F");
        temp_order.clear();
        i = temp_no_order.len() as i16 - 1;
        done = false;
        let mut x_no_bug = 0;
        while !done {
            x_no_bug = x_no_bug + 1;
            let row = &temp_no_order[i as usize];
            let space_right;
            if i == temp_no_order.len() as i16 - 1 {
                let r_right = &temp_no_order[0];
                space_right = self.get_fix_pos(
                    360.0 + r_right.longitude - row.longitude.clone(),
                );
            } else {
                let r_right = &temp_no_order[i as usize + 1];
                space_right =
                    self.get_fix_pos(r_right.longitude - row.longitude.clone());
            };
            temp_order.push(TempPositionBodies {
                init_index: row.init_index,
                index: row.index,
                sw_reserve: row.sw_reserve,
                sw_bodie: row.sw_bodie,
                bodie_enum: row.bodie_enum,
                angle_enum: row.angle_enum,
                longitude: row.longitude,
                space_left: row.space_left,
                space_right: space_right,
                fix: row.fix,
                longitude_fix: row.longitude_fix,
            });
            i = i - 1;
            if i <= -1 || x_no_bug > 1000 {
                done = true;
            }
        }
        temp_order.reverse();
        temp_no_order = temp_order.clone();

        // Fix
        //println!("G");
        done_main = false;
        let mut j = 0;
        let mut x_no_bug = 0;
        while !done_main {
            x_no_bug = x_no_bug + 1;
            temp_order.clear();
            i = temp_no_order.len() as i16 - 1;
            done = false;
            while !done {
                let row = &temp_no_order[i as usize];
                let mut fix = row.fix.clone();
                // Only to right
                /*if row.space_left < BODIE_DISTANCE {
                    if row.space_right > BODIE_DISTANCE {
                        fix = BODIE_DISTANCE * -1.0;
                    }
                } else*/
                if row.space_right < BODIE_DISTANCE_NATAL {
                    if row.space_left > BODIE_DISTANCE_OFFSET_NATAL {
                        fix = fix + BODIE_DISTANCE_OFFSET_NATAL;
                        j = j + 1;
                    }
                }
                let lng_fix = self.get_fix_pos(row.longitude.clone() - fix);
                temp_order.push(TempPositionBodies {
                    init_index: row.init_index,
                    index: row.index,
                    sw_reserve: row.sw_reserve,
                    sw_bodie: row.sw_bodie,
                    bodie_enum: row.bodie_enum,
                    angle_enum: row.angle_enum,
                    longitude: row.longitude,
                    space_left: row.space_left + fix,
                    space_right: row.space_right,
                    fix: fix,
                    longitude_fix: lng_fix,
                });
                i = i - 1;
                if i <= -1 || x_no_bug > 1000 {
                    done = true;
                }
            }
            // End compute
            temp_order.reverse();
            temp_no_order = temp_order.clone();

            temp_order.clear();
            done = false;
            i = 0;
            let mut x_no_bug = 0;
            while !done {
                x_no_bug = x_no_bug + 1;
                let row = &temp_no_order[i as usize];
                let next_row;
                let space_right;
                if i == temp_no_order.len() as i16 - 1 {
                    next_row = &temp_no_order[0];
                    space_right = self.get_fix_pos(
                        360.0 + next_row.longitude_fix
                            - row.longitude_fix.clone(),
                    );
                } else {
                    next_row = &temp_no_order[i as usize + 1];
                    space_right = self.get_fix_pos(
                        next_row.longitude_fix - row.longitude_fix.clone(),
                    )
                }
                temp_order.push(TempPositionBodies {
                    init_index: row.init_index,
                    index: row.index,
                    sw_reserve: row.sw_reserve,
                    sw_bodie: row.sw_bodie,
                    bodie_enum: row.bodie_enum,
                    angle_enum: row.angle_enum,
                    longitude: row.longitude,
                    space_left: row.space_left,
                    space_right: space_right,
                    fix: row.fix,
                    longitude_fix: row.longitude_fix,
                });
                i = i + 1;
                if i > temp_no_order.len() as i16 - 1 || x_no_bug > 1000 {
                    done = true;
                }
            }
            temp_no_order = temp_order.clone();

            if j == 0 {
                done_main = true;
            } else {
                j = 0;
            }
        }

        /*
        for t in temp_no_order.clone() {
            println!(
                "i: {}{} lng: {} left: {} right: {} fix: {} fl: {} fr: {}",
                t.index,
                t.bodie_enum,
                t.longitude,
                t.space_left,
                t.space_right,
                t.fix,
                (t.fix * -1.0) + t.space_left,
                (t.fix * 1.0) + t.space_right
            );
        }*/

        // Set
        self.temp_position_bodies = temp_no_order;
    }

    fn get_closest_distance(&self, angle1: Number, angle2: Number) -> Number {
        self.get_znorm(angle2 - angle1)
    }

    fn get_znorm(&self, mut angle: Number) -> Number {
        angle = angle % 360.0;
        if angle <= 180.0 {
            angle
        } else {
            angle - 360.0
        }
    }
}

impl CalcDraw for WorkingStoragePolyMorphTransit {
    fn get_radius_total(&self) -> Number {
        self.max_size / 2.0
    }

    fn get_radius_circle(&self, occurs: usize) -> (Number, bool) {
        if occurs > CIRCLE_SIZE_TRANSIT.len() {
            panic!("Out of range in circle occurs: {}", occurs);
        }
        (
            (self.get_radius_total() * CIRCLE_SIZE_TRANSIT[occurs].0) / 100.0,
            CIRCLE_SIZE_TRANSIT[occurs].1,
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
            * (((CIRCLE_SIZE_TRANSIT[2].0 - CIRCLE_SIZE_TRANSIT[1].0) / size)
                + CIRCLE_SIZE_TRANSIT[1].0)
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
            * (((CIRCLE_SIZE_TRANSIT[3].0 - CIRCLE_SIZE_TRANSIT[2].0)
                / div_trait_pointer)
                - CIRCLE_SIZE_TRANSIT[3].0))
            / 100.0
    }

    /// Top of pointer
    ///     HERE
    ///     /  \
    ///      ||
    ///      ||
    fn get_radius_rules_inside_circle_house_for_pointer_top(&self) -> Number {
        (self.get_radius_total()
            * ((CIRCLE_SIZE_TRANSIT[3].0 - CIRCLE_SIZE_TRANSIT[2].0)
                - CIRCLE_SIZE_TRANSIT[3].0))
            / 100.0
    }

    fn get_radius_circle_zodiac(&self) -> Number {
        let div_trait_big = 0.2;
        (self.get_radius_total()
            * (((CIRCLE_SIZE_TRANSIT[2].0 - CIRCLE_SIZE_TRANSIT[1].0)
                / (2.0 + div_trait_big))
                + CIRCLE_SIZE_TRANSIT[1].0))
            / 100.0
    }

    fn get_radius_circle_house(&self) -> Number {
        (self.get_radius_total()
            * (((CIRCLE_SIZE_TRANSIT[3].0 - CIRCLE_SIZE_TRANSIT[2].0) / 2.0)
                + CIRCLE_SIZE_TRANSIT[2].0))
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
            || bodie == Bodies::TrueNode // North Node true
       //     || bodie == Bodies::Chiron
       //     || bodie == Bodies::MeanApog // AsteroidLilith != Dark moon mean
            || bodie == Bodies::OscuApog // AsteroidLilith != Dark moon true
         //   || bodie == Bodies::Ceres
          //  || bodie == Bodies::SouthNode
           // || bodie == Bodies::FortunaPart
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

    fn get_bodie_longitude(&self, bodie: Bodies, sw_transit: bool) -> Number {
        let mut pos: Number = 0.0;
        if sw_transit {
            for b in self.object_transit.clone() {
                if b.object_enum.clone() == bodie {
                    pos = 360.0 - self.house[0].longitude as f32
                        + b.longitude as f32;
                    break;
                }
            }
        } else {
            for b in self.object_natal.clone() {
                if b.object_enum.clone() == bodie {
                    pos = 360.0 - self.house[0].longitude as f32
                        + b.longitude as f32;
                    break;
                }
            }
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    fn get_angle_fix_longitude(&self, angle: Angle) -> Number {
        let mut pos: Number = self.get_angle_longitude(angle.clone());
        if angle != Angle::Nothing {
            let arr = self.temp_position_bodies_natal.clone();
            for a in arr {
                if a.angle_enum == angle {
                    pos = a.longitude_fix;
                }
            }
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    fn get_bodie_fix_longitude(
        &self,
        bodie: Bodies,
        sw_transit: bool,
    ) -> Number {
        let mut pos: Number =
            self.get_bodie_longitude(bodie.clone(), sw_transit);
        if bodie != Bodies::EclNut {
            let arr = if sw_transit {
                self.temp_position_bodies_transit.clone()
            } else {
                self.temp_position_bodies_natal.clone()
            };
            for a in arr {
                if a.bodie_enum == bodie {
                    pos = a.longitude_fix;
                }
            }
        }
        pos = self.get_fix_pos(pos);
        pos
    }

    fn set_fix_compute(&mut self, sw_transit: bool) {
        let mut temp_no_order: Vec<TempPositionBodies> = Vec::new();
        let mut i: i16 = 0;
        //println!("A");
        if !sw_transit {
            for a in Angle::iter() {
                if self.get_angle_is_on_chart(a) {
                    i = i + 1;
                    let longitude = self.get_angle_longitude(a);
                    temp_no_order.push(TempPositionBodies {
                        init_index: i,
                        index: 0,
                        sw_reserve: false,
                        sw_bodie: false,
                        bodie_enum: Bodies::EclNut, // -1 Nothing, this variable
                        angle_enum: a,
                        longitude: longitude,
                        space_left: 0.0,
                        space_right: 0.0,
                        fix: 0.0,
                        longitude_fix: 0.0,
                    });
                }
            }
        }
        //println!("B");
        for b in Bodies::iter() {
            if self.get_bodie_is_on_chart(b) {
                i = i + 1;
                let longitude = self.get_bodie_longitude(b, sw_transit);
                temp_no_order.push(TempPositionBodies {
                    init_index: i,
                    index: 0,
                    sw_reserve: false,
                    sw_bodie: true,
                    bodie_enum: b,
                    angle_enum: Angle::Nothing,
                    longitude: longitude,
                    space_left: 0.0,
                    space_right: 0.0,
                    fix: 0.0,
                    longitude_fix: 0.0,
                });
            }
        }
        //println!("C");
        // Order by pos
        let mut done = false;
        let mut old_lng = 0.0; // Value ASC forced
        let mut next_lng;
        let mut old_i;
        let mut next_index = 0;
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
                            angle_enum: t.angle_enum,
                            longitude: t.longitude,
                            space_left: t.space_left,
                            space_right: t.space_right,
                            fix: t.fix,
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
        //println!("D");

        // Order by index
        temp_order.clear();
        i = 1;
        done = false;
        let mut x_no_bug = 0;
        while !done {
            x_no_bug = x_no_bug + 1;
            for t in temp_no_order.clone() {
                if t.index == i {
                    temp_order.push(t);
                    i = i + 1;
                    break;
                }
            }
            if i > temp_no_order.len() as i16 {
                done = true;
            }
            if x_no_bug > 1000 {
                done = true;
            }
        }
        temp_no_order = temp_order.clone();

        // Left <-
        //println!("E");
        temp_order.clear();
        i = 0;
        done = false;
        let mut x_no_bug = 0;
        while !done {
            x_no_bug = x_no_bug + 1;
            let row = &temp_no_order[i as usize];
            let space_left;
            if i == 0 {
                let r_left = &temp_no_order[temp_no_order.len() - 1];
                space_left = self.get_fix_pos(
                    row.longitude.clone() - r_left.longitude + 360.0,
                );
            } else {
                let r_left = &temp_no_order[i as usize - 1];
                space_left =
                    self.get_fix_pos(row.longitude.clone() - r_left.longitude);
            };
            temp_order.push(TempPositionBodies {
                init_index: row.init_index,
                index: row.index,
                sw_reserve: row.sw_reserve,
                sw_bodie: row.sw_bodie,
                bodie_enum: row.bodie_enum,
                angle_enum: row.angle_enum,
                longitude: row.longitude,
                space_left: space_left,
                space_right: row.space_right,
                fix: row.fix,
                longitude_fix: row.longitude_fix,
            });
            i = i + 1;
            if (i > temp_no_order.len() as i16 - 1) || (x_no_bug > 1000) {
                done = true;
            }
        }
        temp_no_order = temp_order.clone();

        // Right ->
        //println!("F");
        temp_order.clear();
        i = temp_no_order.len() as i16 - 1;
        done = false;
        let mut x_no_bug = 0;
        while !done {
            x_no_bug = x_no_bug + 1;
            let row = &temp_no_order[i as usize];
            let space_right;
            if i == temp_no_order.len() as i16 - 1 {
                let r_right = &temp_no_order[0];
                space_right = self.get_fix_pos(
                    360.0 + r_right.longitude - row.longitude.clone(),
                );
            } else {
                let r_right = &temp_no_order[i as usize + 1];
                space_right =
                    self.get_fix_pos(r_right.longitude - row.longitude.clone());
            };
            temp_order.push(TempPositionBodies {
                init_index: row.init_index,
                index: row.index,
                sw_reserve: row.sw_reserve,
                sw_bodie: row.sw_bodie,
                bodie_enum: row.bodie_enum,
                angle_enum: row.angle_enum,
                longitude: row.longitude,
                space_left: row.space_left,
                space_right: space_right,
                fix: row.fix,
                longitude_fix: row.longitude_fix,
            });
            i = i - 1;
            if i <= -1 || x_no_bug > 1000 {
                done = true;
            }
        }
        temp_order.reverse();
        temp_no_order = temp_order.clone();

        // Fix
        //println!("G");
        done_main = false;
        let mut j = 0;
        let mut x_no_bug = 0;
        while !done_main {
            x_no_bug = x_no_bug + 1;
            temp_order.clear();
            i = temp_no_order.len() as i16 - 1;
            done = false;
            while !done {
                let row = &temp_no_order[i as usize];
                let mut fix = row.fix.clone();
                // Only to right
                /*if row.space_left < BODIE_DISTANCE {
                    if row.space_right > BODIE_DISTANCE {
                        fix = BODIE_DISTANCE * -1.0;
                    }
                } else*/
                if sw_transit {
                    if row.space_right < BODIE_DISTANCE_TRANSIT {
                        if row.space_left > BODIE_DISTANCE_OFFSET_TRANSIT {
                            fix = fix + BODIE_DISTANCE_OFFSET_TRANSIT;
                            j = j + 1;
                        }
                    }
                } else {
                    if row.space_right < BODIE_DISTANCE_TRANSIT {
                        if row.space_left > BODIE_DISTANCE_OFFSET_TRANSIT {
                            fix = fix + BODIE_DISTANCE_OFFSET_TRANSIT;
                            j = j + 1;
                        }
                    }
                }
                let lng_fix = self.get_fix_pos(row.longitude.clone() - fix);
                temp_order.push(TempPositionBodies {
                    init_index: row.init_index,
                    index: row.index,
                    sw_reserve: row.sw_reserve,
                    sw_bodie: row.sw_bodie,
                    bodie_enum: row.bodie_enum,
                    angle_enum: row.angle_enum,
                    longitude: row.longitude,
                    space_left: row.space_left + fix,
                    space_right: row.space_right,
                    fix: fix,
                    longitude_fix: lng_fix,
                });
                i = i - 1;
                if i <= -1 || x_no_bug > 1000 {
                    done = true;
                }
            }
            // End compute
            temp_order.reverse();
            temp_no_order = temp_order.clone();

            temp_order.clear();
            done = false;
            i = 0;
            let mut x_no_bug = 0;
            while !done {
                x_no_bug = x_no_bug + 1;
                let row = &temp_no_order[i as usize];
                let next_row;
                let space_right;
                if i == temp_no_order.len() as i16 - 1 {
                    next_row = &temp_no_order[0];
                    space_right = self.get_fix_pos(
                        360.0 + next_row.longitude_fix
                            - row.longitude_fix.clone(),
                    );
                } else {
                    next_row = &temp_no_order[i as usize + 1];
                    space_right = self.get_fix_pos(
                        next_row.longitude_fix - row.longitude_fix.clone(),
                    )
                }
                temp_order.push(TempPositionBodies {
                    init_index: row.init_index,
                    index: row.index,
                    sw_reserve: row.sw_reserve,
                    sw_bodie: row.sw_bodie,
                    bodie_enum: row.bodie_enum,
                    angle_enum: row.angle_enum,
                    longitude: row.longitude,
                    space_left: row.space_left,
                    space_right: space_right,
                    fix: row.fix,
                    longitude_fix: row.longitude_fix,
                });
                i = i + 1;
                if i > temp_no_order.len() as i16 - 1 || x_no_bug > 1000 {
                    done = true;
                }
            }
            temp_no_order = temp_order.clone();

            if j == 0 {
                done_main = true;
            } else {
                j = 0;
            }
        }

        /*
        for t in temp_no_order.clone() {
            println!(
                "i: {}{} lng: {} left: {} right: {} fix: {} fl: {} fr: {}",
                t.index,
                t.bodie_enum,
                t.longitude,
                t.space_left,
                t.space_right,
                t.fix,
                (t.fix * -1.0) + t.space_left,
                (t.fix * 1.0) + t.space_right
            );
        }*/

        // Set
        if sw_transit {
            self.temp_position_bodies_transit = temp_no_order;
        } else {
            self.temp_position_bodies_natal = temp_no_order;
        }
    }

    fn get_closest_distance(&self, angle1: Number, angle2: Number) -> Number {
        self.get_znorm(angle2 - angle1)
    }

    fn get_znorm(&self, mut angle: Number) -> Number {
        angle = angle % 360.0;
        if angle <= 180.0 {
            angle
        } else {
            angle - 360.0
        }
    }
}

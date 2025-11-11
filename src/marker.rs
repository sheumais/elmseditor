use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}, str::from_utf8};

use js_sys::Date;
use regex::Regex;

use crate::zone::{Map, Zone};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Marker {
    Elms(ElmMarker),
    M0r(M0rMarker),
}

#[derive(Debug, Clone)]
pub struct ElmMarker {
    pub position: Position3D,
    pub icon: ElmsIcon,
    pub size: u8,
    pub active: bool,
    pub id: u16,
    pub map_id: u16
}

impl PartialEq for ElmMarker {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
        && self.icon == other.icon
        && self.active == other.active
        && self.size == other.size
        && self.map_id == other.map_id
    }
}

impl Eq for ElmMarker {}

impl Hash for ElmMarker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.icon.hash(state);
        self.active.hash(state);
        self.size.hash(state);
        self.map_id.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct M0rMarker {
    pub position: Position3D,
    pub background_texture: M0rTexture,
    pub text: Option<String>,
    pub size: f32,
    /// (Red, Green, Blue, Alpha)
    pub colour: (u8, u8, u8, u8),
    /// (Pitch, Yaw)
    /// 
    /// If none, always faces player (floating)
    pub orientation: Option<(i8, i16)>,
    pub active: bool,
    pub id: u16,
    pub map_id: u16
}

impl PartialEq for M0rMarker {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
        && self.background_texture == other.background_texture
        && self.colour == other.colour
        && self.text == other.text
        && self.map_id == other.map_id
        && self.orientation == other.orientation
        && self.size == other.size
    }
}

impl Eq for M0rMarker {}

impl Hash for M0rMarker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.background_texture.hash(state);
        self.colour.hash(state);
        self.text.hash(state);
        self.map_id.hash(state);
        if let Some((pitch, yaw)) = self.orientation {
            (pitch, yaw).hash(state);
        } else {
            None::<(i8, i16)>.hash(state);
        }
        self.size.to_bits().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum M0rTexture {
    Known(M0rIcon),
    Unknown(String),
    None,
}

pub fn get_marker_id(m: &Marker) -> u16 {
    match m {
        Marker::Elms(marker) => {marker.id},
        Marker::M0r(marker) => {marker.id}
    }
}

pub fn get_marker_position(m: &Marker) -> Position3D {
    match m {
        Marker::Elms(marker) => {marker.position},
        Marker::M0r(marker) => {marker.position}
    }
}

pub fn set_marker_active(m: &mut Marker, a: bool) {
    match m {
        Marker::Elms(marker) => {marker.active = a},
        Marker::M0r(marker) => {marker.active = a}
    }
}

pub enum MarkerTypes {
    Elms,
    M0r,
}

#[derive(Debug, Clone)]
pub struct BreadcrumbLine {
    pub position1: Position3D,
    pub position2: Position3D,
    pub active: bool,
    pub colour: (u8, u8, u8, u8),
    pub id: u16,
    pub map_id: u16,
}

impl PartialEq for BreadcrumbLine {
    fn eq(&self, other: &Self) -> bool {
        self.position1 == other.position1
        && self.position2 == other.position2
        && self.active == other.active
        && self.colour == other.colour
        && self.map_id == other.map_id
    }
}

impl Eq for BreadcrumbLine {}

impl Hash for BreadcrumbLine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position1.hash(state);
        self.position2.hash(state);
        self.active.hash(state);
        self.colour.hash(state);
        self.map_id.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElmsIcon {
    Num(u8), // 1–12
    Arrow,
    MarkerLightBlue,
    SquareBlue,
    SquareGreen,
    SquareOrange,
    SquareOrangeOT,
    SquarePink,
    SquareRed,
    SquareRedMT,
    SquareYellow,
    SquareTwoBlue,
    SquareTwoBlueOne,
    SquareTwoBlueTwo,
    SquareTwoBlueThree,
    SquareTwoBlueFour,
    SquareTwoGreen,
    SquareTwoGreenOne,
    SquareTwoGreenTwo,
    SquareTwoGreenThree,
    SquareTwoGreenFour,
    SquareTwoOrange,
    SquareTwoOrangeOne,
    SquareTwoOrangeTwo,
    SquareTwoOrangeThree,
    SquareTwoOrangeFour,
    SquareTwoPink,
    SquareTwoRed,
    SquareTwoRedOne,
    SquareTwoRedTwo,
    SquareTwoRedThree,
    SquareTwoRedFour,
    SquareTwoYellow,
    Letter(char), // a–z
    SharkPog,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M0rIcon {
    Blank,
    Circle,
    Hexagon,
    Square,
    Diamond,
    Octagon,
    Chevron,
    SharkPog,
    AllianceBadgeAldmeri,
    AllianceBadgeEbonheart,
    AllianceBadgeDaggerfall,
    RoleIconDPS,
    RoleIconTank,
    RoleIconHealer,
    ClassDragonknight,
    ClassSorcerer,
    ClassNightblade,
    ClassWarden,
    ClassNecromancer,
    ClassTemplar,
    ClassArcanist
}


pub const ALL_ELMS_ICONS: &[ElmsIcon] = &[
    ElmsIcon::Num(1),
    ElmsIcon::Num(2),
    ElmsIcon::Num(3),
    ElmsIcon::Num(4),
    ElmsIcon::Num(5),
    ElmsIcon::Num(6),
    ElmsIcon::Num(7),
    ElmsIcon::Num(8),
    ElmsIcon::Num(9),
    ElmsIcon::Num(10),
    ElmsIcon::Num(11),
    ElmsIcon::Num(12),
    ElmsIcon::Arrow,
    ElmsIcon::MarkerLightBlue,
    ElmsIcon::SquareBlue,
    ElmsIcon::SquareGreen,
    ElmsIcon::SquareOrange,
    ElmsIcon::SquareOrangeOT,
    ElmsIcon::SquarePink,
    ElmsIcon::SquareRed,
    ElmsIcon::SquareRedMT,
    ElmsIcon::SquareYellow,
    ElmsIcon::SquareTwoBlue,
    ElmsIcon::SquareTwoBlueOne,
    ElmsIcon::SquareTwoBlueTwo,
    ElmsIcon::SquareTwoBlueThree,
    ElmsIcon::SquareTwoBlueFour,
    ElmsIcon::SquareTwoGreen,
    ElmsIcon::SquareTwoGreenOne,
    ElmsIcon::SquareTwoGreenTwo,
    ElmsIcon::SquareTwoGreenThree,
    ElmsIcon::SquareTwoGreenFour,
    ElmsIcon::SquareTwoOrange,
    ElmsIcon::SquareTwoOrangeOne,
    ElmsIcon::SquareTwoOrangeTwo,
    ElmsIcon::SquareTwoOrangeThree,
    ElmsIcon::SquareTwoOrangeFour,
    ElmsIcon::SquareTwoPink,
    ElmsIcon::SquareTwoRed,
    ElmsIcon::SquareTwoRedOne,
    ElmsIcon::SquareTwoRedTwo,
    ElmsIcon::SquareTwoRedThree,
    ElmsIcon::SquareTwoRedFour,
    ElmsIcon::SquareTwoYellow,
    ElmsIcon::Letter('a'),
    ElmsIcon::Letter('b'),
    ElmsIcon::Letter('c'),
    ElmsIcon::Letter('d'),
    ElmsIcon::Letter('e'),
    ElmsIcon::Letter('f'),
    ElmsIcon::Letter('g'),
    ElmsIcon::Letter('h'),
    ElmsIcon::Letter('i'),
    ElmsIcon::Letter('j'),
    ElmsIcon::Letter('k'),
    ElmsIcon::Letter('l'),
    ElmsIcon::Letter('m'),
    ElmsIcon::Letter('n'),
    ElmsIcon::Letter('o'),
    ElmsIcon::Letter('p'),
    ElmsIcon::Letter('q'),
    ElmsIcon::Letter('r'),
    ElmsIcon::Letter('s'),
    ElmsIcon::Letter('t'),
    ElmsIcon::Letter('u'),
    ElmsIcon::Letter('v'),
    ElmsIcon::Letter('w'),
    ElmsIcon::Letter('x'),
    ElmsIcon::Letter('y'),
    ElmsIcon::Letter('z'),
    ElmsIcon::SharkPog,
    ElmsIcon::Unknown,
];

pub const ALL_M0R_ICONS: &[M0rTexture] = &[
    M0rTexture::Known(M0rIcon::Blank),
    M0rTexture::Known(M0rIcon::Circle),
    M0rTexture::Known(M0rIcon::Hexagon),
    M0rTexture::Known(M0rIcon::Square),
    M0rTexture::Known(M0rIcon::Diamond),
    M0rTexture::Known(M0rIcon::Octagon),
    M0rTexture::Known(M0rIcon::Chevron),
    M0rTexture::Known(M0rIcon::SharkPog),
    M0rTexture::Known(M0rIcon::AllianceBadgeAldmeri),
    M0rTexture::Known(M0rIcon::AllianceBadgeEbonheart),
    M0rTexture::Known(M0rIcon::AllianceBadgeDaggerfall),
    M0rTexture::Known(M0rIcon::RoleIconDPS),
    M0rTexture::Known(M0rIcon::RoleIconTank),
    M0rTexture::Known(M0rIcon::RoleIconHealer),
    M0rTexture::Known(M0rIcon::ClassDragonknight),
    M0rTexture::Known(M0rIcon::ClassSorcerer),
    M0rTexture::Known(M0rIcon::ClassNightblade),
    M0rTexture::Known(M0rIcon::ClassWarden),
    M0rTexture::Known(M0rIcon::ClassNecromancer),
    M0rTexture::Known(M0rIcon::ClassTemplar),
    M0rTexture::Known(M0rIcon::ClassArcanist),
];

fn find_best_map<'a>(x: i32, y: i32, z: i32, zone: &'a Zone) -> Option<&'a Map> {
    let matching_maps: Vec<&Map> = zone.maps.iter().filter(|map| {
        x >= map.scale_data.min_x as i32 && x <= map.scale_data.max_x as i32
        && z >= map.scale_data.min_z as i32 && z <= map.scale_data.max_z as i32
    }).collect();

    matching_maps.into_iter().min_by_key(|map| {
        let width  = (map.scale_data.max_x - map.scale_data.min_x) as u32;
        let depth  = (map.scale_data.max_z - map.scale_data.min_z) as u32;
        let area   = width.saturating_mul(depth);

        let (unknown_flag, y_offset) = match map.scale_data.y {
            Some(by) => {
                let dy = (y as f32 - by).abs() as u32;
                (0u32, dy)
            }
            None => (1u32, u32::MAX),
        };

        (unknown_flag, y_offset, area)
    })
}

pub fn parse_elms_string(elms_string: &str, zones: Vec<Zone>) -> HashMap<u16, Vec<Marker>> {
    let re = Regex::new(r"/(?P<zone>\d+)//(?P<x>\d+),(?P<y>\d+),(?P<z>\d+),(?P<icon>\d+)/").unwrap();
    let mut result: HashMap<u16, Vec<Marker>> = HashMap::new();
    let mut seen: HashMap<u16, HashSet<Marker>> = HashMap::new();
    let mut id_counter = 0;

    for caps in re.captures_iter(elms_string) {
        let zone_id: u16 = caps["zone"].parse().unwrap();
        let x: i32 = caps["x"].parse().unwrap();
        let y: i32 = caps["y"].parse().unwrap();
        let z: i32 = caps["z"].parse().unwrap();
        let icon_number: u16 = caps["icon"].parse().unwrap();
        let icon_enum = ElmsIcon::try_from(icon_number).unwrap_or(ElmsIcon::Unknown);
        let icon = icon_enum.into();

        if let Some(zone_obj) = zones.iter().find(|zone| zone.id == zone_id) {
            let best_map = find_best_map(x, y, z, zone_obj);

            let marker = ElmMarker {
                position: Position3D { x, y, z },
                icon,
                size: 1,
                active: true,
                id: id_counter,
                map_id: best_map.map_or(0, |m| m.map_id),
            };

            let entry_set = seen.entry(zone_id).or_default();
            if entry_set.insert(Marker::Elms(marker.clone())) {
                result.entry(zone_id).or_default().push(Marker::Elms(marker));
                id_counter += 1;
            }
        }
    }

    result
}

pub fn build_elms_string(markers_by_zone: &HashMap<u16, Vec<Marker>>) -> String {
    let mut all_zones: Vec<u16> = markers_by_zone.keys().cloned().collect();
    all_zones.sort();

    let mut result = String::new();
    for zone in all_zones {
        if let Some(markers) = markers_by_zone.get(&zone) {
            for m in markers {
                match m {
                    Marker::Elms(elms_marker) => {
                        if elms_marker.active {
                                result.push_str(&format!(
                                "/{}//{},{},{},{}/",
                                zone,
                                elms_marker.position.x,
                                elms_marker.position.y,
                                elms_marker.position.z,
                                u16::from(&elms_marker.icon),
                            ));
                        }
                    },
                    _ => {},
                }
            }

        }
    }

    result.trim().to_string()
}

pub fn hex_to_rgba(hex: u32) -> (u8, u8, u8, u8) {
    if hex <= 0xFFFFFF {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        (r, g, b, 255)
    } else {
        let r = ((hex >> 24) & 0xFF) as u8;
        let g = ((hex >> 16) & 0xFF) as u8;
        let b = ((hex >> 8) & 0xFF) as u8;
        let a = (hex & 0xFF) as u8;
        (r, g, b, a)
    }
}

pub fn hex_to_argb(hex: u32) -> (u8, u8, u8, u8) {
    if hex <= 0xFFFFFF {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        (255, r, g, b)
    } else {
        let a = ((hex >> 24) & 0xFF) as u8;
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        (a, r, g, b)
    }
}


pub fn rgba_to_hex_string(rgba: (u8, u8, u8, u8)) -> String {
    let (r, g, b, a) = rgba;
    if a == 255 {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}

pub fn argb_to_hex_string(argb: (u8, u8, u8, u8)) -> String {
    let (a, r, g, b) = argb;
    if a == 255 {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", a, r, g, b)
    }
}


pub fn parse_lines_string(lines_string: &str, zones: Vec<Zone>) -> HashMap<u16, Vec<BreadcrumbLine>> {
    let mut result: HashMap<u16, Vec<BreadcrumbLine>> = HashMap::new();

    let mut parts = lines_string.split(';').filter(|s| !s.trim().is_empty()).peekable();

    while parts.peek().is_some() {
        let zone_id_hex_raw = parts.next().unwrap_or("0").trim();
        let zone_id_hex: String = zone_id_hex_raw
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        let Ok(zone_id) = u16::from_str_radix(&zone_id_hex, 16) else {
            continue;
        };

        let min_x = i32::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
        let min_y = i32::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
        let min_z = i32::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);

        let colour_count = usize::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
        let mut colours: Vec<(u8, u8, u8, u8)> = Vec::new();
        for _ in 0..colour_count {
            if let Some(hex_str) = parts.next() {
                if let Ok(hex_val) = u32::from_str_radix(hex_str.trim(), 16) {
                    colours.push(hex_to_rgba(hex_val));
                }
            }
        }

        let point_count = usize::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
        let mut points: Vec<Position3D> = Vec::new();
        for _ in 0..point_count {
            let x = i32::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
            let y = i32::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
            let z = i32::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
            points.push(Position3D { x, y, z });
        }

        let line_count = usize::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(0);
        let mut lines: Vec<BreadcrumbLine> = Vec::new();
        let mut id_counter = 0;

        for _ in 0..line_count {
            let colour_index = usize::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(1);
            let p1_index = usize::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(1);
            let p2_index = usize::from_str_radix(parts.next().unwrap_or("0").trim(), 16).unwrap_or(1);

            if let (Some(&c), Some(p1), Some(p2)) = (
                colours.get(colour_index.saturating_sub(1)),
                points.get(p1_index.saturating_sub(1)),
                points.get(p2_index.saturating_sub(1)),
            ) {
                let pos1 = Position3D { x: p1.x + min_x, y: p1.y + min_y, z: p1.z + min_z };
                let pos2 = Position3D { x: p2.x + min_x, y: p2.y + min_y, z: p2.z + min_z };

                let mid_x = (pos1.x + pos2.x) / 2;
                let mid_y = (pos1.y + pos2.y) / 2;
                let mid_z = (pos1.z + pos2.z) / 2;

                let map_id = zones
                    .iter()
                    .find(|zone| zone.id == zone_id)
                    .and_then(|zone| find_best_map(mid_x, mid_y, mid_z, zone))
                    .map_or(0, |m| m.map_id);

                lines.push(BreadcrumbLine {
                    position1: pos1,
                    position2: pos2,
                    active: true,
                    colour: c,
                    id: id_counter,
                    map_id,
                });

                id_counter += 1;
            }
        }

        if !lines.is_empty() {
            result.insert(zone_id, lines);
        }
    }

    result
}

pub fn lines_to_string(lines_by_zone: &HashMap<u16, Vec<BreadcrumbLine>>) -> String {
    let mut result = String::new();

    for (zone_id, lines) in lines_by_zone {
        if lines.is_empty() {
            continue;
        }

        result.push_str(&format!("{:X};", zone_id));

        let (min_x, min_y, min_z) = lines.iter().flat_map(|line| [line.position1, line.position2])
            .fold((i32::MAX, i32::MAX, i32::MAX), |(mx, my, mz), pos| {
                (mx.min(pos.x), my.min(pos.y), mz.min(pos.z))
            });

        result.push_str(&format!("{:X};{:X};{:X};", min_x, min_y, min_z));

        let mut colours: Vec<(u8, u8, u8, u8)> = Vec::new();
        for line in lines {
            if !colours.contains(&line.colour) {
                colours.push(line.colour);
            }
        }
        result.push_str(&format!("{:X};", colours.len()));
        for &(r, g, b, _a) in &colours {
            let hex_val = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            result.push_str(&format!("{:X};", hex_val));
        }

        let mut points: Vec<Position3D> = Vec::new();
        let mut point_index = HashMap::new();
        let mut next_idx = 1;

        let add_point = |pos: Position3D, points: &mut Vec<Position3D>, point_index: &mut HashMap<Position3D, usize>, next_idx: &mut usize| {
            let rel = Position3D { x: pos.x - min_x, y: pos.y - min_y, z: pos.z - min_z };
            if let Some(&idx) = point_index.get(&rel) {
                idx
            } else {
                points.push(rel);
                point_index.insert(rel, *next_idx);
                *next_idx += 1;
                *next_idx - 1
            }
        };

        for line in lines {
            add_point(line.position1, &mut points, &mut point_index, &mut next_idx);
            add_point(line.position2, &mut points, &mut point_index, &mut next_idx);
        }

        result.push_str(&format!("{:X};", points.len()));
        for p in &points {
            result.push_str(&format!("{:X};{:X};{:X};", p.x, p.y, p.z));
        }

        result.push_str(&format!("{:X};", lines.len()));
        for line in lines {
            let colour_idx = colours.iter().position(|&c| c == line.colour).unwrap_or(0) + 1;
            let p1_idx = point_index[&Position3D { x: line.position1.x - min_x, y: line.position1.y - min_y, z: line.position1.z - min_z }];
            let p2_idx = point_index[&Position3D { x: line.position2.x - min_x, y: line.position2.y - min_y, z: line.position2.z - min_z }];
            result.push_str(&format!("{:X};{:X};{:X};", colour_idx, p1_idx, p2_idx));
        }
    }

    result
}

const MOR_COLON: &[u8] = b"\xee\x80\x80";
const MOR_COMMA: &[u8] = b"\xee\x80\x81";
const MOR_SQUAREBRACKET: &[u8] = b"\xee\x80\x82";
const MOR_SEMICOLON: &[u8] = b"\xee\x80\x83";
const MOR_GREATERTHAN: &[u8] = b"\xee\x80\x84";

fn parse_hex_i32(s: &str) -> i32 {
    let s = s.trim().trim_start_matches("0x");
    if s.is_empty() {
        return 0;
    }
    match i32::from_str_radix(s, 16) {
        Ok(v) => v,
        Err(_) => 0,
    }
}

pub fn parse_m0r_string(m0r_string: &str, zones: Vec<Zone>) -> HashMap<u16, Vec<Marker>> {
    let mut result: HashMap<u16, Vec<Marker>> = HashMap::new();

    let trimmed = m0r_string.trim();
    let inner = match (trimmed.find('<'), trimmed.rfind('>')) {
        (Some(start), Some(end)) if end > start => &trimmed[start + 1..end],
        _ => {
            eprintln!("Malformed m0r marker string: no enclosing <>");
            return result;
        }
    };

    let fields: Vec<&str> = inner.split(']').collect();
    if fields.len() < 9 {
        eprintln!(
            "Malformed m0r marker string: expected >=9 ']' separated fields, got {}",
            fields.len()
        );
        return result;
    }

    let zone_str = fields[0].trim();
    let mins = fields[2].trim();
    let sizes = fields[3].trim();
    let pitch_field = fields[4].trim();
    let yaw_field = fields[5].trim();
    let colour_field = fields[6].trim();
    let texture_field = fields[7].trim();
    let positions_field = fields[8].trim();

    let zone = match zones.iter().find(|z| z.id.to_string() == zone_str) {
        Some(z) => z,
        None => return result,
    };

    let mins_parts: Vec<&str> = mins.split(':').collect();
    let min_x = parse_hex_i32(mins_parts.get(0).unwrap_or(&"0"));
    let min_y = parse_hex_i32(mins_parts.get(1).unwrap_or(&"0"));
    let min_z = parse_hex_i32(mins_parts.get(2).unwrap_or(&"0"));

    let unescape_text = |s: &str| {
        let mut t = s.to_string();
        t = t.replace(std::str::from_utf8(MOR_COLON).unwrap_or("::"), ":");
        t = t.replace(std::str::from_utf8(MOR_COMMA).unwrap_or(","), ",");
        t = t.replace(std::str::from_utf8(MOR_SQUAREBRACKET).unwrap_or("]"), "]");
        t = t.replace(std::str::from_utf8(MOR_SEMICOLON).unwrap_or(";"), ";");
        t = t.replace(std::str::from_utf8(MOR_GREATERTHAN).unwrap_or(">"), ">");
        t = t.replace(r#"\\n"#, "\n");
        t
    };

    let mut markers: Vec<M0rMarker> = Vec::new();
    for chunk in positions_field.split(',') {
        let parts: Vec<&str> = chunk.trim().split(':').collect();
        if parts.len() < 3 {
            continue;
        }

        let cx = parse_hex_i32(parts[0]) + min_x;
        let cy = parse_hex_i32(parts[1]) + min_y;
        let cz = parse_hex_i32(parts[2]) + min_z;

        let map = match find_best_map(cx, cy, cz, zone) {
            Some(m) => m,
            None => continue,
        };

        let text_raw = parts.get(3).copied().unwrap_or("");
        let mut unescaped_text = unescape_text(text_raw);
        if unescaped_text.is_empty() {
            unescaped_text = String::new();
        }

        let new_marker = M0rMarker {
            id: markers.len() as u16,
            map_id: map.map_id,
            active: true,
            position: Position3D { x: cx, y: cy, z: cz },
            background_texture: M0rTexture::None,
            text: unescaped_text.into(),
            size: 1.0,
            colour: (255, 255, 255, 255),
            orientation: None,
        };

        markers.push(new_marker);
    }

    let parse_index = |s: &str| s.parse::<usize>().ok().and_then(|i| i.checked_sub(1));

    for segment in sizes.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((size_str, idx_str)) = segment.split_once(':') {
            if let Ok(size) = size_str.parse::<f32>() {
                for idx in idx_str.split(',').map(str::trim) {
                    if let Some(i) = parse_index(idx) {
                        if let Some(marker) = markers.get_mut(i) {
                            marker.size = size;
                        }
                    }
                }
            }
        }
    }

    for segment in colour_field.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((hex, idx_str)) = segment.split_once(':') {
            if let Ok(hex_val) = u32::from_str_radix(hex.trim_start_matches("0x"), 16) {
                let (a, r, g, b) = hex_to_argb(hex_val);
                for idx in idx_str.split(',').map(str::trim) {
                    if let Some(i) = parse_index(idx) {
                        if let Some(marker) = markers.get_mut(i) {
                            marker.colour = (r, g, b, a);
                        }
                    }
                }
            }
        }
    }

    for segment in pitch_field.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((p_str, idx_str)) = segment.split_once(':') {
            if let Ok(pitch) = p_str.parse::<i8>() {
                for idx in idx_str.split(',').map(str::trim) {
                    if let Some(i) = parse_index(idx) {
                        if let Some(marker) = markers.get_mut(i) {
                            let yaw = marker.orientation.map(|(_, y)| y).unwrap_or(0);
                            marker.orientation = Some((pitch, yaw));
                        }
                    }
                }
            }
        }
    }

    for segment in yaw_field.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((y_str, idx_str)) = segment.split_once(':') {
            if let Ok(yaw) = y_str.parse::<i16>() {
                for idx in idx_str.split(',').map(str::trim) {
                    if let Some(i) = parse_index(idx) {
                        if let Some(marker) = markers.get_mut(i) {
                            let pitch = marker.orientation.map(|(p, _)| p).unwrap_or(0);
                            marker.orientation = Some((pitch, yaw));
                        }
                    }
                }
            }
        }
    }

    for segment in texture_field.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((tex_str, idx_str)) = segment.split_once(':') {
            for idx in idx_str.split(',').map(str::trim) {
                if let Some(i) = parse_index(idx) {
                    if let Some(marker) = markers.get_mut(i) {
                        marker.background_texture = M0rTexture::from(tex_str);
                    }
                }
            }
        }
    }

    let returned_markers = markers.into_iter().map(Marker::M0r).collect::<Vec<_>>();
    result.insert(zone.id, returned_markers);

    result
}

pub fn get_timestamp() -> String {
    let timestamp = Date::now() / 1000.0;
    timestamp.floor().to_string()
}

pub fn build_m0r_string(markers_by_zone: &HashMap<u16, Vec<Marker>>) -> String {
    let mut result = String::new();

    for (zone_id, markers) in markers_by_zone {
        if markers.is_empty() {
            continue;
        }

        let min_x = markers.iter().map(|m| get_marker_position(m).x).min().unwrap_or(0);
        let min_y = markers.iter().map(|m| get_marker_position(m).y).min().unwrap_or(0);
        let min_z = markers.iter().map(|m| get_marker_position(m).z).min().unwrap_or(0);

        let mins = format!(
            "{:X}:{:X}:{:X}",
            min_x.max(0) as u32,
            min_y.max(0) as u32,
            min_z.max(0) as u32
        );

        let timestamp = get_timestamp();

        let mut size_groups: HashMap<String, Vec<usize>> = HashMap::new();
        let mut pitch_groups: HashMap<i8, Vec<usize>> = HashMap::new();
        let mut yaw_groups: HashMap<i16, Vec<usize>> = HashMap::new();
        let mut colour_groups: HashMap<String, Vec<usize>> = HashMap::new();
        let mut texture_groups: HashMap<M0rTexture, Vec<usize>> = HashMap::new();

        let mut positions: Vec<String> = Vec::new();

        for (i, marker_enum) in markers.iter().enumerate() {
            if let Marker::M0r(marker) = marker_enum {
                if !marker.active {continue;}
                let pos = marker.position;
                let cx = (pos.x - min_x) as u32;
                let cy = (pos.y - min_y) as u32;
                let cz = (pos.z - min_z) as u32;

                let mut text = marker.text.clone().unwrap_or_else(|| "".to_string());
                text = text
                    .replace(":", from_utf8(MOR_COLON).unwrap_or("::"))
                    .replace(",", from_utf8(MOR_COMMA).unwrap_or(","))
                    .replace("]", from_utf8(MOR_SQUAREBRACKET).unwrap_or("]"))
                    .replace(";", from_utf8(MOR_SEMICOLON).unwrap_or(";"))
                    .replace(">", from_utf8(MOR_GREATERTHAN).unwrap_or(">"))
                    .replace("\n", r#"\\n"#);

                let pos_string = format!("{:X}:{:X}:{:X}", cx, cy, cz).to_lowercase();
                positions.push(format!("{}:{}", pos_string, text));

                size_groups.entry(marker.size.to_string()).or_default().push(i + 1);

                if let Some((p, y)) = marker.orientation {
                    pitch_groups.entry(p).or_default().push(i + 1);
                    yaw_groups.entry(y).or_default().push(i + 1);
                }

                let (r, g, b, a) = marker.colour;
                let rgba_hex = argb_to_hex_string((a, r, g, b));
                colour_groups.entry(rgba_hex).or_default().push(i + 1);

                texture_groups.entry(marker.background_texture.clone()).or_default().push(i + 1);
            }
        }

        let sizes_str = {
            let mut parts: Vec<String> = Vec::new();
            for (size, idxs) in size_groups {
                if size == "1" {continue;}
                let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
                parts.push(format!("{}:{}", size, idx_str));
            }
            parts.join(";")
        };

        let pitches_str = {
            let mut parts: Vec<String> = Vec::new();
            for (p, idxs) in pitch_groups {
                let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
                parts.push(format!("{}:{}", p, idx_str));
            }
            parts.join(";")
        };

        let yaws_str = {
            let mut parts: Vec<String> = Vec::new();
            for (y, idxs) in yaw_groups {
                let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
                parts.push(format!("{}:{}", y, idx_str));
            }
            parts.join(";")
        };

        let colours_str = {
            let mut parts: Vec<String> = Vec::new();
            for (hex, idxs) in colour_groups {
                let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
                parts.push(format!("{}:{}", hex, idx_str));
            }
            parts.join(";")
        };

        let textures_str = {
            let mut parts: Vec<String> = Vec::new();
            for (tex, idxs) in texture_groups {
                let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
                parts.push(format!("{}:{}", m0r_texture_to_og(&tex), idx_str));
            }
            parts.join(";")
        };

        let positions_str = positions.join(",");

        result.push_str(&format!(
            "<{}]{}]{}]{}]{}]{}]{}]{}]{}>",
            zone_id,
            timestamp,
            mins.to_lowercase(),
            sizes_str.to_lowercase(),
            pitches_str.to_lowercase(),
            yaws_str.to_lowercase(),
            colours_str.to_lowercase(),
            textures_str,
            positions_str
        ));
    }

    result
}

impl From<u16> for ElmsIcon {
    fn from(n: u16) -> Self {
        match n {
            1..=12 => ElmsIcon::Num(n as u8),
            13 => ElmsIcon::Arrow,
            14 => ElmsIcon::MarkerLightBlue, // this is very definitely GREEN but whatever 
            15 => ElmsIcon::SquareBlue,
            16 => ElmsIcon::SquareGreen,
            17 => ElmsIcon::SquareOrange,
            18 => ElmsIcon::SquareOrangeOT,
            19 => ElmsIcon::SquarePink,
            20 => ElmsIcon::SquareRed,
            21 => ElmsIcon::SquareRedMT,
            22 => ElmsIcon::SquareYellow,
            23 => ElmsIcon::SquareTwoBlue,
            24 => ElmsIcon::SquareTwoBlueOne,
            25 => ElmsIcon::SquareTwoBlueTwo,
            26 => ElmsIcon::SquareTwoBlueThree,
            27 => ElmsIcon::SquareTwoBlueFour,
            28 => ElmsIcon::SquareTwoGreen,
            29 => ElmsIcon::SquareTwoGreenOne,
            30 => ElmsIcon::SquareTwoGreenTwo,
            31 => ElmsIcon::SquareTwoGreenThree,
            32 => ElmsIcon::SquareTwoGreenFour,
            33 => ElmsIcon::SquareTwoOrange,
            34 => ElmsIcon::SquareTwoOrangeOne,
            35 => ElmsIcon::SquareTwoOrangeTwo,
            36 => ElmsIcon::SquareTwoOrangeThree,
            37 => ElmsIcon::SquareTwoOrangeFour,
            38 => ElmsIcon::SquareTwoPink,
            39 => ElmsIcon::SquareTwoRed,
            40 => ElmsIcon::SquareTwoRedOne,
            41 => ElmsIcon::SquareTwoRedTwo,
            42 => ElmsIcon::SquareTwoRedThree,
            43 => ElmsIcon::SquareTwoRedFour,
            44 => ElmsIcon::SquareTwoYellow,
            45..=70 => ElmsIcon::Letter((b'a' + (n - 45) as u8) as char),
            71 => ElmsIcon::SharkPog,
            _ => ElmsIcon::Unknown,
        }
    }
}

impl From<&ElmsIcon> for u16 {
    fn from(marker_icon: &ElmsIcon) -> u16 {
        match marker_icon {
            ElmsIcon::Num(n) if (1..=12).contains(n) => *n as u16,
            ElmsIcon::Arrow => 13,
            ElmsIcon::MarkerLightBlue => 14,
            ElmsIcon::SquareBlue => 15,
            ElmsIcon::SquareGreen => 16,
            ElmsIcon::SquareOrange => 17,
            ElmsIcon::SquareOrangeOT => 18,
            ElmsIcon::SquarePink => 19,
            ElmsIcon::SquareRed => 20,
            ElmsIcon::SquareRedMT => 21,
            ElmsIcon::SquareYellow => 22,
            ElmsIcon::SquareTwoBlue => 23,
            ElmsIcon::SquareTwoBlueOne => 24,
            ElmsIcon::SquareTwoBlueTwo => 25,
            ElmsIcon::SquareTwoBlueThree => 26,
            ElmsIcon::SquareTwoBlueFour => 27,
            ElmsIcon::SquareTwoGreen => 28,
            ElmsIcon::SquareTwoGreenOne => 29,
            ElmsIcon::SquareTwoGreenTwo => 30,
            ElmsIcon::SquareTwoGreenThree => 31,
            ElmsIcon::SquareTwoGreenFour => 32,
            ElmsIcon::SquareTwoOrange => 33,
            ElmsIcon::SquareTwoOrangeOne => 34,
            ElmsIcon::SquareTwoOrangeTwo => 35,
            ElmsIcon::SquareTwoOrangeThree => 36,
            ElmsIcon::SquareTwoOrangeFour => 37,
            ElmsIcon::SquareTwoPink => 38,
            ElmsIcon::SquareTwoRed => 39,
            ElmsIcon::SquareTwoRedOne => 40,
            ElmsIcon::SquareTwoRedTwo => 41,
            ElmsIcon::SquareTwoRedThree => 42,
            ElmsIcon::SquareTwoRedFour => 43,
            ElmsIcon::SquareTwoYellow => 44,
            ElmsIcon::Letter(c) if c.is_ascii_lowercase() => 45 + (*c as u8 - b'a') as u16,
            ElmsIcon::SharkPog => 71,
            ElmsIcon::Unknown => 14,
            _ => 14,
        }
    }
}

impl From<&str> for ElmsIcon {
    fn from(s: &str) -> Self {
        match s {
            "1.png" => ElmsIcon::Num(1),
            "2.png" => ElmsIcon::Num(2),
            "3.png" => ElmsIcon::Num(3),
            "4.png" => ElmsIcon::Num(4),
            "5.png" => ElmsIcon::Num(5),
            "6.png" => ElmsIcon::Num(6),
            "7.png" => ElmsIcon::Num(7),
            "8.png" => ElmsIcon::Num(8),
            "9.png" => ElmsIcon::Num(9),
            "10.png" => ElmsIcon::Num(10),
            "11.png" => ElmsIcon::Num(11),
            "12.png" => ElmsIcon::Num(12),
            "arrow.png" => ElmsIcon::Arrow,
            "squares/marker_lightblue.png" => ElmsIcon::MarkerLightBlue,
            "squares/square_blue.png" => ElmsIcon::SquareBlue,
            "squares/square_green.png" => ElmsIcon::SquareGreen,
            "squares/square_orange.png" => ElmsIcon::SquareOrange,
            "squares/square_orange_OT.png" => ElmsIcon::SquareOrangeOT,
            "squares/square_pink.png" => ElmsIcon::SquarePink,
            "squares/square_red.png" => ElmsIcon::SquareRed,
            "squares/square_red_MT.png" => ElmsIcon::SquareRedMT,
            "squares/square_yellow.png" => ElmsIcon::SquareYellow,
            "squares/squaretwo_blue.png" => ElmsIcon::SquareTwoBlue,
            "squares/squaretwo_blue_one.png" => ElmsIcon::SquareTwoBlueOne,
            "squares/squaretwo_blue_two.png" => ElmsIcon::SquareTwoBlueTwo,
            "squares/squaretwo_blue_three.png" => ElmsIcon::SquareTwoBlueThree,
            "squares/squaretwo_blue_four.png" => ElmsIcon::SquareTwoBlueFour,
            "squares/squaretwo_green.png" => ElmsIcon::SquareTwoGreen,
            "squares/squaretwo_green_one.png" => ElmsIcon::SquareTwoGreenOne,
            "squares/squaretwo_green_two.png" => ElmsIcon::SquareTwoGreenTwo,
            "squares/squaretwo_green_three.png" => ElmsIcon::SquareTwoGreenThree,
            "squares/squaretwo_green_four.png" => ElmsIcon::SquareTwoGreenFour,
            "squares/squaretwo_orange.png" => ElmsIcon::SquareTwoOrange,
            "squares/squaretwo_orange_one.png" => ElmsIcon::SquareTwoOrangeOne,
            "squares/squaretwo_orange_two.png" => ElmsIcon::SquareTwoOrangeTwo,
            "squares/squaretwo_orange_three.png" => ElmsIcon::SquareTwoOrangeThree,
            "squares/squaretwo_orange_four.png" => ElmsIcon::SquareTwoOrangeFour,
            "squares/squaretwo_pink.png" => ElmsIcon::SquareTwoPink,
            "squares/squaretwo_red.png" => ElmsIcon::SquareTwoRed,
            "squares/squaretwo_red_one.png" => ElmsIcon::SquareTwoRedOne,
            "squares/squaretwo_red_two.png" => ElmsIcon::SquareTwoRedTwo,
            "squares/squaretwo_red_three.png" => ElmsIcon::SquareTwoRedThree,
            "squares/squaretwo_red_four.png" => ElmsIcon::SquareTwoRedFour,
            "squares/squaretwo_yellow.png" => ElmsIcon::SquareTwoYellow,
            "sharkpog.png" => ElmsIcon::SharkPog,
            s if s.len() == 5 && s.ends_with(".png") && s.chars().next().unwrap().is_ascii_lowercase() => {
                ElmsIcon::Letter(s.chars().next().unwrap())
            }
            _ => ElmsIcon::Unknown,
        }
    }
}

impl From<ElmsIcon> for String {
    fn from(marker_icon: ElmsIcon) -> Self {
        match marker_icon {
            ElmsIcon::Num(n) => format!("{}.png", n),
            ElmsIcon::Arrow => "arrow.png".into(),
            ElmsIcon::MarkerLightBlue => "squares/marker_lightblue.png".into(),
            ElmsIcon::SquareBlue => "squares/square_blue.png".into(),
            ElmsIcon::SquareGreen => "squares/square_green.png".into(),
            ElmsIcon::SquareOrange => "squares/square_orange.png".into(),
            ElmsIcon::SquareOrangeOT => "squares/square_orange_OT.png".into(),
            ElmsIcon::SquarePink => "squares/square_pink.png".into(),
            ElmsIcon::SquareRed => "squares/square_red.png".into(),
            ElmsIcon::SquareRedMT => "squares/square_red_MT.png".into(),
            ElmsIcon::SquareYellow => "squares/square_yellow.png".into(),
            ElmsIcon::SquareTwoBlue => "squares/squaretwo_blue.png".into(),
            ElmsIcon::SquareTwoBlueOne => "squares/squaretwo_blue_one.png".into(),
            ElmsIcon::SquareTwoBlueTwo => "squares/squaretwo_blue_two.png".into(),
            ElmsIcon::SquareTwoBlueThree => "squares/squaretwo_blue_three.png".into(),
            ElmsIcon::SquareTwoBlueFour => "squares/squaretwo_blue_four.png".into(),
            ElmsIcon::SquareTwoGreen => "squares/squaretwo_green.png".into(),
            ElmsIcon::SquareTwoGreenOne => "squares/squaretwo_green_one.png".into(),
            ElmsIcon::SquareTwoGreenTwo => "squares/squaretwo_green_two.png".into(),
            ElmsIcon::SquareTwoGreenThree => "squares/squaretwo_green_three.png".into(),
            ElmsIcon::SquareTwoGreenFour => "squares/squaretwo_green_four.png".into(),
            ElmsIcon::SquareTwoOrange => "squares/squaretwo_orange.png".into(),
            ElmsIcon::SquareTwoOrangeOne => "squares/squaretwo_orange_one.png".into(),
            ElmsIcon::SquareTwoOrangeTwo => "squares/squaretwo_orange_two.png".into(),
            ElmsIcon::SquareTwoOrangeThree => "squares/squaretwo_orange_three.png".into(),
            ElmsIcon::SquareTwoOrangeFour => "squares/squaretwo_orange_four.png".into(),
            ElmsIcon::SquareTwoPink => "squares/squaretwo_pink.png".into(),
            ElmsIcon::SquareTwoRed => "squares/squaretwo_red.png".into(),
            ElmsIcon::SquareTwoRedOne => "squares/squaretwo_red_one.png".into(),
            ElmsIcon::SquareTwoRedTwo => "squares/squaretwo_red_two.png".into(),
            ElmsIcon::SquareTwoRedThree => "squares/squaretwo_red_three.png".into(),
            ElmsIcon::SquareTwoRedFour => "squares/squaretwo_red_four.png".into(),
            ElmsIcon::SquareTwoYellow => "squares/squaretwo_yellow.png".into(),
            ElmsIcon::Letter(c) => format!("{}.png", c),
            ElmsIcon::SharkPog => "sharkpog.png".into(),
            ElmsIcon::Unknown => "unknown.png".into(),
        }
    }
}

impl From<&str> for M0rTexture {
    fn from(s: &str) -> Self {
        match s {
            "^1" => M0rTexture::Known(M0rIcon::Circle),
            "^2" => M0rTexture::Known(M0rIcon::Hexagon),
            "^3" => M0rTexture::Known(M0rIcon::Square),
            "^4" => M0rTexture::Known(M0rIcon::Diamond),
            "^5" => M0rTexture::Known(M0rIcon::Octagon),
            "^6" => M0rTexture::Known(M0rIcon::Chevron),
            "^7" => M0rTexture::Known(M0rIcon::Blank),
            "^8" => M0rTexture::Known(M0rIcon::SharkPog),
            "^9" => M0rTexture::Known(M0rIcon::AllianceBadgeAldmeri),
            "^10" => M0rTexture::Known(M0rIcon::AllianceBadgeEbonheart),
            "^11" => M0rTexture::Known(M0rIcon::AllianceBadgeDaggerfall),
            "^12" => M0rTexture::Known(M0rIcon::RoleIconDPS),
            "^13" => M0rTexture::Known(M0rIcon::RoleIconTank),
            "^14" => M0rTexture::Known(M0rIcon::RoleIconHealer),
            "^15" => M0rTexture::Known(M0rIcon::ClassDragonknight),
            "^16" => M0rTexture::Known(M0rIcon::ClassSorcerer),
            "^17" => M0rTexture::Known(M0rIcon::ClassNightblade),
            "^18" => M0rTexture::Known(M0rIcon::ClassWarden),
            "^19" => M0rTexture::Known(M0rIcon::ClassNecromancer),
            "^20" => M0rTexture::Known(M0rIcon::ClassTemplar),
            "^21" => M0rTexture::Known(M0rIcon::ClassArcanist),
            _ => if s.is_empty() {
                M0rTexture::None   
            } else {
                M0rTexture::Unknown(s.to_string())
            }
        }
    }
}

impl From<M0rTexture> for String {
    fn from(t: M0rTexture) -> Self {
        match t {
            M0rTexture::Known(p) => {
                match p {
                    M0rIcon::Blank => "blank.png".into(),
                    M0rIcon::Circle => "circle.svg".into(),
                    M0rIcon::Hexagon => "hexagon.svg".into(),
                    M0rIcon::Square => "square.svg".into(),
                    M0rIcon::Diamond => "diamond.svg".into(),
                    M0rIcon::Octagon => "octagon.svg".into(),
                    M0rIcon::Chevron => "chevron.svg".into(),
                    M0rIcon::SharkPog => "sharkpog.png".into(),
                    M0rIcon::AllianceBadgeAldmeri => "alliancebadge_aldmeri.png".into(),
                    M0rIcon::AllianceBadgeEbonheart => "alliancebadge_ebonheart.png".into(),
                    M0rIcon::AllianceBadgeDaggerfall => "alliancebadge_daggerfall.png".into(),
                    M0rIcon::RoleIconDPS => "lfg_roleicon_dps.png".into(),
                    M0rIcon::RoleIconTank => "lfg_roleicon_tank.png".into(),
                    M0rIcon::RoleIconHealer => "lfg_roleicon_healer.png".into(),
                    M0rIcon::ClassDragonknight => "gp_class_dragonknight.png".into(),
                    M0rIcon::ClassSorcerer => "gp_class_sorcerer.png".into(),
                    M0rIcon::ClassNightblade => "gp_class_nightblade.png".into(),
                    M0rIcon::ClassWarden => "gp_class_warden.png".into(),
                    M0rIcon::ClassNecromancer => "gp_class_necromancer.png".into(),
                    M0rIcon::ClassTemplar => "gp_class_templar.png".into(),
                    M0rIcon::ClassArcanist => "gp_class_arcanist.png".into(),
                }
            },
            M0rTexture::Unknown(_p) => {
                "unknown.png".into()
            },
            M0rTexture::None => {"blank.png".into()},
        }
    }
}

pub fn m0r_texture_to_og(tex: &M0rTexture) -> String {
    match tex {
        M0rTexture::Known(icon) => match icon {
            M0rIcon::Circle => "^1".to_string(),
            M0rIcon::Hexagon => "^2".to_string(),
            M0rIcon::Square => "^3".to_string(),
            M0rIcon::Diamond => "^4".to_string(),
            M0rIcon::Octagon => "^5".to_string(),
            M0rIcon::Chevron => "^6".to_string(),
            M0rIcon::Blank => "^7".to_string(),
            M0rIcon::SharkPog => "^8".to_string(),
            M0rIcon::AllianceBadgeAldmeri => "^9".to_string(),
            M0rIcon::AllianceBadgeEbonheart => "^10".to_string(),
            M0rIcon::AllianceBadgeDaggerfall => "^11".to_string(),
            M0rIcon::RoleIconDPS => "^12".to_string(),
            M0rIcon::RoleIconTank => "^13".to_string(),
            M0rIcon::RoleIconHealer => "^14".to_string(),
            M0rIcon::ClassDragonknight => "^15".to_string(),
            M0rIcon::ClassSorcerer => "^16".to_string(),
            M0rIcon::ClassNightblade => "^17".to_string(),
            M0rIcon::ClassWarden => "^18".to_string(),
            M0rIcon::ClassNecromancer => "^19".to_string(),
            M0rIcon::ClassTemplar => "^20".to_string(),
            M0rIcon::ClassArcanist => "^21".to_string(),
        },
        M0rTexture::Unknown(s) => s.clone(),
        M0rTexture::None => "^7".to_string(),
    }
}

pub struct SVGData {
    pub path: String,
    pub view_box: String,
}


pub fn get_svg(tex: &M0rTexture) -> Option<SVGData> {
    match tex {
        M0rTexture::Known(icon) => match icon {
            M0rIcon::Circle => Some(SVGData {
                path: "M127.5 252.5c69.036 0 125-55.964 125-125 0-69.0356-55.964-125-125-125-69.0356 0-125 55.9644-125 125 0 69.036 55.9644 125 125 125Z".to_string(),
                view_box: "0 0 255 255".to_string(),
            }),
            M0rIcon::Hexagon => Some(SVGData {
                path: "M64 223.5h127l64-110-64-111H64l-64 111z".to_string(),
                view_box: "0 0 256 226".to_string(),
            }),
            M0rIcon::Square => Some(SVGData {
                path: "M3 3v249h249V3z".to_string(),
                view_box: "0 0 256 256".to_string(),
            }),
            M0rIcon::Diamond => Some(SVGData {
                path: "M0 128 128 0l127 128-127 127z".to_string(),
                view_box: "0 0 256 256".to_string(),
            }),
            M0rIcon::Octagon => Some(SVGData {
                path: "M2.5 178.277V75.7233L75.7233 2.5H178.277L251.5 75.7233V178.277L178.277 251.5H75.7233z".to_string(),
                view_box: "0 0 254 254".to_string(),
            }),
            M0rIcon::Chevron => Some(SVGData {
                path: "M0 100-125 0v-100L0 0l125-100V0z".to_string(),
                view_box: "-128 -128 256 256".to_string(),
            }),
            _ => None,
        },
        _ => None,
    }
}
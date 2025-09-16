use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}};

use regex::Regex;

use crate::zone::{Map, Zone};

#[derive(Debug, Clone)]
pub struct MarkerFlat {
    pub position: Position3D,
    pub icon: MarkerIcon,
    pub size: u8,
    pub active: bool,
    pub id: u16,
    pub map_id: u16,
    pub format: MarkerFormat,
}

impl PartialEq for MarkerFlat {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
        && self.icon == other.icon
        && self.active == other.active
        && self.size == other.size
        && self.format == other.format
        && self.map_id == other.map_id
    }
}

impl Eq for MarkerFlat {}

impl Hash for MarkerFlat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.icon.hash(state);
        self.active.hash(state);
        self.format.hash(state);
        self.size.hash(state);
        self.map_id.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct BreadcrumbLine {
    pub position1: Position3D,
    pub position2: Position3D,
    pub active: bool,
    pub colour: (u8, u8, u8),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarkerFormat {
    Bitrock,
    Akamatsu,
    M0R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkerIcon {
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

pub const ALL_ICONS: &[&str] = &[
    "1.png", "2.png", "3.png", "4.png", "5.png", "6.png", "7.png", "8.png", "9.png", "10.png", "11.png", "12.png",
    "arrow.png", "squares/marker_lightblue.png", "squares/square_pink.png", "squares/squaretwo_pink.png", "squares/square_yellow.png", "squares/squaretwo_yellow.png",
    "squares/square_blue.png", "squares/squaretwo_blue.png", "squares/squaretwo_blue_one.png", "squares/squaretwo_blue_two.png", "squares/squaretwo_blue_three.png", "squares/squaretwo_blue_four.png",
    "squares/square_green.png", "squares/squaretwo_green.png", "squares/squaretwo_green_one.png", "squares/squaretwo_green_two.png", "squares/squaretwo_green_three.png", "squares/squaretwo_green_four.png",
    "squares/square_orange_OT.png", "squares/square_orange.png", "squares/squaretwo_orange.png", "squares/squaretwo_orange_one.png", "squares/squaretwo_orange_two.png", "squares/squaretwo_orange_three.png", "squares/squaretwo_orange_four.png",
    "squares/square_red_MT.png", "squares/square_red.png", "squares/squaretwo_red.png", "squares/squaretwo_red_one.png", "squares/squaretwo_red_two.png", "squares/squaretwo_red_three.png", "squares/squaretwo_red_four.png",
    "a.png", "b.png", "c.png", "d.png", "e.png", "f.png", "g.png", "h.png", "i.png", "j.png", "k.png", "l.png", "m.png", "n.png", "o.png", "p.png", "q.png", "r.png", "s.png", "t.png", "u.png", "v.png", "w.png", "x.png", "y.png", "z.png",
    "sharkpog.png", 
    //"unknown.png", // inaccessible
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

pub fn parse_elms_string(elms_string: &str, zones: Vec<Zone>) -> HashMap<u16, Vec<MarkerFlat>> {
    let re = Regex::new(r"/(?P<zone>\d+)//(?P<x>\d+),(?P<y>\d+),(?P<z>\d+),(?P<icon>\d+)/").unwrap();
    let mut result: HashMap<u16, Vec<MarkerFlat>> = HashMap::new();
    let mut seen: HashMap<u16, HashSet<MarkerFlat>> = HashMap::new();
    let mut id_counter = 0;

    for caps in re.captures_iter(elms_string) {
        let zone_id: u16 = caps["zone"].parse().unwrap();
        let x: i32 = caps["x"].parse().unwrap();
        let y: i32 = caps["y"].parse().unwrap();
        let z: i32 = caps["z"].parse().unwrap();
        let icon_number: u16 = caps["icon"].parse().unwrap();
        let icon_enum = MarkerIcon::try_from(icon_number).unwrap_or(MarkerIcon::Unknown);
        let icon = icon_enum.into();

        if let Some(zone_obj) = zones.iter().find(|zone| zone.id == zone_id) {
            let best_map = find_best_map(x, y, z, zone_obj);

            let marker = MarkerFlat {
                position: Position3D { x, y, z },
                icon,
                size: 1,
                active: true,
                id: id_counter,
                format: MarkerFormat::Bitrock,
                map_id: best_map.map_or(0, |m| m.map_id),
            };

            let entry_set = seen.entry(zone_id).or_default();
            if entry_set.insert(marker.clone()) {
                result.entry(zone_id).or_default().push(marker);
                id_counter += 1;
            }
        }
    }

    for markers in result.values_mut() {
        markers.sort_by_key(|m| -> u16 { (&m.icon).into() });
    }

    result
}

pub fn markers_to_elms_string(markers_by_zone: &HashMap<u16, Vec<MarkerFlat>>) -> String {
    let mut all_zones: Vec<u16> = markers_by_zone.keys().cloned().collect();
    all_zones.sort();

    let mut result = String::new();
    for zone in all_zones {
        if let Some(markers) = markers_by_zone.get(&zone) {
            let mut active: Vec<_> = markers.iter().filter(|m| m.active).collect();
            active.sort_by_key(|m| -> u16 { (&m.icon).into() });

            for m in active {
                result.push_str(&format!(
                    "/{}//{},{},{},{}/",
                    zone,
                    m.position.x,
                    m.position.y,
                    m.position.z,
                    u16::from(&m.icon),
                ));
            }
        }
    }

    result.trim().to_string()
}

fn hex_to_rgb(hex: u32) -> (u8, u8, u8) {
    let r = (hex >> 16) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    (r, g, b)
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
        let mut colours: Vec<(u8, u8, u8)> = Vec::new();
        for _ in 0..colour_count {
            if let Some(hex_str) = parts.next() {
                if let Ok(hex_val) = u32::from_str_radix(hex_str.trim(), 16) {
                    colours.push(hex_to_rgb(hex_val));
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

        let mut colours: Vec<(u8, u8, u8)> = Vec::new();
        for line in lines {
            if !colours.contains(&line.colour) {
                colours.push(line.colour);
            }
        }
        result.push_str(&format!("{:X};", colours.len()));
        for &(r, g, b) in &colours {
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

impl From<u16> for MarkerIcon {
    fn from(n: u16) -> Self {
        match n {
            1..=12 => MarkerIcon::Num(n as u8),
            13 => MarkerIcon::Arrow,
            14 => MarkerIcon::MarkerLightBlue, // this is very definitely GREEN but whatever 
            15 => MarkerIcon::SquareBlue,
            16 => MarkerIcon::SquareGreen,
            17 => MarkerIcon::SquareOrange,
            18 => MarkerIcon::SquareOrangeOT,
            19 => MarkerIcon::SquarePink,
            20 => MarkerIcon::SquareRed,
            21 => MarkerIcon::SquareRedMT,
            22 => MarkerIcon::SquareYellow,
            23 => MarkerIcon::SquareTwoBlue,
            24 => MarkerIcon::SquareTwoBlueOne,
            25 => MarkerIcon::SquareTwoBlueTwo,
            26 => MarkerIcon::SquareTwoBlueThree,
            27 => MarkerIcon::SquareTwoBlueFour,
            28 => MarkerIcon::SquareTwoGreen,
            29 => MarkerIcon::SquareTwoGreenOne,
            30 => MarkerIcon::SquareTwoGreenTwo,
            31 => MarkerIcon::SquareTwoGreenThree,
            32 => MarkerIcon::SquareTwoGreenFour,
            33 => MarkerIcon::SquareTwoOrange,
            34 => MarkerIcon::SquareTwoOrangeOne,
            35 => MarkerIcon::SquareTwoOrangeTwo,
            36 => MarkerIcon::SquareTwoOrangeThree,
            37 => MarkerIcon::SquareTwoOrangeFour,
            38 => MarkerIcon::SquareTwoPink,
            39 => MarkerIcon::SquareTwoRed,
            40 => MarkerIcon::SquareTwoRedOne,
            41 => MarkerIcon::SquareTwoRedTwo,
            42 => MarkerIcon::SquareTwoRedThree,
            43 => MarkerIcon::SquareTwoRedFour,
            44 => MarkerIcon::SquareTwoYellow,
            45..=70 => MarkerIcon::Letter((b'a' + (n - 45) as u8) as char),
            71 => MarkerIcon::SharkPog,
            _ => MarkerIcon::Unknown,
        }
    }
}

impl From<&MarkerIcon> for u16 {
    fn from(marker_icon: &MarkerIcon) -> u16 {
        match marker_icon {
            MarkerIcon::Num(n) if (1..=12).contains(n) => *n as u16,
            MarkerIcon::Arrow => 13,
            MarkerIcon::MarkerLightBlue => 14,
            MarkerIcon::SquareBlue => 15,
            MarkerIcon::SquareGreen => 16,
            MarkerIcon::SquareOrange => 17,
            MarkerIcon::SquareOrangeOT => 18,
            MarkerIcon::SquarePink => 19,
            MarkerIcon::SquareRed => 20,
            MarkerIcon::SquareRedMT => 21,
            MarkerIcon::SquareYellow => 22,
            MarkerIcon::SquareTwoBlue => 23,
            MarkerIcon::SquareTwoBlueOne => 24,
            MarkerIcon::SquareTwoBlueTwo => 25,
            MarkerIcon::SquareTwoBlueThree => 26,
            MarkerIcon::SquareTwoBlueFour => 27,
            MarkerIcon::SquareTwoGreen => 28,
            MarkerIcon::SquareTwoGreenOne => 29,
            MarkerIcon::SquareTwoGreenTwo => 30,
            MarkerIcon::SquareTwoGreenThree => 31,
            MarkerIcon::SquareTwoGreenFour => 32,
            MarkerIcon::SquareTwoOrange => 33,
            MarkerIcon::SquareTwoOrangeOne => 34,
            MarkerIcon::SquareTwoOrangeTwo => 35,
            MarkerIcon::SquareTwoOrangeThree => 36,
            MarkerIcon::SquareTwoOrangeFour => 37,
            MarkerIcon::SquareTwoPink => 38,
            MarkerIcon::SquareTwoRed => 39,
            MarkerIcon::SquareTwoRedOne => 40,
            MarkerIcon::SquareTwoRedTwo => 41,
            MarkerIcon::SquareTwoRedThree => 42,
            MarkerIcon::SquareTwoRedFour => 43,
            MarkerIcon::SquareTwoYellow => 44,
            MarkerIcon::Letter(c) if c.is_ascii_lowercase() => 45 + (*c as u8 - b'a') as u16,
            MarkerIcon::SharkPog => 71,
            MarkerIcon::Unknown => 14,
            _ => 14,
        }
    }
}

impl From<&str> for MarkerIcon {
    fn from(s: &str) -> Self {
        match s {
            "1.png" => MarkerIcon::Num(1),
            "2.png" => MarkerIcon::Num(2),
            "3.png" => MarkerIcon::Num(3),
            "4.png" => MarkerIcon::Num(4),
            "5.png" => MarkerIcon::Num(5),
            "6.png" => MarkerIcon::Num(6),
            "7.png" => MarkerIcon::Num(7),
            "8.png" => MarkerIcon::Num(8),
            "9.png" => MarkerIcon::Num(9),
            "10.png" => MarkerIcon::Num(10),
            "11.png" => MarkerIcon::Num(11),
            "12.png" => MarkerIcon::Num(12),
            "arrow.png" => MarkerIcon::Arrow,
            "squares/marker_lightblue.png" => MarkerIcon::MarkerLightBlue,
            "squares/square_blue.png" => MarkerIcon::SquareBlue,
            "squares/square_green.png" => MarkerIcon::SquareGreen,
            "squares/square_orange.png" => MarkerIcon::SquareOrange,
            "squares/square_orange_OT.png" => MarkerIcon::SquareOrangeOT,
            "squares/square_pink.png" => MarkerIcon::SquarePink,
            "squares/square_red.png" => MarkerIcon::SquareRed,
            "squares/square_red_MT.png" => MarkerIcon::SquareRedMT,
            "squares/square_yellow.png" => MarkerIcon::SquareYellow,
            "squares/squaretwo_blue.png" => MarkerIcon::SquareTwoBlue,
            "squares/squaretwo_blue_one.png" => MarkerIcon::SquareTwoBlueOne,
            "squares/squaretwo_blue_two.png" => MarkerIcon::SquareTwoBlueTwo,
            "squares/squaretwo_blue_three.png" => MarkerIcon::SquareTwoBlueThree,
            "squares/squaretwo_blue_four.png" => MarkerIcon::SquareTwoBlueFour,
            "squares/squaretwo_green.png" => MarkerIcon::SquareTwoGreen,
            "squares/squaretwo_green_one.png" => MarkerIcon::SquareTwoGreenOne,
            "squares/squaretwo_green_two.png" => MarkerIcon::SquareTwoGreenTwo,
            "squares/squaretwo_green_three.png" => MarkerIcon::SquareTwoGreenThree,
            "squares/squaretwo_green_four.png" => MarkerIcon::SquareTwoGreenFour,
            "squares/squaretwo_orange.png" => MarkerIcon::SquareTwoOrange,
            "squares/squaretwo_orange_one.png" => MarkerIcon::SquareTwoOrangeOne,
            "squares/squaretwo_orange_two.png" => MarkerIcon::SquareTwoOrangeTwo,
            "squares/squaretwo_orange_three.png" => MarkerIcon::SquareTwoOrangeThree,
            "squares/squaretwo_orange_four.png" => MarkerIcon::SquareTwoOrangeFour,
            "squares/squaretwo_pink.png" => MarkerIcon::SquareTwoPink,
            "squares/squaretwo_red.png" => MarkerIcon::SquareTwoRed,
            "squares/squaretwo_red_one.png" => MarkerIcon::SquareTwoRedOne,
            "squares/squaretwo_red_two.png" => MarkerIcon::SquareTwoRedTwo,
            "squares/squaretwo_red_three.png" => MarkerIcon::SquareTwoRedThree,
            "squares/squaretwo_red_four.png" => MarkerIcon::SquareTwoRedFour,
            "squares/squaretwo_yellow.png" => MarkerIcon::SquareTwoYellow,
            "sharkpog.png" => MarkerIcon::SharkPog,
            s if s.len() == 5 && s.ends_with(".png") && s.chars().next().unwrap().is_ascii_lowercase() => {
                MarkerIcon::Letter(s.chars().next().unwrap())
            }
            _ => MarkerIcon::Unknown,
        }
    }
}

impl From<MarkerIcon> for String {
    fn from(marker_icon: MarkerIcon) -> Self {
        match marker_icon {
            MarkerIcon::Num(n) => format!("{}.png", n),
            MarkerIcon::Arrow => "arrow.png".into(),
            MarkerIcon::MarkerLightBlue => "squares/marker_lightblue.png".into(),
            MarkerIcon::SquareBlue => "squares/square_blue.png".into(),
            MarkerIcon::SquareGreen => "squares/square_green.png".into(),
            MarkerIcon::SquareOrange => "squares/square_orange.png".into(),
            MarkerIcon::SquareOrangeOT => "squares/square_orange_OT.png".into(),
            MarkerIcon::SquarePink => "squares/square_pink.png".into(),
            MarkerIcon::SquareRed => "squares/square_red.png".into(),
            MarkerIcon::SquareRedMT => "squares/square_red_MT.png".into(),
            MarkerIcon::SquareYellow => "squares/square_yellow.png".into(),
            MarkerIcon::SquareTwoBlue => "squares/squaretwo_blue.png".into(),
            MarkerIcon::SquareTwoBlueOne => "squares/squaretwo_blue_one.png".into(),
            MarkerIcon::SquareTwoBlueTwo => "squares/squaretwo_blue_two.png".into(),
            MarkerIcon::SquareTwoBlueThree => "squares/squaretwo_blue_three.png".into(),
            MarkerIcon::SquareTwoBlueFour => "squares/squaretwo_blue_four.png".into(),
            MarkerIcon::SquareTwoGreen => "squares/squaretwo_green.png".into(),
            MarkerIcon::SquareTwoGreenOne => "squares/squaretwo_green_one.png".into(),
            MarkerIcon::SquareTwoGreenTwo => "squares/squaretwo_green_two.png".into(),
            MarkerIcon::SquareTwoGreenThree => "squares/squaretwo_green_three.png".into(),
            MarkerIcon::SquareTwoGreenFour => "squares/squaretwo_green_four.png".into(),
            MarkerIcon::SquareTwoOrange => "squares/squaretwo_orange.png".into(),
            MarkerIcon::SquareTwoOrangeOne => "squares/squaretwo_orange_one.png".into(),
            MarkerIcon::SquareTwoOrangeTwo => "squares/squaretwo_orange_two.png".into(),
            MarkerIcon::SquareTwoOrangeThree => "squares/squaretwo_orange_three.png".into(),
            MarkerIcon::SquareTwoOrangeFour => "squares/squaretwo_orange_four.png".into(),
            MarkerIcon::SquareTwoPink => "squares/squaretwo_pink.png".into(),
            MarkerIcon::SquareTwoRed => "squares/squaretwo_red.png".into(),
            MarkerIcon::SquareTwoRedOne => "squares/squaretwo_red_one.png".into(),
            MarkerIcon::SquareTwoRedTwo => "squares/squaretwo_red_two.png".into(),
            MarkerIcon::SquareTwoRedThree => "squares/squaretwo_red_three.png".into(),
            MarkerIcon::SquareTwoRedFour => "squares/squaretwo_red_four.png".into(),
            MarkerIcon::SquareTwoYellow => "squares/squaretwo_yellow.png".into(),
            MarkerIcon::Letter(c) => format!("{}.png", c),
            MarkerIcon::SharkPog => "sharkpog.png".into(),
            MarkerIcon::Unknown => "unknown.png".into(),
        }
    }
}
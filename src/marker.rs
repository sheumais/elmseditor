use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}, str::from_utf8};

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
    pub orientation: Option<(i8, i8)>,
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
            None::<(i8, i8)>.hash(state);
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

pub fn get_marker_map_id(m: &Marker) -> u16 {
    match m {
        Marker::Elms(marker) => {marker.map_id},
        Marker::M0r(marker) => {marker.map_id}
    }
}

pub fn get_marker_position(m: &Marker) -> Position3D {
    match m {
        Marker::Elms(marker) => {marker.position},
        Marker::M0r(marker) => {marker.position}
    }
}

pub fn get_marker_icon(m: &Marker) -> MarkerIcon {
    match m {
        Marker::Elms(marker) => {MarkerIcon::Elms(marker.icon)},
        Marker::M0r(marker) => {MarkerIcon::M0r(marker.background_texture.clone())}
    }
}

pub fn set_marker_active(m: &mut Marker, a: bool) {
    match m {
        Marker::Elms(marker) => {marker.active = a},
        Marker::M0r(marker) => {marker.active = a}
    }
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

pub enum MarkerIcon {
    Elms(ElmsIcon),
    M0r(M0rTexture)
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

pub fn markers_to_elms_string(markers_by_zone: &HashMap<u16, Vec<Marker>>) -> String {
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

fn hex_to_rgba(hex: u32) -> (u8, u8, u8, u8) {
    if hex <= 0x00FF_FFFF {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        (r, g, b, 255)
    } else {
        let r = (hex >> 24) as u8;
        let g = ((hex >> 16) & 0xFF) as u8;
        let b = ((hex >> 8) & 0xFF) as u8;
        let a = (hex & 0xFF) as u8;
        (r, g, b, a)
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

pub fn parse_m0r_string(m0r_string: &str, zones: Vec<Zone>) -> HashMap<u16, Vec<Marker>> {
    let mut result: HashMap<u16, Vec<Marker>> = HashMap::new();
    /* <1552]
    1760161167]
    8f39:7d20:11381]
    2:1,3,4,5,6,17;1.9:13;1.5:8,14,15,16;2.5:7;5:12]
    0:12,17;-71:10;-70:14;-60:11;-75:16;-90:9,13,15]
    82:9;261:14;326:16;279:13;181:17;63:12;45:11;263:15;31:10]
    ffffff:9,10,11;ffcc00:12,13,14;00ffa6:1,2,3,4,5,6,7,15;ff00e6:8,16,17]
    ^6:1,3,4,5,6,7;^14:12;^7:9,10,11,13,14,15,16,17;^4:2,8]
    419d:1051:11b9:,235d:bc3:343f:Jump off here,1dea:300:214d:#1,8e5:2b3:28a8:#2,1586:0:11a3:#3,7ec:30c:c6c:#4,4e94:33d:443:,534d:34c:0:SECRET\nBOSS,34fc:e38:36f1:Keep running off of\nboth cliffs ahead,18cb:4e6:2d98:Follow the chevrons\nPull each pack,9ea:147:1370:Go up and to the left\nStack on the heals on the\ntop of the hill,0:6fb:daa:,7d:4dc:fc9:Place destro ultimate down there ^,3501:e0:362:Keep running forwards.\nDon't stop.\nGo to the secret boss.,4b75:2f0:3d2:Everything will stack\non the chevron ^,60c5:731:1b4f:Healer slot taunt!,40f5:fe7:3365:----->

    */
    let pattern = regex::Regex::new(r"<(.*?)\](.*?)\](.*?)\](.*?)\](.*?)\](.*?)\](.*?)\](.*?)\](.*?)>").unwrap();

    let caps = match pattern.captures(m0r_string) {
        Some(c) => c,
        None => {
            eprintln!("Malformed m0r marker string");
            return result;
        }
    };

    let zone_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
    let timestamp = caps.get(2).map(|m| m.as_str()).unwrap_or("");
    let mins = caps.get(3).map(|m| m.as_str()).unwrap_or("");
    let sizes = caps.get(4).map(|m| m.as_str()).unwrap_or("");
    let pitch = caps.get(5).map(|m| m.as_str()).unwrap_or("");
    let yaw = caps.get(6).map(|m| m.as_str()).unwrap_or("");
    let colour = caps.get(7).map(|m| m.as_str()).unwrap_or("");
    let texture = caps.get(8).map(|m| m.as_str()).unwrap_or("");
    let positions = caps.get(9).map(|m| m.as_str()).unwrap_or("");

    let zone_id: u16 = zone_str.parse().unwrap_or(0);
    let zone = match zones.iter().find(|z| z.id.to_string() == zone_str) {
        Some(z) => z,
        None => {
            return result;
        }
    };

    let mins_parts: Vec<&str> = mins.split(':').collect();
    let min_x = i32::from_str_radix(mins_parts.get(0).unwrap_or(&"0"), 16).unwrap_or(0);
    let min_y = i32::from_str_radix(mins_parts.get(1).unwrap_or(&"0"), 16).unwrap_or(0);
    let min_z = i32::from_str_radix(mins_parts.get(2).unwrap_or(&"0"), 16).unwrap_or(0);

    let mut markers: Vec<M0rMarker> = Vec::new();

    for chunk in positions.split(',') {
        let parts: Vec<&str> = chunk.split(':').collect();
        if parts.len() < 3 {
            continue;
        }

        let cx = i32::from_str_radix(parts[0], 16).unwrap_or(0) + min_x;
        let cy = i32::from_str_radix(parts[1], 16).unwrap_or(0) + min_y;
        let cz = i32::from_str_radix(parts[2], 16).unwrap_or(0) + min_z;

        let map = find_best_map(cx, cy, cz, zone).unwrap();

        let text_raw = parts.get(3).map(|t| *t).unwrap_or("");
        let mut unescaped_text = text_raw
            .replace(from_utf8(MOR_COLON).unwrap(), ":")
            .replace(from_utf8(MOR_COMMA).unwrap(), ",")
            .replace(from_utf8(MOR_SQUAREBRACKET).unwrap(), "]")
            .replace(from_utf8(MOR_SEMICOLON).unwrap(), ";")
            .replace(from_utf8(MOR_GREATERTHAN).unwrap(), ">");

        if unescaped_text.is_empty() {
            unescaped_text = String::new();
        }

        let new_marker = M0rMarker {
            id: markers.len() as u16,
            map_id: map.map_id,
            active: true,
            position: Position3D {
                x: cx as i32,
                y: cy as i32,
                z: cz as i32,
            },
            background_texture: M0rTexture::None,
            text: unescaped_text.into(),
            size: 1.0,
            colour: (255, 255, 255, 255),
            orientation: None,
        };

        markers.push(new_marker);
    }

    for segment in sizes.split(';') {
        if let Some((size_str, idx_str)) = segment.split_once(':') {
            if let Ok(size) = size_str.parse::<f32>() {
                for idx in idx_str.split(',') {
                    if let Ok(i) = idx.parse::<usize>() {
                        if let Some(icon) = markers.get_mut(i) {
                            icon.size = size;
                        }
                    }
                }
            }
        }
    }

    for segment in colour.split(';') {
        if let Some((hex, idx_str)) = segment.split_once(':') {
            if let Ok(hex_val) = u32::from_str_radix(hex, 16) {
                let floats = hex_to_rgba(hex_val);
                for idx in idx_str.split(',') {
                    if let Ok(i) = idx.parse::<usize>() {
                        if let Some(icon) = markers.get_mut(i) {
                            icon.colour = floats;
                        }
                    }
                }
            }
        }
    }

    for segment in pitch.split(';') {
        if let Some((p_str, idx_str)) = segment.split_once(':') {
            if let Ok(pitch) = p_str.parse::<i8>() {
                for idx in idx_str.split(',') {
                    if let Ok(i) = idx.parse::<usize>() {
                        if let Some(icon) = markers.get_mut(i) {
                            icon.orientation.unwrap_or((0, 0)).0 = pitch;
                        }
                    }
                }
            }
        }
    }

    for segment in yaw.split(';') {
        if let Some((y_str, idx_str)) = segment.split_once(':') {
            if let Ok(yaw) = y_str.parse::<i8>() {
                for idx in idx_str.split(',') {
                    if let Ok(i) = idx.parse::<usize>() {
                        if let Some(icon) = markers.get_mut(i) {
                            icon.orientation.unwrap_or((0, 0)).1 = yaw;
                        }
                    }
                }
            }
        }
    }

    for segment in texture.split(';') {
        if let Some((tex_str, idx_str)) = segment.split_once(':') {
            for idx in idx_str.split(',') {
                if let Ok(i) = idx.parse::<usize>() {
                    if let Some(icon) = markers.get_mut(i) {
                        icon.background_texture = M0rTexture::from(tex_str);
                    }
                }
            }
        }
    }

    let returned_markers = markers.iter().map(|i| Marker::M0r(i.clone())).collect();

    result.insert(zone.id, returned_markers);
    println!("{result:?}");
    result
}

pub fn build_m0r_string(markers_by_zone: &HashMap<u16, Vec<Marker>>) -> String {
    let mut result = String::new();
    let zones: Vec<u16> = markers_by_zone.keys().cloned().collect();

    for (zone_id, markers) in markers_by_zone {
        let zone = match zones.iter().find(|z| **z == *zone_id) {
            Some(z) => z,
            None => continue,
        };

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

        let timestamp = "0";

        let mut sizes: Vec<String> = Vec::new();
        let mut pitches: Vec<String> = Vec::new();
        let mut yaws: Vec<String> = Vec::new();
        let mut colours: Vec<String> = Vec::new();
        let mut textures: Vec<String> = Vec::new();

        let mut size_groups: HashMap<String, Vec<usize>> = HashMap::new();
        let mut pitch_groups: HashMap<i8, Vec<usize>> = HashMap::new();
        let mut yaw_groups: HashMap<i8, Vec<usize>> = HashMap::new();
        let mut colour_groups: HashMap<u32, Vec<usize>> = HashMap::new();
        let mut texture_groups: HashMap<String, Vec<usize>> = HashMap::new();

        let mut positions = Vec::new();

        for (i, marker) in markers.iter().enumerate() {
            match marker {
                Marker::M0r(marker) => {
                    let pos = marker.position;
                    let cx = (pos.x - min_x) as u32;
                    let cy = (pos.y - min_y) as u32;
                    let cz = (pos.z - min_z) as u32;

                    let mut text = marker.text.clone().unwrap_or("".to_string());
                    text = text
                        .replace(":", from_utf8(MOR_COLON).unwrap())
                        .replace(",", from_utf8(MOR_COMMA).unwrap())
                        .replace("]", from_utf8(MOR_SQUAREBRACKET).unwrap())
                        .replace(";", from_utf8(MOR_SEMICOLON).unwrap())
                        .replace(">", from_utf8(MOR_GREATERTHAN).unwrap())
                        .replace("\n", "\\n");

                    positions.push(format!("{:X}:{:X}:{:X}:{}", cx, cy, cz, text));

                    size_groups.entry(marker.size.to_string()).or_default().push(i);

                    if let Some((p, y)) = marker.orientation {
                        pitch_groups.entry(p).or_default().push(i);
                        yaw_groups.entry(y).or_default().push(i);
                    }

                    let rgba = marker.colour;
                    let rgba_hex = ((rgba.0 as u32) << 24)
                        | ((rgba.1 as u32) << 16)
                        | ((rgba.2 as u32) << 8)
                        | (rgba.3 as u32);
                    colour_groups.entry(rgba_hex).or_default().push(i);
                    match &marker.background_texture {
                        M0rTexture::Unknown(texture) => texture_groups.entry(texture.to_string()).or_default().push(i),
                        _ => {},
                    }
                }
                _ => (),
            }

            // match &marker.background_texture() {
            //     M0rTexture::Unknown(tex) => texture_groups.entry(tex.clone()).or_default().push(i),
            //     _ => {}
            // }
        }

        for (size, idxs) in size_groups {
            let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            sizes.push(format!("{}:{}", size, idx_str));
        }
        for (pitch, idxs) in pitch_groups {
            let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            pitches.push(format!("{}:{}", pitch, idx_str));
        }
        for (yaw, idxs) in yaw_groups {
            let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            yaws.push(format!("{}:{}", yaw, idx_str));
        }
        for (hex, idxs) in colour_groups {
            let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            colours.push(format!("{:X}:{}", hex, idx_str));
        }
        for (tex, idxs) in texture_groups {
            let idx_str = idxs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            textures.push(format!("{}:{}", tex, idx_str));
        }

        result.push_str(&format!(
            "<{}]{}]{}]{}]{}]{}]{}]{}]{}>",
            zone,
            timestamp,
            mins,
            sizes.join(";"),
            pitches.join(";"),
            yaws.join(";"),
            colours.join(";"),
            textures.join(";"),
            positions.join(","),
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
                    M0rIcon::Circle => "circle.png".into(),
                    M0rIcon::Hexagon => "hexagon.png".into(),
                    M0rIcon::Square => "square.png".into(),
                    M0rIcon::Diamond => "diamond.png".into(),
                    M0rIcon::Octagon => "octagon.png".into(),
                    M0rIcon::Chevron => "chevron.png".into(),
                    M0rIcon::SharkPog => "sharkpog.png".into(),
                    M0rIcon::AllianceBadgeAldmeri => "alliancebadge_aldmeri.png".into(),
                    M0rIcon::AllianceBadgeEbonheart => "alliancebadge_ebonheart.png".into(),
                    M0rIcon::AllianceBadgeDaggerfall => "alliancebadge_daggerfall.png".into(),
                    M0rIcon::RoleIconDPS => "lfg_roleicon_dps".into(),
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
            M0rTexture::Unknown(p) => {
                "../unknown.png".into()
            },
            M0rTexture::None => {"blank.png".into()},
        }
    }
}
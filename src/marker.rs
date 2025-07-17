use std::hash::{Hash, Hasher};

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
    }
}

impl Eq for MarkerFlat {}

impl Hash for MarkerFlat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.icon.hash(state);
        self.active.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarkerFormat {
    Bitrock,
    Akamatsu
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

impl From<u16> for MarkerIcon {
    fn from(n: u16) -> Self {
        match n {
            1..=12 => MarkerIcon::Num(n as u8),
            13 => MarkerIcon::Arrow,
            14 => MarkerIcon::MarkerLightBlue,
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
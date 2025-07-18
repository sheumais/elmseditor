use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Zone {
    pub id: u16,
    pub maps: Vec<Map>,
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Map {
    pub name: String,
    pub tiles: Vec<Tile>,
    pub map_id: u16,
    pub zone_id: u16,
    pub count: u8,
    pub scale_data: MapScaleData,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tile {
    pub path: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MapScaleData {
    pub scale_factor: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

pub struct MapMeta {
    pub name: &'static str,
    pub slug: &'static str, // lowercase
    pub map_id: u16, 
    pub zone_id: u16,
    pub count: u8,
    pub map_scale_data: MapScaleData,
}

const MAP_METAS: &[MapMeta] = &[
    MapMeta {
        name: "Asylum Atrium",
        map_id: 1391,
        zone_id: 1000,
        count: 3,
        slug: "clockwork/ui_map_asylumsanctorum001_base_",
        map_scale_data: MapScaleData { scale_factor: 0.0000210217, min_x: 63360.0, max_x: 110930.0, min_z: 75410.0, max_z: 122980.0 }
    },
    MapMeta {
        name: "Upper Level",
        map_id: 1392,
        zone_id: 1000,
        count: 3,
        slug: "clockwork/ui_map_asylumsanctorum002_base_",
        map_scale_data: MapScaleData { scale_factor: 0.0000312500, min_x: 84629.0, max_x: 116629.0, min_z: 83199.0, max_z: 115199.0 }
    },
    MapMeta {
        name: "Cloudrest",
        map_id: 1502,
        zone_id: 1051,
        count: 3,
        slug: "summerset/ui_map_cloudresttrial_base_",
        map_scale_data: MapScaleData { scale_factor: 0.0000174606, min_x: 118653.0, max_x: 196202.0, min_z: 51100.0, max_z: 128648.0 }
    },
    MapMeta {
        name: "Dreadsail Beach",
        map_id: 2164,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_beach_01_",
        map_scale_data: MapScaleData { scale_factor: 0.0000123963, min_x: 8461.0, max_x: 89130.0, min_z: 120141.0, max_z: 200811.0 }
    },
    MapMeta {
        name: "(Twins) Bloodsport Arena",
        map_id: 2165,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_boss1_map_",
        map_scale_data: MapScaleData { scale_factor: 0.0000389544, min_x: 57121.0, max_x: 82792.0, min_z: 71757.0, max_z: 97428.0 }
    },
    MapMeta {
        name: "Reef Warren",
        map_id: 2166,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_doors_map_",
        map_scale_data: MapScaleData { scale_factor: 0.0000262208, min_x: 93316.0, max_x: 131453.0, min_z: 83429.0, max_z: 121567.0 }
    },
    MapMeta {
        name: "(Bird) Tempest Heights",
        map_id: 2179,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_e_map_",
        map_scale_data: MapScaleData { scale_factor: 0.0000111884, min_x: 110272.0, max_x: 199650.0, min_z: 110266.0, max_z: 199645.0 }
    },
    MapMeta {
        name: "(Crab) Reef Caverns",
        map_id: 2180,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_w_map_",
        map_scale_data: MapScaleData { scale_factor: 0.0000144686, min_x: 4439.0, max_x: 73554.0, min_z: -4129.0, max_z: 64986.0 }
    },
    MapMeta {
        name: "(Reef Guardian) Coral Cavern",
        map_id: 2181,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_b2_map_",
        map_scale_data: MapScaleData { scale_factor: 0.0000372149, min_x: 159011.0, max_x: 185882.0, min_z: 69115.0, max_z: 95986.0 }
    },
    MapMeta {
        name: "Coral Cavern Whorlpools",
        map_id: 2182,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_b2under_map_",
        map_scale_data: MapScaleData { scale_factor: 0.0000372149, min_x: 159051.0, max_x: 185922.0, min_z: 69115.0, max_z: 95986.0 }
    },
    MapMeta {
        name: "Fleet Queen's Parlors",
        map_id: 2183,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_v_map_", // game straight up lied to me. it was actually in elsweyr folder ???
        map_scale_data: MapScaleData { scale_factor: 0.0000177090, min_x: 89102.0, max_x: 145570.0, min_z: 9548.0, max_z: 66016.0 }
    },
    MapMeta {
        name: "(Taleria) Coral Caldera",
        map_id: 2184,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_b3_map_",
        map_scale_data: MapScaleData { scale_factor: 0.0000380036, min_x: 156781.0, max_x: 183094.0, min_z: 14641.0, max_z: 40954.0 }
    },
    MapMeta {
        name: "Kyne's Aegis",
        map_id: 1805,
        zone_id: 1196,
        slug: "skyrim/kynesaegismap001_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000084731, min_x: 44399.0, max_x: 162419.0, min_z: 35279.0, max_z: 153299.0 },
    },
    MapMeta {
        name: "(Falgravn) Ruins",
        map_id: 1806,
        zone_id: 1196,
        slug: "skyrim/kynesaegisboss3floor001_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000980392, min_x: 73919.0, max_x: 84119.0, min_z: 51039.0, max_z: 61259.0 },
    },
    MapMeta {
        name: "(Floor 2) Hidden Barrow",
        map_id: 1807,
        zone_id: 1196,
        slug: "skyrim/kynesaegisboss3floor002_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000980392, min_x: 73919.0, max_x: 84119.0, min_z: 51039.0, max_z: 61259.0 }, // WRONG
    },
    MapMeta {
        name: "(Floor 3) Ritual Vault",
        map_id: 1808,
        zone_id: 1196,
        slug: "skyrim/kynesaegisboss3floor003_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000783699, min_x: 72660.0, max_x: 85420.0, min_z: 49719.0, max_z: 62519.0 },
    },
    MapMeta {
        name: "Lucent Citadel",
        map_id: 2552,
        zone_id: 1478,
        slug: "deadlands/u42tri_lucentcitmap001_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000092868, min_x: 64733.0, max_x: 172413.0, min_z: 78056.0, max_z: 185736.0 }
    },
    MapMeta {
        name: "(Shapers) The Marred Path",
        map_id: 2687,
        zone_id: 1548,
        slug: "coldharbour/osscage_section1map002_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000115969, min_x: 144318.0, max_x: 230548.0, min_z: 29010.0, max_z: 115240.0 }
    },
    MapMeta {
        name: "(Twins) Quarreler's Quarry",
        map_id: 2688,
        zone_id: 1548,
        slug: "coldharbour/osscage_section2map003_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000157973, min_x: 57954.0, max_x: 121256.0, min_z: 96323.0, max_z: 159625.0 }
    },
    MapMeta {
        name: "The Wormgut",
        map_id: 2689,
        zone_id: 1548,
        slug: "coldharbour/osscage_section3map004_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000125188, min_x: 141042.0, max_x: 220922.0, min_z: 138302.0, max_z: 218181.0 }
    },
    MapMeta {
        name: "(Kazpian) The Mangled Court",
        map_id: 2690,
        zone_id: 1548,
        slug: "coldharbour/osscage_boss3map005_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000249333, min_x: 30815.0, max_x: 70922.0, min_z: 179812.0, max_z: 219919.0 }
    },
    MapMeta {
        name: "Inscrutable Lichyard",
        map_id: 2691,
        zone_id: 1548,
        slug: "coldharbour/osscage_secret1map006_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000396817, min_x: 63168.0, max_x: 88368.0, min_z: 12901.0, max_z: 38101.0 }
    },
    MapMeta {
        name: "Gaol of Transition",
        map_id: 2692,
        zone_id: 1548,
        slug: "coldharbour/osscage_secret2map007_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000485714, min_x: 64572.0, max_x: 85160.0, min_z: 64639.0, max_z: 85227.0 }
    },
    MapMeta {
        name: "Sitient Lair",
        map_id: 2693,
        zone_id: 1548,
        slug: "coldharbour/osscage_secret3map008_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000526761, min_x: 15508.0, max_x: 34491.0, min_z: 65574.0, max_z: 84558.0 }
    },
    MapMeta {
        name: "Ancient City of Rockgrove",
        map_id: 2004,
        zone_id: 1263,
        slug: "blackwood/u30_rg_map_outside_001_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000125125, min_x: 59680.0, max_x: 139600.0, min_z: 43400.0, max_z: 123320.0 }
    },
    // MapMeta {
    //     name: "Tower of the Five Crimes",
    //     map_id: 2005,
    //     zone_id: 1263,
    //     slug: "blackwood/u30_rg_map_outside_002_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: 0.0000526761, min_x: 15508.0, max_x: 34491.0, min_z: 65574.0, max_z: 84558.0 }
    // },
    // MapMeta {
    //     name: "Xanmeer Corridors",
    //     map_id: 2012,
    //     zone_id: 1263,
    //     slug: "blackwood/rg_map_inside_001_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: 0.0000526761, min_x: 15508.0, max_x: 34491.0, min_z: 65574.0, max_z: 84558.0 }
    // },
    // MapMeta {
    //     name: "Sanity's Edge",
    //     map_id: 2330,
    //     zone_id: 1427,
    //     slug: "telvanni/sanitysedgesection3_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Sanity's Edge",
    //     map_id: 2331,
    //     zone_id: 1427,
    //     slug: "telvanni/sanitysedgeboss2_map_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Sanity's Edge",
    //     map_id: 2332,
    //     zone_id: 1427,
    //     slug: "telvanni/sanitysedgeboss1_map_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Sanity's Edge",
    //     map_id: 2333,
    //     zone_id: 1427,
    //     slug: "telvanni/sanitysedgesection0_map_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Sanity's Edge",
    //     map_id: 2372,
    //     zone_id: 1427,
    //     slug: "telvanni/se_alinor_",
    //     count: 5,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Sanity's Edge",
    //     map_id: 2373,
    //     zone_id: 1427,
    //     slug: "telvanni/se_orsinium_",
    //     count: 5,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    MapMeta {
        name: "Maw of Lorkhaj",
        map_id: 997,
        zone_id: 725,
        slug: "reapersmarch/maw_of_lorkaj_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000191064, min_x: 71481.0, max_x: 123819.0, min_z: 113086.0, max_z: 165424.0 }
    },
    MapMeta {
        name: "Suthay Sanctuary",
        map_id: 999,
        zone_id: 725,
        slug: "reapersmarch/mawlorkajsuthaysanctuary_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000550497, min_x: 70411.0, max_x: 88576.0, min_z: 136138.0, max_z: 154304.0 }
    },
    MapMeta {
        name: "The High Lunarium",
        map_id: 1000,
        zone_id: 725,
        slug: "reapersmarch/mawlorkajsevenriddles_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000224066, min_x: 23734.0, max_x: 68364.0, min_z: 167379.0, max_z: 212009.0 }
    },
    MapMeta {
        name: "Sanctum Ophidia",
        map_id: 704,
        zone_id: 639,
        slug: "craglorn/trl_so_map01_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000132988, min_x: 72754.0, max_x: 147949.0, min_z: 126075.0, max_z: 201270.0 }
    },
    MapMeta {
        name: "Sanctum Ophidia",
        map_id: 705,
        zone_id: 639,
        slug: "craglorn/trl_so_map02_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000113779, min_x: 66794.0, max_x: 154684.0, min_z: 103904.0, max_z: 191794.0 }
    },
    // MapMeta {
    //     name: "Sanctum Ophidia",
    //     map_id: 706,
    //     zone_id: 639,
    //     slug: "craglorn/trl_so_map03_base_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: 0.0000029280, min_x: 23734.0, max_x: 68364.0, min_z: 167379.0, max_z: 212009.0 }
    // },
    // MapMeta {
    //     name: "Sanctum Ophidia",
    //     map_id: 707,
    //     zone_id: 639,
    //     slug: "craglorn/trl_so_map04_base_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: 0.0000224066, min_x: 23734.0, max_x: 68364.0, min_z: 167379.0, max_z: 212009.0 }
    // },
    MapMeta {
        name: "Sunspire Temple Grounds",
        map_id: 1649,
        zone_id: 1121,
        slug: "elsweyr/sunspireoverworld_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000100348, min_x: 54438.0, max_x: 154091.0, min_z: 47127.0, max_z: 146780.0 }
    },
    // MapMeta {
    //     name: "Sunspire Temple Vestibule",
    //     map_id: 1650,
    //     zone_id: 1121,
    //     slug: "elsweyr/sunspirehall001_base_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: 0.0000435114, min_x: 96878.0, max_x: 119861.0, min_z: 41634.0, max_z: 64617.0 }
    // },
    MapMeta {
        name: "Chancel of Alkosh Vestibule",
        map_id: 1651,
        zone_id: 1121,
        slug: "elsweyr/sunspirehall002_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000712216, min_x: 98753.0, max_x: 112794.0, min_z: 68647.0, max_z: 82688.0 }
    },
    // MapMeta {
    //     name: "(Fire) Shrine of Jode Vestibule",
    //     map_id: 1652,
    //     zone_id: 1121,
    //     slug: "elsweyr/sunspirehall003_base_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: 0.0000522478, min_x: 85200.0, max_x: 104339.0, min_z: 71046.0, max_z: 90186.0 }
    // },
    // MapMeta {
    //     name: "(Ice) Shrine of Jode Vestibule",
    //     map_id: 1653,
    //     zone_id: 1121,
    //     slug: "elsweyr/sunspirehall004_base_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: 0.0000443433, min_x: 104695.0, max_x: 127247.0, min_z: 72921.0, max_z: 95472.0 }
    // },
    MapMeta {
        name: "(Ice) Shrine of Jone",
        map_id: 1655,
        zone_id: 1121,
        slug: "elsweyr/sunspireroom001_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000814427, min_x: 168937.0, max_x: 181216.0, min_z: 163876.0, max_z: 176155.0 }
    },
    MapMeta {
        name: "(Fire) Shrine of Jone",
        map_id: 1657,
        zone_id: 1121,
        slug: "elsweyr/sunspireroom002_base_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000814427, min_x: 23863.0, max_x: 36142.0, min_z: 168844.0, max_z: 181122.0 }
    },
];

fn populate_map_data() -> Vec<Map> {
    MAP_METAS
        .iter()
        .map(|m| {
            let total = (m.count as usize).pow(2);
            let tiles = (0..total)
                .map(|i| {
                    Tile {
                        path: format!("{}{}.png", m.slug, i),
                    }
                })
                .collect();

            Map {
                name: m.name.to_string(),
                map_id: m.map_id,
                zone_id: m.zone_id,
                count: m.count,
                tiles,
                scale_data: m.map_scale_data.clone(),
            }
        })
        .collect()
}

pub fn populate_zone_data() -> Vec<Zone> {
    let mut zone_map: BTreeMap<u16, Vec<Map>> = BTreeMap::new();

    for map in populate_map_data() {
        zone_map.entry(map.zone_id)
            .or_default()
            .push(map);
    }

    zone_map.into_iter()
        .map(|(zone_id, mut maps)| {
            maps.sort_by_key(|m| m.map_id);
            Zone {
                id: zone_id,
                maps,
                name: match zone_id {
                    636 => "Hel Ra Citadel".to_string(),
                    638 => "Aetherian Archive".to_string(),
                    639 => "Sanctum Ophidia".to_string(),
                    725 => "Maw of Lorkhaj".to_string(),
                    975 => "Halls of Fabrication".to_string(),
                    1000 => "Asylum Sanctorium".to_string(),
                    1051 => "Cloudrest".to_string(),
                    1121 => "Sunspire".to_string(),
                    1196 => "Kyne's Aegis".to_string(),
                    1263 => "Rockgrove".to_string(),
                    1344 => "Dreadsail Reef".to_string(),
                    1427 => "Sanity's Edge".to_string(),
                    1478 => "Lucent Citadel".to_string(),
                    1548 => "Ossein Cage".to_string(),
                    _ => "Unknown Trial Map".to_string(),
                }
            }
        })
        .collect()
}
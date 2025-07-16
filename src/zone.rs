use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Zone {
    pub id: u16,
    pub maps: Vec<Map>,
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
        name: "Beach",
        map_id: 2164,
        zone_id: 1344,
        count: 3,
        slug: "systres/dsr_beach_01_",
        map_scale_data: MapScaleData { scale_factor: 0.0000123963, min_x: 8461.0, max_x: 89130.0, min_z: 120141.0, max_z: 200811.0 }
    },
    // MapMeta {
    //     name: "(Twins) Bloodsport Arena",
    //     map_id: 2165,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_boss1_map_",
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Reef Warren",
    //     map_id: 2166,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_doors_map_",
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "(Bird) Tempest Heights",
    //     map_id: 2179,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_e_map_",
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "(Crab) Reef Caverns",
    //     map_id: 2179,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_w_map_",
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "(Reef Guardian) Coral Cavern",
    //     map_id: 2183,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_b2_map_",
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Coral Cavern Whorlpools",
    //     map_id: 2184,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_b2under_map_",
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "Fleet Queen's Parlors",
    //     map_id: 2181,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_v_map_", -- game straight up lied to me. it was actually in elsweyr folder ???
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    // MapMeta {
    //     name: "(Taleria) Coral Caldera",
    //     map_id: 2182,
    //     zone_id: 1344,
    //     count: 3,
    //     slug: "systres/dsr_b3_map_",
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () }
    // },
    MapMeta {
        name: "Kyne's Aegis",
        map_id: 1805,
        zone_id: 1196,
        slug: "skyrim/kynesaegismap001_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000142439, min_x: -686955.0, max_x: -616749.0, min_z: 1555830.0, max_z: 1626035.0 },
    },
    // MapMeta {
    //     name: "(Floor 1) Ruins of Kyne's Aegis",
    //     map_id: 1806,
    //     zone_id: 1196,
    //     slug: "skyrim/kynesaegisboss3floor001_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () },
    // },
    // MapMeta {
    //     name: "(Floor 2) Kyne's Aegis's Hidden Barrow",
    //     map_id: 1807,
    //     zone_id: 1196,
    //     slug: "skyrim/kynesaegisboss3floor002_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () },
    // },
    // MapMeta {
    //     name: "(Floor 3) Kyne's Aegis's Ritual Vault",
    //     map_id: 1808,
    //     zone_id: 1196,
    //     slug: "skyrim/kynesaegisboss3floor003_",
    //     count: 3,
    //     map_scale_data: MapScaleData { scale_factor: (), min_x: (), max_x: (), min_z: (), max_z: () },
    // },
    MapMeta {
        name: "Lucent Citadel",
        map_id: 2552,
        zone_id: 1478,
        slug: "deadlands/u42tri_lucentcitmap001_",
        count: 3,
        map_scale_data: MapScaleData { scale_factor: 0.0000092868, min_x: 64733.0, max_x: 172413.0, min_z: 78056.0, max_z: 185736.0 }
    },
    MapMeta {
        name: "Trash & Shapers",
        map_id: 2687,
        zone_id: 1548,
        slug: "coldharbour/osscage_section1map002_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000115969, min_x: 144318.0, max_x: 230548.0, min_z: 29010.0, max_z: 115240.0 }
    },
    MapMeta {
        name: "Trash & Twins",
        map_id: 2688,
        zone_id: 1548,
        slug: "coldharbour/osscage_section2map003_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000157973, min_x: 57954.0, max_x: 121256.0, min_z: 96323.0, max_z: 159625.0 }
    },
    MapMeta {
        name: "Trash",
        map_id: 2689,
        zone_id: 1548,
        slug: "coldharbour/osscage_section3map004_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000125188, min_x: 141042.0, max_x: 220922.0, min_z: 138302.0, max_z: 218181.0 }
    },
    MapMeta {
        name: "Kazpian",
        map_id: 2690,
        zone_id: 1548,
        slug: "coldharbour/osscage_boss3map005_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000249333, min_x: 30815.0, max_x: 70922.0, min_z: 179812.0, max_z: 219919.0 }
    },
    MapMeta {
        name: "Secret Boss 1",
        map_id: 2691,
        zone_id: 1548,
        slug: "coldharbour/osscage_secret1map006_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000396817, min_x: 63168.0, max_x: 88368.0, min_z: 12901.0, max_z: 38101.0 }
    },
    MapMeta {
        name: "Secret Boss 2",
        map_id: 2692,
        zone_id: 1548,
        slug: "coldharbour/osscage_secret2map007_",
        count: 4,
        map_scale_data: MapScaleData { scale_factor: 0.0000485714, min_x: 64572.0, max_x: 85160.0, min_z: 64639.0, max_z: 85227.0 }
    },
    MapMeta {
        name: "Secret Boss 3",
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
            }
        })
        .collect()
}
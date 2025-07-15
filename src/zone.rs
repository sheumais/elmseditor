use std::collections::{BTreeMap, HashMap};

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
    pub x_scale_factor: f32,
    pub z_scale_factor: f32,
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
        map_scale_data: MapScaleData { x_scale_factor: 0.0000210217, z_scale_factor: 0.0000210217, min_x: 63360f32, max_x: 110930f32, min_z: 75410f32, max_z: 122980f32 }
    },
    MapMeta {
        name: "Asylum Atrium Upper Level",
        map_id: 1392,
        zone_id: 1000,
        count: 3,
        slug: "clockwork/ui_map_asylumsanctorum002_base_",
        map_scale_data: MapScaleData { x_scale_factor: 0.0000312500, z_scale_factor: 0.0000312500, min_x: 84629f32, max_x: 116629f32, min_z: 83199f32, max_z: 115199f32 }
    },
    MapMeta {
        name: "Cloudrest",
        map_id: 1502,
        zone_id: 1051,
        count: 3,
        slug: "summerset/ui_map_cloudresttrial_base_",
        map_scale_data: MapScaleData { x_scale_factor: 0.0000174606, z_scale_factor: 0.0000174606, min_x: 118653f32, max_x: 196202f32, min_z: 51100f32, max_z: 128648f32 }
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
            }
        })
        .collect()
}
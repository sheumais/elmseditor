#[derive(Clone, PartialEq)]
pub struct Zone {
    pub name: String,
    pub tiles: Vec<Tile>,
    pub id: u16, 
    pub count: u8,
    pub scale_data: ZoneScaleData,
}

#[derive(Clone, PartialEq)]
pub struct Tile {
    pub path: String,
}

#[derive(Clone, PartialEq)]
pub struct ZoneScaleData {
    pub x_scale_factor: f32,
    pub z_scale_factor: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

pub struct ZoneMeta {
    pub name: &'static str,
    pub slug: &'static str,
    pub id: u16, 
    pub count: u8,
    pub zone_scale_data: ZoneScaleData,
}

const ZONE_METAS: &[ZoneMeta] = &[
    ZoneMeta {
        name: "Asylum Sanctorium",
        id: 1000,
        count: 3,
        slug: "clockwork/ui_map_asylumsanctorum001_base_",
        zone_scale_data: ZoneScaleData { x_scale_factor: 0.0000210217, z_scale_factor: 0.0000210217, min_x: 63360f32, max_x: 110930f32, min_z: 75410f32, max_z: 122980f32 }
    },
];

pub fn populate_zone_data() -> Vec<Zone> {
    ZONE_METAS
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

            Zone {
                name: m.name.to_string(),
                id: m.id,
                count: m.count,
                tiles,
                scale_data: m.zone_scale_data.clone(),
            }
        })
        .collect()
}
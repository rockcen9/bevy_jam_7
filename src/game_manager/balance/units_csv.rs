use std::collections::HashMap;

use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};

use crate::{asset_tracking::LoadResource, prelude::*, screens::loading::LoadingScreen};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(CsvAssetPlugin::<UnitRow>::new(&["unit.csv"]));
    app.init_resource::<UnitStatsCache>();
    app.add_systems(OnEnter(LoadingScreen::Level), build_unit_stats_cache);

    app.load_resource::<UnitBalanceAssets>();
}

/// Cache of unit stats indexed by unit ID for fast lookup.
#[derive(Resource, Default, Reflect)]
pub struct UnitStatsCache {
    pub stats: HashMap<String, UnitRow>,
}

fn build_unit_stats_cache(
    mut cache: ResMut<UnitStatsCache>,
    unit_assets: Res<UnitBalanceAssets>,
    csv_assets: Res<Assets<LoadedCsv<UnitRow>>>,
) {
    let Some(loaded) = csv_assets.get(&unit_assets.units) else {
        warn!("UnitAssets CSV not loaded yet");
        return;
    };

    cache.stats.clear();
    for row in &loaded.rows {
        cache.stats.insert(row.id.clone(), row.clone());
    }
    info!("Built UnitStatsCache with {} entries", cache.stats.len());
}

#[derive(serde::Deserialize, Asset, Debug, Clone, Reflect)]
pub struct UnitRow {
    pub id: String,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub atk_speed: f32,
    pub move_speed: f32,
    pub range: f32,
    pub weight: f32,
    pub cost: i32,
    pub game_unit_name: String,
    pub desc: String,
    #[serde(deserialize_with = "deserialize_optional_unit_kind")]
    pub counter: Option<UnitKind>,
    pub unity_type: UnitKind,
}

fn deserialize_optional_unit_kind<'de, D>(deserializer: D) -> Result<Option<UnitKind>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.trim() {
        "" => Ok(None),
        "Shield" => Ok(Some(UnitKind::Shield)),
        "Spear" => Ok(Some(UnitKind::Spear)),
        "Archer" => Ok(Some(UnitKind::Archer)),
        "Cavalry" => Ok(Some(UnitKind::Cavalry)),
        other => Err(serde::de::Error::unknown_variant(
            other,
            &["Shield", "Spear", "Archer", "Cavalry"],
        )),
    }
}

#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct UnitBalanceAssets {
    #[dependency]
    pub(crate) units: Handle<LoadedCsv<UnitRow>>,
}

impl FromWorld for UnitBalanceAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            units: assets.load("balance/all.unit.csv"),
        }
    }
}

use std::collections::HashMap;

use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};

use crate::{asset_tracking::LoadResource, prelude::*, screens::loading::LoadingScreen};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(CsvAssetPlugin::<MemoryRow>::new(&["memory.csv"]));
    app.init_resource::<MemoryStatsCache>();
    app.add_systems(OnEnter(LoadingScreen::Level), build_memory_stats_cache);

    app.load_resource::<MemoryBalanceAssets>();
}

/// Cache of memory stats indexed by memory ID for fast lookup.
#[derive(Resource, Default, Reflect)]
pub struct MemoryStatsCache {
    pub stats: HashMap<String, MemoryRow>,
}

fn build_memory_stats_cache(
    mut cache: ResMut<MemoryStatsCache>,
    memory_assets: Res<MemoryBalanceAssets>,
    csv_assets: Res<Assets<LoadedCsv<MemoryRow>>>,
) {
    let Some(loaded) = csv_assets.get(&memory_assets.memories) else {
        warn!("MemoryAssets CSV not loaded yet");
        return;
    };

    cache.stats.clear();
    for row in &loaded.rows {
        cache.stats.insert(row.id.clone(), row.clone());
    }
    info!("Built MemoryStatsCache with {} entries", cache.stats.len());
}

#[derive(serde::Deserialize, Asset, Debug, Clone, Reflect)]
pub struct MemoryRow {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub description: String,
}

#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct MemoryBalanceAssets {
    #[dependency]
    pub(crate) memories: Handle<LoadedCsv<MemoryRow>>,
}

impl FromWorld for MemoryBalanceAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            memories: assets.load("balance/all.memory.csv"),
        }
    }
}

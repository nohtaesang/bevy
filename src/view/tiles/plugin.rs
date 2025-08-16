// src/view/tiles/plugin.rs
use bevy::prelude::*;
use bevy::ecs::schedule::common_conditions::{
    resource_added, resource_changed, on_event, // ← 추가
};

use crate::gameplay::tiles::{
    resources::{BaseTileMap, TileConfig},
    events::MapReinitialized,                  // ← 이제 진짜로 사용됨
};
use super::{
    resources::TileViewConfig,
    systems::{
        spawn_tiles_on_map_added,
        rebuild_tiles_on_reinit,
        sync_tiles_on_cfg_change,
        apply_view_palette_change,
    },
};

pub struct TilesViewPlugin;

impl Plugin for TilesViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TileViewConfig>()
            .add_systems(Update, spawn_tiles_on_map_added.run_if(resource_added::<BaseTileMap>))
            // ✅ 이벤트가 있을 때만 리빌드 시스템 실행
            .add_systems(Update, rebuild_tiles_on_reinit.run_if(on_event::<MapReinitialized>))
            .add_systems(Update, sync_tiles_on_cfg_change.run_if(resource_changed::<TileConfig>))
            .add_systems(Update, apply_view_palette_change.run_if(resource_changed::<TileViewConfig>));
    }
}

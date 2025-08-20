use bevy::prelude::*;

use super::{
    components::{Map, MapSize, TerrainKind},
    events::{MapInitRequested, MapInitializedApplied},
    grid_index::OccupancyIndex,
};
use crate::app::{
    Phase,
    state::{AppState, ModeState},
};

/// Internal resource to bridge Apply→Publish
#[derive(Resource, Default)]
struct MapJustInitialized(Option<MapSize>);

/// Config for demo. Replace later with your real loader.
#[derive(Resource)]
pub struct MapConfig {
    pub w: u32,
    pub h: u32,
}

#[derive(Resource, Default)]
struct MapBootstrap {
    sent: bool,
} // 요청 1회 보장
pub struct MapDomainPlugin;

impl Plugin for MapDomainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapConfig { w: 20, h: 12 })
            .init_resource::<MapBootstrap>()
            .init_resource::<MapJustInitialized>()
            .add_event::<MapInitRequested>()
            .add_event::<MapInitializedApplied>()
            // ★ Input: 첫 Update 프레임에 요청 1번만 보냄
            .add_systems(
                Update,
                emit_map_init_request_once
                    .in_set(Phase::Input)
                    .run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
            )
            // ★ Apply: SSOT(Map/OccupancyIndex) 생성
            .add_systems(
                Update,
                apply_map_init
                    .in_set(Phase::Apply)
                    .run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
            )
            // ★ Publish: Applied 이벤트 방출
            .add_systems(
                Update,
                publish_map_initialized
                    .in_set(Phase::Publish)
                    .run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
            );
    }
}

fn emit_map_init_request_once(
    mut ev: EventWriter<MapInitRequested>,
    mut boot: ResMut<MapBootstrap>,
    cfg: Res<MapConfig>,
    existing_map: Option<Res<Map>>,
) {
    if boot.sent || existing_map.is_some() {
        return;
    }
    boot.sent = true;
    ev.write(MapInitRequested {
        size: MapSize { w: cfg.w, h: cfg.h },
    });
}

fn apply_map_init(
    mut commands: Commands,
    mut ev_req: EventReader<MapInitRequested>,
    mut just: ResMut<MapJustInitialized>,
    existing_map: Option<Res<Map>>,
) {
    if existing_map.is_some() {
        return;
    }

    if let Some(MapInitRequested { size }) = ev_req.read().last().copied() {
        // Create SSOT resources
        let map = Map::new(size, TerrainKind::Ground);
        let occ = OccupancyIndex::new(size);
        commands.insert_resource(map);
        commands.insert_resource(occ);
        just.0 = Some(size);
    }
}

fn publish_map_initialized(
    mut ev_out: EventWriter<MapInitializedApplied>,
    mut just: ResMut<MapJustInitialized>,
) {
    if let Some(size) = just.0.take() {
        ev_out.write(MapInitializedApplied { size });
    }
}

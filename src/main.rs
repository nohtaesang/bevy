use bevy::prelude::*;
use bevy_game::app::plugin::AppPlugins;
use bevy_game::app::state::ModeState;
use bevy_game::domain::map::components::Map;
use bevy_game::domain::map::plugin::MapDomainPlugin;
use bevy_game::infra::view_core::camera::plugin::CameraPlugin;
use bevy_game::infra::view_core::coords::CoordsPlugin;
use bevy_game::modes::battle::features::map_view::plugin::MapViewPlugin;
use bevy_game::modes::battle::features::overlays::hover_tile::plugin::HoverTilePlugin;
use bevy_game::modes::battle::features::spawner_debug::plugin::SpawnerDebugPlugin;
use bevy_game::modes::battle::features::selection::plugin::SelectionPlugin;
use bevy_game::modes::battle::features::overlays::move_range::plugin::MoveRangeOverlayPlugin;

fn main() {
    App::new()
    // 0) 기본/앱 스케줄 먼저
    .add_plugins((DefaultPlugins, AppPlugins))
    .init_state::<ModeState>()
    .insert_state(ModeState::Battle)

    // 1) Domain (SSOT 생성/갱신)
    .add_plugins(MapDomainPlugin)

    // 2) Infra (카메라/좌표계 등 공용 인프라)
    .add_plugins((CameraPlugin, CoordsPlugin))

    // 3) Mode/Features (뷰·오버레이·선택·디버그 등)
    .add_plugins((
        MapViewPlugin,
        HoverTilePlugin,
        SelectionPlugin,
        MoveRangeOverlayPlugin,
        SpawnerDebugPlugin,
    ))

    .add_systems(Update, log_map_once)
    .run();
}

fn log_map_once(map: Option<Res<Map>>, mut done: Local<bool>) {
    if *done {
        return;
    }
    if let Some(map) = map {
        info!(
            "Map ready: {}x{} (tiles={})",
            map.size.w,
            map.size.h,
            map.tiles.len()
        );
        *done = true;
    }
}

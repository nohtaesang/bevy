use bevy::prelude::*;
use bevy_game::app::plugin::AppPlugins;
use bevy_game::app::state::ModeState;
use bevy_game::domain::map::components::Map;
use bevy_game::domain::map::plugin::MapDomainPlugin;
use bevy_game::infra::view_core::camera::plugin::CameraPlugin;
use bevy_game::modes::battle::features::map_view::plugin::MapViewPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AppPlugins)) // ★ 여기서 Phase 세트 구성
        .init_state::<ModeState>()
        .insert_state(ModeState::Battle)
        .add_plugins((MapDomainPlugin, CameraPlugin, MapViewPlugin))
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

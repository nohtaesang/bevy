// examples/sandbox.rs
use bevy::prelude::*;
use bevy_game::app::plugin::AppStatesPlugin;
use bevy_game::view::plugin::ViewPlugin;
use bevy_game::input::InputPlugin;
use bevy_game::gameplay::plugin::GameplayPlugin;

use bevy_game::gameplay::tiles::prelude::{TileConfig, BaseTileMap, TerrainKind, GridIndex, GridPos};
use bevy_game::gameplay::units::assets::{PendingUnitLoads, enqueue_unit};
use bevy_game::app::state::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppStatesPlugin)
        .add_plugins(ViewPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(GameplayPlugin) // Tiles/Interaction/Units가 한 번에 묶여 있음
        .add_systems(OnEnter(AppState::Battle), demo_setup_level_10x10)
        .add_systems(OnEnter(AppState::Battle), spawn_one_ally)
        .run();
}

// === 데모: 10x10 맵 구성 ===
fn demo_setup_level_10x10(mut commands: Commands) {
    let (w, h) = (10, 10);
    let cfg = TileConfig { cell_size: Vec2::new(32.0, 32.0), origin: Vec2::ZERO };
    commands.insert_resource(cfg);
    commands.insert_resource(BaseTileMap::filled(w, h, TerrainKind::Ground));
    commands.insert_resource(GridIndex::new(w, h));
}

// === 데모: 유닛 1기 스폰 ===
fn spawn_one_ally(asset_server: Res<AssetServer>, mut pending: ResMut<PendingUnitLoads>) {
    enqueue_unit(&asset_server, &mut pending, "units/ally.ron", GridPos::new(2, 2));
}

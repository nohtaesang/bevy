mod app;
mod gameplay;
mod view;
mod input;     

use bevy::prelude::*;
use crate::app::plugin::AppStatesPlugin; use crate::gameplay::assets::{enqueue_unit, PendingUnitLoads};
// 너가 만든 plugin.rs
use crate::gameplay::tiles::prelude::{GridPos, TilesPlugin};
use crate::app::state::AppState;
use crate::gameplay::tiles::prelude::{TileConfig, BaseTileMap, TerrainKind, GridIndex};
use crate::view::plugin::ViewPlugin;
use crate::input::InputPlugin;
use crate::gameplay::interaction::InteractionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)   // Bevy 기본 플러그인
        .add_plugins(AppStatesPlugin)  // 네가 만든 AppStatesPlugin
        .add_plugins(TilesPlugin)
        .add_plugins(ViewPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(InteractionPlugin)
        .add_plugins(gameplay::units::UnitsPlugin)
        .add_systems(OnEnter(AppState::Battle), demo_setup_level_10x10)
        .add_systems(OnEnter(AppState::Battle), spawn_one_ally)
        .run();
}   

fn demo_setup_level_10x10(mut commands: Commands) {
    let (w, h) = (10, 10);
    // 셀 크기 32x32, 좌표 원점 (0,0)
    let cfg = TileConfig { cell_size: Vec2::new(32.0, 32.0), origin: Vec2::ZERO };

    commands.insert_resource(cfg);
    commands.insert_resource(BaseTileMap::filled(w, h, TerrainKind::Ground));
    commands.insert_resource(GridIndex::new(w, h));
}

fn spawn_one_ally(
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingUnitLoads>,
) {
    enqueue_unit(&asset_server, &mut pending, "units/ally.ron", GridPos::new(2, 2));
}

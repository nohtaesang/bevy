mod app;
mod gameplay;
mod view;
mod input;     

use bevy::prelude::*;
use crate::app::plugin::AppStatesPlugin; // 너가 만든 plugin.rs
use crate::gameplay::tiles::prelude::TilesPlugin;
use crate::app::state::AppState;
use crate::gameplay::tiles::prelude::{TileConfig, BaseTileMap, TerrainKind, GridIndex};
use crate::view::plugin::ViewPlugin;
use crate::input::InputPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)   // Bevy 기본 플러그인
        .add_plugins(AppStatesPlugin)  // 네가 만든 AppStatesPlugin
        .add_plugins(TilesPlugin)
        .add_plugins(ViewPlugin)
        .add_plugins(InputPlugin)
        .add_systems(OnEnter(AppState::Battle), setup_level_10x10)
        .run();
}   

fn setup_level_10x10(mut commands: Commands) {
    let (w, h) = (10, 10);
    // 셀 크기 32x32, 좌표 원점 (0,0)
    let cfg = TileConfig { cell_size: Vec2::new(32.0, 32.0), origin: Vec2::ZERO };

    commands.insert_resource(cfg);
    commands.insert_resource(BaseTileMap::filled(w, h, TerrainKind::Ground));
    commands.insert_resource(GridIndex::new(w, h));
}
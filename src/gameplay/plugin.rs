use bevy::prelude::*;
use crate::app::state::AppState;

// 타일/보드 관련
use crate::gameplay::tiles::prelude::{
    TilesPlugin, GridPos, TileConfig, BaseTileMap, TerrainKind, GridIndex,
};

// 상호작용 & 유닛
use crate::gameplay::interaction::InteractionPlugin;
use crate::gameplay::units::UnitsPlugin;
use crate::gameplay::units::assets::{enqueue_unit, PendingUnitLoads};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // 하위 도메인 플러그인들
            .add_plugins(TilesPlugin)
            .add_plugins(InteractionPlugin)
            .add_plugins(UnitsPlugin)
            // 전투 씬 진입 시 셋업/스폰
            .add_systems(OnEnter(AppState::Battle), demo_setup_level_10x10)
            .add_systems(OnEnter(AppState::Battle), spawn_one_ally);
    }
}

// === 데모: 10x10 맵 구성 ===
fn demo_setup_level_10x10(mut commands: Commands) {
    let (w, h) = (10, 10);
    // 셀 크기 32x32, 좌표 원점 (0,0)
    let cfg = TileConfig { cell_size: Vec2::new(32.0, 32.0), origin: Vec2::ZERO };

    commands.insert_resource(cfg);
    commands.insert_resource(BaseTileMap::filled(w, h, TerrainKind::Ground));
    commands.insert_resource(GridIndex::new(w, h));
}

// === 데모: 유닛 1기 스폰 ===
fn spawn_one_ally(
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingUnitLoads>,
) {
    enqueue_unit(&asset_server, &mut pending, "units/ally.ron", GridPos::new(2, 2));
}

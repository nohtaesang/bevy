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
        .add_plugins(GameplayPlugin) 
        .add_systems(OnEnter(AppState::Battle), demo_setup_level_10x10)
        .add_systems(OnEnter(AppState::Battle), spawn_one_ally.after(demo_setup_level_10x10)) 
        .add_systems(OnEnter(AppState::Battle), spawn_random_enemies.after(demo_setup_level_10x10)) // ⬅️ 추가
       
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


// === 데모: 적 4명 랜덤 스폰 ===
fn spawn_random_enemies(
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingUnitLoads>,
    map: Res<BaseTileMap>,
) {
    // 간단 LCG 난수
    struct SimpleRng(u64);
    impl SimpleRng {
        fn new(seed: u64) -> Self { Self(seed | 1) } // 짝수 회피
        fn next_u32(&mut self) -> u32 {
            // LCG: X_{n+1} = aX_n + c (64-bit)
            self.0 = self.0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            (self.0 >> 32) as u32
        }
        fn gen_range_u32(&mut self, low: u32, high: u32) -> u32 {
            debug_assert!(low < high);
            let span = (high - low) as u64;
            (low as u64 + (self.next_u32() as u64 % span)) as u32
        }
    }

    // 시간 기반 시드
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9E3779B97F4A7C15); // fallback
    let mut rng = SimpleRng::new(seed);

    // 4개 고유 좌표 선택 (아군 (2,2) 제외)
    use std::collections::HashSet;
    let mut picked: HashSet<GridPos> = HashSet::with_capacity(4);
    let forbidden = GridPos::new(2, 2);

    while picked.len() < 4 {
        let x = rng.gen_range_u32(0, map.w);
        let y = rng.gen_range_u32(0, map.h);
        let gp = GridPos::new(x, y);
        if gp == forbidden { continue; }
        picked.insert(gp);
    }

    for gp in picked {
        // 적 프리셋 파일 경로는 프로젝트에 맞게 조정 (예: "units/enemy.ron")
        enqueue_unit(&asset_server, &mut pending, "units/enemy.ron", gp);
    }
}

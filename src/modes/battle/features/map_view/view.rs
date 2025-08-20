// ===============================
// src/modes/battle/features/map_view/view.rs
// ===============================
use bevy::prelude::*;

use crate::domain::map::components::{Map, TerrainKind};
use crate::domain::map::events::MapInitializedApplied;

use crate::infra::view_core::z_index::ZLayer;
/// Marker for spawned tile sprites (so we can clear/rebuild)
#[derive(Component)]
pub struct TileSprite;

/// Simple view config for tile size & origin.
#[derive(Resource)]
pub struct TileViewConfig {
    pub cell_size: Vec2,   // world units per tile
    pub origin: Vec2,      // lower-left world origin for (0,0)
    pub tile_gap: f32,     // 타일 간 격자선 두께(월드 유닛) ← 추가!
    pub grid_color: Color, // 격자선 색 ← 추가!
}

impl Default for TileViewConfig {
    fn default() -> Self {
        Self {
            cell_size: Vec2::new(32.0, 32.0),
            origin: Vec2::ZERO,
            tile_gap: 1.0, // 1px 느낌(스케일 1 기준)
            grid_color: Color::srgb(0.35, 0.40, 0.48),
        }
    }
}

#[derive(Component)]
pub struct MapUnderlay;

pub fn spawn_tiles_on_map_initialized(
    mut commands: Commands,
    mut ev_in: EventReader<MapInitializedApplied>,
    map: Option<Res<Map>>,
    q_existing: Query<Entity, Or<(With<TileSprite>, With<MapUnderlay>)>>,
    cfg: Res<TileViewConfig>,
) {
    if ev_in.is_empty() {
        return;
    }
    let _ = ev_in.read().last();

    // 이전 타일/언더레이 제거
    for e in q_existing.iter() {
        commands.entity(e).despawn();
    }

    let Some(map) = map else {
        return;
    };
    let size = map.size;

    // === 2-1) 언더레이 (격자선 색이 비칠 바닥 한 장) ===
    let total_w = size.w as f32 * cfg.cell_size.x;
    let total_h = size.h as f32 * cfg.cell_size.y;
    let center = cfg.origin + Vec2::new(total_w * 0.5, total_h * 0.5);

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(total_w, total_h)),
            color: cfg.grid_color, // 이 색이 '선'으로 보임
            ..Default::default()
        },
        Transform::from_xyz(center.x, center.y, ZLayer::GroundDecal.z()),
        MapUnderlay,
        Name::new("map_underlay"),
    ));

    // === 2-2) 타일 스폰 (크기를 gap 만큼 줄임) ===
    let tile_size = Vec2::new(
        (cfg.cell_size.x - cfg.tile_gap).max(1.0),
        (cfg.cell_size.y - cfg.tile_gap).max(1.0),
    );

    for y in 0..size.h {
        for x in 0..size.w {
            let i = (x + y * size.w) as usize;
            let kind = map.tiles[i];

            let color = match kind {
                TerrainKind::Ground => Color::srgb(0.22, 0.25, 0.30),
                TerrainKind::Wall => Color::srgb(0.10, 0.10, 0.10),
            };

            let center = cfg.origin
                + Vec2::new(
                    x as f32 * cfg.cell_size.x + cfg.cell_size.x * 0.5,
                    y as f32 * cfg.cell_size.y + cfg.cell_size.y * 0.5,
                );

            commands.spawn((
                Sprite {
                    custom_size: Some(tile_size), // ★ gap 반영
                    color,
                    ..Default::default()
                },
                Transform::from_xyz(center.x, center.y, ZLayer::Tiles.z()),
                TileSprite,
                Name::new(format!("tile({},{})", x, y)),
            ));
        }
    }
}

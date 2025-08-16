// src/view/tiles/systems.rs
use bevy::prelude::*;

use crate::gameplay::tiles::{
    components::GridPos,
    resources::{BaseTileMap, TileConfig},
    events::MapReinitialized,
};

use super::{
    components::{TileSprite, TileSpriteBundle, TileVisual},
    resources::TileViewConfig,
};

use crate::view::z_index::layer; // 타일은 동일 Z로 배칭 유리

/// BaseTileMap 리소스가 '처음' 생긴 프레임에 전체 타일 스폰
pub fn spawn_tiles_on_map_added(
    mut commands: Commands,
    map: Res<BaseTileMap>,
    cfg: Res<TileConfig>,
    view: Res<TileViewConfig>,
    q_tiles: Query<Entity, With<TileSprite>>,
) {
    // 혹시 기존 타일이 남아있다면 정리
    despawn_all_tiles(&mut commands, &q_tiles);
    spawn_all_tiles(&mut commands, &map, &cfg, &view);
}

/// 맵 리사이즈/교체 이벤트가 오면 타일 리빌드
pub fn rebuild_tiles_on_reinit(
    mut commands: Commands,
    mut ev: EventReader<MapReinitialized>,
    map: Option<Res<BaseTileMap>>,
    cfg: Option<Res<TileConfig>>,
    view: Option<Res<TileViewConfig>>,
    q_tiles: Query<Entity, With<TileSprite>>,
) {
    // on_event::<MapReinitialized>()를 플러그인에서 걸었다면 굳이 필요 없지만,
    // 혹시를 대비해 커서를 앞으로만 진행해 둔다.
    for _ in ev.read() {}

    let (Some(map), Some(cfg), Some(view)) = (map, cfg, view) else { return; };

    despawn_all_tiles(&mut commands, &q_tiles);
    spawn_all_tiles(&mut commands, &map, &cfg, &view);
}

/// TileConfig가 바뀐 프레임에 '모든' 타일 위치/크기 동기화
pub fn sync_tiles_on_cfg_change(
    cfg: Res<TileConfig>,
    view: Res<TileViewConfig>,
    mut q: Query<(&GridPos, &mut Transform, &mut Sprite), With<TileSprite>>,
) {
    if !cfg.is_changed() { return; }

    let size = view.tile_size(cfg.cell_size);
    for (gp, mut tr, mut sp) in &mut q {
        tr.translation = cfg.grid_to_world_center(*gp, layer::TILES);
        sp.custom_size = Some(size);
    }
}

/// 팔레트/갭 등 TileViewConfig 변경 시 색/크기 동기화
pub fn apply_view_palette_change(
    view: Res<TileViewConfig>,
    cfg: Res<TileConfig>,
    mut q: Query<(&TileVisual, &mut Sprite), With<TileSprite>>,
) {
    if !view.is_changed() { return; }

    let size = view.tile_size(cfg.cell_size);
    for (visual, mut sp) in &mut q {
        sp.color = view.color(visual.kind);
        sp.custom_size = Some(size);
    }
}

// ---------- 내부 헬퍼 ----------

fn despawn_all_tiles(commands: &mut Commands, q_tiles: &Query<Entity, With<TileSprite>>) {
    for e in q_tiles.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn spawn_all_tiles(
    commands: &mut Commands,
    map: &BaseTileMap,
    cfg: &TileConfig,
    view: &TileViewConfig,
) {
    let size = view.tile_size(cfg.cell_size);

    for y in 0..map.h {
        for x in 0..map.w {
            let gp = GridPos::new(x, y);
            let kind = map.terrain_at(x, y);
            let color = view.color(kind);

            // 0.16: Sprite + Transform 직접 구성
            let sprite = Sprite {
                color,
                custom_size: Some(size),
                ..default()
            };
            let transform = Transform::from_translation(
                cfg.grid_to_world_center(gp, layer::TILES),
            );

            commands.spawn(TileSpriteBundle::new(
                gp,
                TileVisual::new(kind),
                sprite,
                transform,
            ));
        }
    }
}

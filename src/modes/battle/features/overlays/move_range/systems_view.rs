

// =============================================
// src/modes/battle/features/overlays/move_range/systems_view.rs
// =============================================
use bevy::prelude::*;
use crate::infra::view_core::z_index::ZLayer;
use crate::infra::view_core::coords::grid_to_world_center;
use crate::modes::battle::features::map_view::view::TileViewConfig;
use crate::domain::units::components::{TeamId, Unit};
use super::resources::{MoveRangeOverlay, MoveRangeConfig};


#[derive(Component)]
pub struct MoveRangeTileSprite;


/// 오버레이를 타일 스프라이트들로 동기화 (간단 구현: 매번 리스폰)
pub fn sync_move_range_overlay(
mut commands: Commands,
overlay: Res<MoveRangeOverlay>,
view: Option<Res<TileViewConfig>>,
units: Query<&TeamId, With<Unit>>,
q_existing: Query<Entity, With<MoveRangeTileSprite>>,
cfg: Res<MoveRangeConfig>,
mut last_key: Local<Option<Entity>>, // 선택 유닛 변화 감지용
) {
let Some(view) = view else { return; };


if !overlay.is_changed() && overlay.of_unit == *last_key { return; }
*last_key = overlay.of_unit;


// 기존 제거
for e in q_existing.iter() {
    commands.entity(e).despawn();
}


let Some(unit_e) = overlay.of_unit else { return; };
let color = match units.get(unit_e).ok() {
Some(TeamId::Ally) => cfg.ally_color,
Some(TeamId::Enemy) => cfg.enemy_color,
None => cfg.ally_color,
};


let size = view.cell_size * cfg.size_scale;


for gp in overlay.tiles.iter().copied() {
    let center = grid_to_world_center(gp, &view);
    commands.spawn((
        Transform::from_xyz(center.x, center.y, ZLayer::RangePreview.z()),
        Sprite { custom_size: Some(size), color, ..Default::default() },
        MoveRangeTileSprite,
        Name::new(format!("move_range@{},{}", gp.x, gp.y)),
    ));
}
}
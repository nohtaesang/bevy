// =============================================
// src/modes/battle/features/overlays/move_range/systems_apply.rs
// =============================================
use super::resources::MoveRangeOverlay;
use crate::domain::map::components::Map;
use crate::domain::map::grid_index::OccupancyIndex;
use crate::domain::units::components::{TeamId, Unit, UnitGrid, UnitMove};
use crate::modes::battle::features::overlays::move_range::utils::compute_reachable_teamaware;
use crate::modes::battle::features::selection::resources::Selected;
use bevy::prelude::*;

/// 선택 변경/점유 변경 시 이동 범위 재계산
pub fn recompute_move_range(
    selected: Res<Selected>,
    units: Query<(&UnitGrid, Option<&UnitMove>, &TeamId), With<Unit>>, // ⬅ 팀 포함
    teams: Query<&TeamId>, // ⬅ 점유 엔티티의 팀 조회용 (클로저에서 사용)
    map: Option<Res<Map>>,
    occ: Option<Res<OccupancyIndex>>,
    mut overlay: ResMut<MoveRangeOverlay>,
) {
    let (Some(map), Some(occ)) = (map, occ) else { 
        overlay.tiles.clear();
        overlay.of_unit = None;
        return; 
    };
    let Some(unit_e) = selected.unit else { 
        overlay.tiles.clear();
        overlay.of_unit = None;
        return; 
    };

    let (grid, mv, mover_team) = match units.get(unit_e) {
        Ok(v) => v,
        Err(_) => return,
    };
    let max_steps = mv.map(|m| m.max_steps).unwrap_or(5);

    // 점유 엔티티의 팀 조회 클로저
    let tiles = compute_reachable_teamaware(
        &map,
        &occ,
        grid.0,
        max_steps,
        *mover_team,
        |e| teams.get(e).ok().copied(), // ⬅ Some(TeamId) 또는 None(팀 없음/뷰 전용 등)
    );

    overlay.tiles = tiles;
    overlay.of_unit = Some(unit_e);
}

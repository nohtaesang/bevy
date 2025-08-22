// src/infra/view_core/coords/systems.rs
//! GridGeometry 동기화 시스템

use super::types::{make_grid_geometry, GridGeometry};
use crate::domain::map::components::Map;
use crate::modes::battle::features::map_view::view::TileViewConfig;
use bevy::prelude::*;

/// Map과 TileViewConfig 변경 시 GridGeometry를 자동 갱신
pub fn sync_grid_geometry(
    mut grid_geom: ResMut<GridGeometry>,
    map: Option<Res<Map>>,
    view_cfg: Option<Res<TileViewConfig>>,
) {
    let (Some(map), Some(view_cfg)) = (map, view_cfg) else { return; };

    if map.is_changed() || view_cfg.is_changed() || !grid_geom.is_ready() {
        *grid_geom = make_grid_geometry(
            view_cfg.origin,
            view_cfg.cell_size,
            UVec2::new(map.size.w, map.size.h),
        );
    }
}

/// GridGeometry가 준비되었는지 확인하는 run_if 조건
pub fn grid_geometry_ready(grid_geom: Option<Res<GridGeometry>>) -> bool {
    grid_geom.map_or(false, |g| g.is_ready())
}

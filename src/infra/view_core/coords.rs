use crate::domain::map::components::Map;
use crate::domain::map::grid_index::GridPos;
use crate::modes::battle::features::map_view::view::TileViewConfig;
use bevy::prelude::*;

/// 월드 좌표를 그리드 좌표로. 범위 밖이면 None
#[inline]
pub fn world_to_grid(world: Vec2, view: &TileViewConfig, map: &Map) -> Option<GridPos> {
    let rel = world - view.origin;
    if rel.x < 0.0 || rel.y < 0.0 {
        return None;
    }
    let gx = (rel.x / view.cell_size.x).floor() as i64;
    let gy = (rel.y / view.cell_size.y).floor() as i64;
    if gx < 0 || gy < 0 {
        return None;
    }
    let (gx, gy) = (gx as u32, gy as u32);
    if gx >= map.size.w || gy >= map.size.h {
        return None;
    }
    Some(GridPos { x: gx, y: gy })
}

/// 그리드 중심의 월드 좌표
#[inline]
pub fn grid_to_world_center(g: GridPos, view: &TileViewConfig) -> Vec2 {
    view.origin
        + Vec2::new(
            g.x as f32 * view.cell_size.x + view.cell_size.x * 0.5,
            g.y as f32 * view.cell_size.y + view.cell_size.y * 0.5,
        )
}
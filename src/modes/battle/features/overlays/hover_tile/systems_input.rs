use super::resources::HoverTile;
use crate::domain::map::components::Map;
use crate::infra::view_core::camera::types::MainCamera;
use crate::infra::view_core::coords::GridGeometry;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn update_hover_tile(
    mut hover: ResMut<HoverTile>,
    q_cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_win: Query<&Window, With<PrimaryWindow>>,
    map: Option<Res<Map>>,
    grid_geom: Option<Res<GridGeometry>>,
) {
    let (Some(_map), Some(grid_geom)) = (map, grid_geom) else { hover.grid = None; return; };

    let Ok(win) = q_win.single() else { hover.grid = None; return; };
    let Some(cursor) = win.cursor_position() else { hover.grid = None; return; };

    let Ok((cam, gt)) = q_cam.single() else { hover.grid = None; return; };
    let Ok(world) = cam.viewport_to_world_2d(gt, cursor) else { hover.grid = None; return; };

    hover.grid = grid_geom.world_to_grid(world);
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::infra::view_core::camera::types::MainCamera;
use crate::infra::view_core::coords::world_to_grid;
use crate::domain::map::components::Map;
use crate::modes::battle::features::map_view::view::TileViewConfig;
use super::resources::HoverTile;

pub fn update_hover_tile(
    mut hover: ResMut<HoverTile>,
    q_cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_win: Query<&Window, With<PrimaryWindow>>,
    map: Option<Res<Map>>,
    view: Option<Res<TileViewConfig>>,
) {
    let (Some(map), Some(view)) = (map, view) else { hover.grid = None; return; };

    let Ok(win) = q_win.single() else { hover.grid = None; return; };
    let Some(cursor) = win.cursor_position() else { hover.grid = None; return; };

    let Ok((cam, gt)) = q_cam.single() else { hover.grid = None; return; };
    let Ok(world) = cam.viewport_to_world_2d(gt, cursor) else { hover.grid = None; return; };

    hover.grid = world_to_grid(world, &view, &map);
}

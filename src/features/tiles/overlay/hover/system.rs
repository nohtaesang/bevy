//! Hover overlay system

use bevy::prelude::*;
use crate::features::tiles::tile_to_world_coords;
use crate::features::tiles::utils::world_to_tile_coords;
use crate::features::tiles::TileConfig;
use super::HoverOverlay;

pub fn tile_hover_system(
    mut cursor_moved: EventReader<CursorMoved>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    mut overlay_query: Query<(&mut Transform, &mut Visibility), With<HoverOverlay>>,
) {
    for cursor_event in cursor_moved.read() {
        let screen_pos = cursor_event.position;
        
        if let Ok((camera, camera_transform)) = camera_q.single() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                if let Some(tile_coords) = world_to_tile_coords(world_pos, &tile_config) {
                    let world_pos = tile_to_world_coords(tile_coords.0, tile_coords.1, &tile_config);
                    
                    if let Ok((mut transform, mut visibility)) = overlay_query.single_mut() {
                        transform.translation.x = world_pos.x;
                        transform.translation.y = world_pos.y;
                        *visibility = Visibility::Visible;
                    }
                } else {
                    if let Ok((_, mut visibility)) = overlay_query.single_mut() {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}
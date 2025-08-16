//! Hover tracking system
//!
//! Updates HoverTile resource based on cursor position

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, world_to_tile_coords},
    interaction::resources::HoverTile,
};

/// System that updates HoverTile resource based on cursor position
/// Only updates when tile position changes or cursor leaves grid
pub fn update_hover_tile(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    mut hover_tile: ResMut<HoverTile>,
) {
    // Get cursor position
    let Ok(window) = windows.single() else {
        // Window not found - clear hover if it was set
        if hover_tile.is_hovering() {
            hover_tile.clear();
        }
        return;
    };
    
    let Some(cursor_pos) = window.cursor_position() else {
        // Cursor outside window - clear hover if it was set
        if hover_tile.is_hovering() {
            hover_tile.clear();
        }
        return;
    };
    
    // Convert to world position
    let Ok((camera, camera_transform)) = camera_q.single() else {
        if hover_tile.is_hovering() {
            hover_tile.clear();
        }
        return;
    };
    
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        if hover_tile.is_hovering() {
            hover_tile.clear();
        }
        return;
    };
    
    // Convert to tile coordinates
    if let Some(tile_coords) = world_to_tile_coords(world_pos, &tile_config) {
        let new_tile_pos = IVec2::new(tile_coords.0, tile_coords.1);
        
        // Only update if tile position changed
        if hover_tile.tile_pos != Some(new_tile_pos) {
            hover_tile.set(new_tile_pos);
        }
    } else {
        // Cursor outside grid - clear hover if it was set
        if hover_tile.is_hovering() {
            hover_tile.clear();
        }
    }
}
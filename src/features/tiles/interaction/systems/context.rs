//! Common interaction context utilities
//!
//! This module provides cursor tile calculation and other shared interaction context

use bevy::prelude::*;
use crate::features::tiles::core::{TileConfig, world_to_tile_coords};

/// Helper function to get cursor position in world coordinates
pub fn get_cursor_world_position(
    windows: &Query<&Window>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let cursor_pos = window.cursor_position()?;
    let (camera, camera_transform) = camera_q.single().ok()?;
    camera.viewport_to_world_2d(camera_transform, cursor_pos).ok()
}

/// Helper function to get cursor tile coordinates
pub fn get_cursor_tile_coords(
    windows: &Query<&Window>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
    tile_config: &TileConfig,
) -> Option<(i32, i32)> {
    let world_pos = get_cursor_world_position(windows, camera_q)?;
    world_to_tile_coords(world_pos, tile_config)
}
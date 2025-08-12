use bevy::prelude::*;
use crate::features::tiles::TileConfig;

/// Convert world position to tile coordinates
pub fn world_to_tile_coords(world_pos: Vec2, tile_config: &TileConfig) -> Option<(i32, i32)> {
    let total_size = tile_config.tile_size + tile_config.tile_spacing;
    let offset = -(tile_config.grid_size as f32 * total_size) / 2.0 + total_size / 2.0;
    
    // Calculate tile coordinates
    let tile_x = ((world_pos.x - offset + total_size / 2.0) / total_size).floor() as i32;
    let tile_y = ((world_pos.y - offset + total_size / 2.0) / total_size).floor() as i32;
    
    // Check if coordinates are within grid bounds
    if tile_x >= 0 && tile_x < tile_config.grid_size && tile_y >= 0 && tile_y < tile_config.grid_size {
        Some((tile_x, tile_y))
    } else {
        None
    }
}
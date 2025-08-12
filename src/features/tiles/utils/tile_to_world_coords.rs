use bevy::prelude::*;
use crate::features::tiles::TileConfig;

/// Convert tile coordinates to world position
pub fn tile_to_world_coords(tile_x: i32, tile_y: i32, tile_config: &TileConfig) -> Vec2 {
    let total_size = tile_config.tile_size + tile_config.tile_spacing;
    let offset = -(tile_config.grid_size as f32 * total_size) / 2.0 + total_size / 2.0;
    
    let world_x = offset + tile_x as f32 * total_size;
    let world_y = offset + tile_y as f32 * total_size;
    
    Vec2::new(world_x, world_y)
}
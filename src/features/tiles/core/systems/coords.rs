//! Coordinate conversion systems
//!
//! This module handles conversion between tile coordinates and world positions

use bevy::prelude::*;
use crate::features::tiles::core::resources::TileConfig;

/// Convert tile coordinates to world position
pub fn tile_to_world_coords(tile_x: i32, tile_y: i32, tile_config: &TileConfig) -> Vec2 {
    let total_size = tile_config.tile_size + tile_config.tile_spacing;
    let offset = -(tile_config.grid_size as f32 * total_size) / 2.0 + total_size / 2.0;
    
    let world_x = offset + tile_x as f32 * total_size;
    let world_y = offset + tile_y as f32 * total_size;
    
    Vec2::new(world_x, world_y)
}

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
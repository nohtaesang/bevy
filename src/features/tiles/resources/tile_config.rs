use bevy::prelude::*;

/// Tile configuration resource
#[derive(Resource)]
pub struct TileConfig {
    pub tile_size: f32,
    pub tile_spacing: f32,
    pub grid_size: i32,
}

impl Default for TileConfig {
    fn default() -> Self {
        Self {
            tile_size: 64.0,
            tile_spacing: 2.0,
            grid_size: 30,
        }
    }
}
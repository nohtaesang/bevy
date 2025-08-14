//! Tile visual rendering system
//!
//! Handles spawning and rendering of visual tile sprites

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, tile_to_world_coords, MapRebuilt},
    visual::z,
};

/// Marker component for visual tile sprites
#[derive(Component)]
pub struct VisualTile {
    pub grid_pos: IVec2,
}

/// System to rebuild visual tiles when map is rebuilt
pub fn rebuild_visual_tiles_on_map_event(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    mut map_rebuilt_events: EventReader<MapRebuilt>,
    existing_tiles: Query<Entity, With<VisualTile>>,
) {
    // Process all map rebuilt events
    for event in map_rebuilt_events.read() {
        info!("Map rebuilt event received: {}x{}", event.width, event.height);
        
        // Clean up existing tiles first
        for entity in existing_tiles.iter() {
            commands.entity(entity).despawn();
        }
        
        // Spawn new tiles based on the event dimensions
        for x in 0..event.width {
            for y in 0..event.height {
                let grid_pos = IVec2::new(x, y);
                let world_pos = tile_to_world_coords(x, y, &tile_config);
                
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.3, 0.3, 0.3), // Dark gray tiles
                        custom_size: Some(Vec2::new(tile_config.tile_size, tile_config.tile_size)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, z::TILE), // Background tiles
                    VisualTile { grid_pos },
                    Name::new(format!("Tile ({}, {})", x, y)),
                ));
            }
        }
        
        info!("Spawned {}x{} visual tiles", event.width, event.height);
    }
}
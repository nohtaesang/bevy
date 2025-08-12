//! Movement overlay component and creation functions

use bevy::prelude::*;
use crate::{
    features::tiles::{tile_to_world_coords, TileConfig, TileMap},
};
use super::pathfinding::find_reachable_tiles;

/// Component for movement range overlay tiles
#[derive(Component)]
pub struct MovementOverlay {
    pub tile_pos: IVec2,
}

/// Creates movement overlay sprites using pathfinding to show reachable tiles
/// Returns both the overlay entities and the set of valid positions
pub fn create_movement_overlay_sprites(
    commands: &mut Commands,
    tile_config: &TileConfig,
    tile_map: &TileMap,
    center_pos: IVec2,
    movement_range: i32,
) -> (Vec<Entity>, Vec<IVec2>) {
    let mut overlay_entities = Vec::new();
    
    // Find all reachable tiles using flood fill pathfinding
    let reachable_tiles = find_reachable_tiles(
        center_pos,
        movement_range,
        tile_config,
        tile_map,
    );
    
    // Create overlay sprites for reachable tiles
    for &tile_pos in &reachable_tiles {
        let world_pos_2d = tile_to_world_coords(tile_pos.x, tile_pos.y, tile_config);
        let world_pos = Vec3::new(
            world_pos_2d.x,
            world_pos_2d.y,
            1.0, // Z-index between hover (0) and selected (2)
        );
        
        let entity = commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.3), // Semi-transparent green
                custom_size: Some(Vec2::new(tile_config.tile_size, tile_config.tile_size)),
                ..default()
            },
            Transform::from_translation(world_pos),
            Visibility::Hidden,
            MovementOverlay { tile_pos },
        )).id();
        
        overlay_entities.push(entity);
    }
    
    (overlay_entities, reachable_tiles)
}
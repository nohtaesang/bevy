//! Attack overlay component and creation functions

use bevy::prelude::*;
use crate::{
    features::{
        tiles::{tile_to_world_coords, TileConfig, TileMap},
        units::Unit,
    },
};
use super::range_calculation::find_attackable_tiles;

/// Component for attack range overlay tiles
#[derive(Component)]
pub struct AttackOverlay {
    pub tile_pos: IVec2,
}

/// Creates attack overlay sprites using attack range calculation to show attackable tiles
/// Returns both the overlay entities and the set of valid positions
pub fn create_attack_overlay_sprites(
    commands: &mut Commands,
    tile_config: &TileConfig,
    tile_map: &TileMap,
    attacker_unit: &Unit,
) -> (Vec<Entity>, Vec<IVec2>) {
    let mut overlay_entities = Vec::new();
    
    // Find all attackable tiles based on unit's attack capabilities
    let attackable_tiles = find_attackable_tiles(
        attacker_unit.tile_pos,
        attacker_unit.attack_direction,
        attacker_unit.attack_type,
        &attacker_unit.attack_range,
        tile_config,
        tile_map,
    );
    
    // Create overlay sprites for attackable tiles
    for &tile_pos in &attackable_tiles {
        let world_pos_2d = tile_to_world_coords(tile_pos.x, tile_pos.y, tile_config);
        let world_pos = Vec3::new(
            world_pos_2d.x,
            world_pos_2d.y,
            1.0, // Z-index between hover (0) and selected (2)
        );
        
        let entity = commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.3), // Semi-transparent red
                custom_size: Some(Vec2::new(tile_config.tile_size, tile_config.tile_size)),
                ..default()
            },
            Transform::from_translation(world_pos),
            Visibility::Hidden,
            AttackOverlay { tile_pos },
        )).id();
        
        overlay_entities.push(entity);
    }
    
    (overlay_entities, attackable_tiles)
}
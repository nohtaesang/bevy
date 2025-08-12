//! Spawn system for enemy units

use bevy::prelude::*;
use crate::features::tiles::{tile_to_world_coords, TileConfig, TileMap};
use super::Enemy;

pub fn spawn_enemies(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    mut tile_map: ResMut<TileMap>,
) {
    // Spawn 500 enemies randomly across the map
    let grid_size = tile_config.grid_size;
    let mut enemy_count = 0;
    let target_enemies = 500;
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Generate random positions for enemies
    while enemy_count < target_enemies {
        let x = rng.gen_range(10..grid_size); // Start from x=10 to avoid ally area
        let y = rng.gen_range(0..grid_size);
        let pos = IVec2::new(x, y);
        
        // Check if position is empty
        if tile_map.is_empty(pos) {
            let world_pos = tile_to_world_coords(pos.x, pos.y, &tile_config);
            
            let enemy = commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.2, 0.2), // Red for enemies
                    custom_size: Some(Vec2::new(tile_config.tile_size * 0.6, tile_config.tile_size * 0.6)),
                    ..default()
                },
                Transform::from_xyz(world_pos.x, world_pos.y, 1.0), // Z=1 to be above tiles
                Enemy::new(pos),
                Name::new(format!("Enemy at ({}, {})", pos.x, pos.y)),
            )).id();
            tile_map.place_enemy(pos, enemy);
            enemy_count += 1;
        }
    }
    
}
//! Systems for spawning units and enemies

use bevy::prelude::*;
use crate::{
    features::tiles::tile_to_world_coords,
    resources::TileConfig,
};
use super::components::{Unit, Enemy};

pub fn spawn_units(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
) {
    // Spawn a few units at specific positions
    let unit_positions = vec![
        IVec2::new(2, 2),
        IVec2::new(3, 2),
        IVec2::new(2, 3),
    ];
    
    for pos in unit_positions {
        let world_pos = tile_to_world_coords(pos.x, pos.y, &tile_config);
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.6, 1.0), // Blue for units
                custom_size: Some(Vec2::new(tile_config.tile_size * 0.6, tile_config.tile_size * 0.6)),
                ..default()
            },
            Transform::from_xyz(world_pos.x, world_pos.y, 1.0), // Z=1 to be above tiles
            Unit::new(pos),
            Name::new(format!("Unit at ({}, {})", pos.x, pos.y)),
        ));
    }
    
    println!("Spawned {} units", 3);
}

pub fn spawn_enemies(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
) {
    // Spawn a few enemies at specific positions
    let enemy_positions = vec![
        IVec2::new(7, 7),
        IVec2::new(8, 7),
        IVec2::new(7, 8),
    ];
    
    for pos in enemy_positions {
        let world_pos = tile_to_world_coords(pos.x, pos.y, &tile_config);
        
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.2, 0.2), // Red for enemies
                custom_size: Some(Vec2::new(tile_config.tile_size * 0.6, tile_config.tile_size * 0.6)),
                ..default()
            },
            Transform::from_xyz(world_pos.x, world_pos.y, 1.0), // Z=1 to be above tiles
            Enemy::new(pos),
            Name::new(format!("Enemy at ({}, {})", pos.x, pos.y)),
        ));
    }
    
    println!("Spawned {} enemies", 3);
}
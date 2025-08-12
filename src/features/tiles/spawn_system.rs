use bevy::prelude::*;
use crate::features::tiles::TileConfig;
use super::Tile;

/// System to spawn the tile grid when entering InGame state
pub fn spawn_tiles(mut commands: Commands, tile_config: Res<TileConfig>) {
    println!("Spawning {}x{} tile grid...", tile_config.grid_size, tile_config.grid_size);
    
    let total_size = tile_config.tile_size + tile_config.tile_spacing;
    let offset = -(tile_config.grid_size as f32 * total_size) / 2.0 + total_size / 2.0;
    
    for x in 0..tile_config.grid_size {
        for y in 0..tile_config.grid_size {
            let world_x = offset + x as f32 * total_size;
            let world_y = offset + y as f32 * total_size;
            
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.7, 0.7, 0.7),
                    custom_size: Some(Vec2::splat(tile_config.tile_size)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
                Tile { x, y },
                Name::new(format!("Tile_{}_{}", x, y)),
            ));
        }
    }
    
    println!("Spawned {} tiles", tile_config.grid_size * tile_config.grid_size);
}